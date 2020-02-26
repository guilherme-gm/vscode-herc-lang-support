#[macro_use]
macro_rules! debug_ {
    ($con: expr, $msg: expr) => {
        $con.write(format!("{}\n", $msg).as_bytes()).unwrap();
    };
}

mod completion;
mod diag;
mod file;
mod script_commands;
mod signature;
mod source_file;
mod state;

use jsonrpc_core::Result;
use script_commands::ScriptCommand;
use serde_json::Value;
use state::State;
use std::collections::HashMap;
use std::fs::File;
use std::sync::Mutex;
use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageServer, LspService, Printer, Server};
use tree_sitter::{Language, Parser};

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

extern "C" {
    fn tree_sitter_hercscript() -> Language;
}

pub struct HerculesScript {
    state: Mutex<State>,
    con: Mutex<TcpStream>, // Debugger
}

impl HerculesScript {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        let language = unsafe { tree_sitter_hercscript() };
        parser.set_language(language).unwrap();
        let mut stream = TcpStream::connect("127.0.0.1:10000").unwrap();
        let _ = stream.set_nodelay(true);
        // let _ = stream.write("Client Connected\n".as_bytes());
        debug_!(stream, "Client Connected");

        HerculesScript {
            state: Mutex::new(State {
                sources: HashMap::new(),
                parser,
                commands: None,
            }),
            con: Mutex::new(stream),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for HerculesScript {
    fn initialize(&self, _: &Printer, params: InitializeParams) -> Result<InitializeResult> {
        let init = params.initialization_options.unwrap();
        if let Value::String(cmd_path) = init.get("script_cmd").unwrap() {
            let commands_json = File::open(cmd_path).unwrap();
            let mut commands: HashMap<String, ScriptCommand> =
                serde_json::from_reader(commands_json).unwrap();
            script_commands::load_prototypes(&mut commands);
            self.state.lock().unwrap().commands = Some(commands);
        }
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Incremental,
                )),
                hover_provider: Some(false),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                }),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                    retrigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                    work_done_progress_options: Default::default(),
                }),
                document_highlight_provider: Some(false),
                workspace_symbol_provider: Some(false),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_string()],
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(WorkspaceCapability {
                    workspace_folders: Some(WorkspaceFolderCapability {
                        supported: Some(true),
                        change_notifications: Some(
                            WorkspaceFolderCapabilityChangeNotifications::Bool(true),
                        ),
                    }),
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    fn initialized(&self, printer: &Printer, _: InitializedParams) {
        printer.log_message(MessageType::Info, "server initialized!");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    fn did_change_workspace_folders(&self, printer: &Printer, _: DidChangeWorkspaceFoldersParams) {
        printer.log_message(MessageType::Info, "workspace folders changed!");
    }

    fn did_change_configuration(&self, printer: &Printer, _: DidChangeConfigurationParams) {
        printer.log_message(MessageType::Info, "configuration changed!");
    }

    fn did_change_watched_files(&self, printer: &Printer, _: DidChangeWatchedFilesParams) {
        printer.log_message(MessageType::Info, "watched files have changed!");
    }

    async fn execute_command(
        &self,
        printer: &Printer,
        _: ExecuteCommandParams,
    ) -> Result<Option<Value>> {
        printer.log_message(MessageType::Info, "command executed!");
        printer.apply_edit(WorkspaceEdit::default());
        Ok(None)
    }

    fn did_open(&self, printer: &Printer, params: DidOpenTextDocumentParams) {
        printer.log_message(
            MessageType::Info,
            format!("file '{}' opened!", params.text_document.uri),
        );
        // We are only locking it here, so it is safe to unwrap_or_else (I think)
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let uri = params.text_document.uri.clone();
        let src = file::open(&mut state, params.text_document);
        let cp = src.clone(); //@removeme
        let diags = diag::get_diagnostics(src);
        printer.log_message(
            MessageType::Info,
            format!("{:?}", cp.lock().unwrap().line_bytes),
        ); //@removeme
        printer.publish_diagnostics(uri, diags, None);
    }

    fn did_change(&self, printer: &Printer, params: DidChangeTextDocumentParams) {
        printer.log_message(MessageType::Info, "file changed!");
        // We are only locking it here, so it is safe to unwrap_or_else (I think)
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(src) = file::get(&mut state, &params.text_document.uri) {
            file::update(&mut state, src.clone(), params.content_changes);
            let diags = diag::get_diagnostics(src);

            printer.publish_diagnostics(params.text_document.uri, diags, None);
        } else {
            printer.log_message(
                MessageType::Error,
                format!(
                    "File {} was not open. Failed to update and diagnose it.",
                    params.text_document.uri
                ),
            );
        }
    }

    fn did_save(&self, _printer: &Printer, _: DidSaveTextDocumentParams) {
        // Should we do something here?
    }

    fn did_close(&self, _printer: &Printer, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        // We are only locking it here, so it is safe to unwrap_or_else (I think)
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(_src) = file::get(&mut state, &uri) {
            file::close(&mut state, &uri);
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        // params.text_document_position.
        let uri = &params.text_document_position.text_document.uri;
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());

        if let Some(src) = file::get(&mut state, uri) {
            let position = params.text_document_position.position;
            Ok(Some(CompletionResponse::Array(completion::get_completion(
                &self.con, &state, src, position,
            ))))
        } else {
            Ok(None)
        }
    }

    async fn signature_help(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<SignatureHelp>> {
        // debug_!(&mut self.con.lock().unwrap(), format!("Sign: {:?}", params));
        // params.text_document_position.
        let uri = &params.text_document.uri;
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());

        if let Some(src) = file::get(&mut state, uri) {
            let position = params.position;

            Ok(signature::get_signature(&self.con, &state, src, position))
        } else {
            Ok(None)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(HerculesScript::new());
    let handle = service.close_handle();
    let server = Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service);

    handle.run_until_exit(server).await;
    Ok(())
}
