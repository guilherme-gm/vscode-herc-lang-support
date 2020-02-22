#[macro_use]
macro_rules! debug_ {
    ($con: expr, $msg: expr) => {
        $con.write(format!("{}\n", $msg).as_bytes()).unwrap();
    };
}

mod completion;
mod diag;
mod file;
mod source_file;
mod state;

use std::collections::HashMap;
use std::sync::{Mutex};
use futures::future;
use jsonrpc_core::{BoxFuture, Result};
use serde_json::Value;
use tower_lsp::lsp_types::request::GotoDefinitionResponse;
use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageServer, LspService, Printer, Server};
use tree_sitter::{Parser, Language};
use state::State;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

extern "C" { fn tree_sitter_hercscript() -> Language; }


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
            }),
            con: Mutex::new(stream),
        }
    }
}

impl LanguageServer for HerculesScript {
    type ShutdownFuture = BoxFuture<()>;
    type SymbolFuture = BoxFuture<Option<Vec<SymbolInformation>>>;
    type ExecuteFuture = BoxFuture<Option<Value>>;
    type CompletionFuture = BoxFuture<Option<CompletionResponse>>;
    type HoverFuture = BoxFuture<Option<Hover>>;
    type DeclarationFuture = BoxFuture<Option<GotoDefinitionResponse>>;
    type DefinitionFuture = BoxFuture<Option<GotoDefinitionResponse>>;
    type TypeDefinitionFuture = BoxFuture<Option<GotoDefinitionResponse>>;
    type HighlightFuture = BoxFuture<Option<Vec<DocumentHighlight>>>;

    fn initialize(&self, _: &Printer, _: InitializeParams) -> Result<InitializeResult> {
        // printer.log_message(MessageType::Info, format!("{:?}", p.initialization_options.unwrap()));
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Incremental,
                )),
                hover_provider: Some(true),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                }),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: None,
                    retrigger_characters: None,
                    work_done_progress_options: Default::default(),
                }),
                document_highlight_provider: Some(true),
                workspace_symbol_provider: Some(true),
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

    fn shutdown(&self) -> Self::ShutdownFuture {
        Box::new(future::ok(()))
    }

    fn symbol(&self, _: WorkspaceSymbolParams) -> Self::SymbolFuture {
        Box::new(future::ok(None))
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

    fn execute_command(&self, printer: &Printer, _: ExecuteCommandParams) -> Self::ExecuteFuture {
        printer.log_message(MessageType::Info, "command executed!");
        printer.apply_edit(WorkspaceEdit::default());
        Box::new(future::ok(None))
    }

    fn did_open(&self, printer: &Printer, params: DidOpenTextDocumentParams) {
        printer.log_message(MessageType::Info, format!("file '{}' opened!", params.text_document.uri));
        // We are only locking it here, so it is safe to unwrap_or_else (I think)
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let uri = params.text_document.uri.clone();
        let src = file::open(&mut state, params.text_document);
        let cp = src.clone(); //@removeme
        let diags = diag::get_diagnostics(src);
        printer.log_message(MessageType::Info, format!("{:?}", cp.lock().unwrap().line_bytes)); //@removeme
        
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
            printer.log_message(MessageType::Error, format!("File {} was not open. Failed to update and diagnose it.", params.text_document.uri));
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

    fn completion(&self, params: CompletionParams) -> Self::CompletionFuture {
        // params.text_document_position.
        let uri = &params.text_document_position.text_document.uri;
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());

        if let Some(src) = file::get(&mut state, uri) {
            let position = params.text_document_position.position;
            Box::new(future::ok(Some(CompletionResponse::Array(completion::get_completion(&self.con, src, position)))))
        } else {
            Box::new(future::ok(None))
        }
    }

    fn hover(&self, _: TextDocumentPositionParams) -> Self::HoverFuture {
        Box::new(future::ok(None))
    }

    fn goto_declaration(&self, _: TextDocumentPositionParams) -> Self::DeclarationFuture {
        Box::new(future::ok(None))
    }

    fn goto_definition(&self, _: TextDocumentPositionParams) -> Self::DefinitionFuture {
        Box::new(future::ok(None))
    }

    fn goto_type_definition(&self, _: TextDocumentPositionParams) -> Self::TypeDefinitionFuture {
        Box::new(future::ok(None))
    }

    fn document_highlight(&self, _: TextDocumentPositionParams) -> Self::HighlightFuture {
        Box::new(future::ok(None))
    }
}

fn start_language_server() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(HerculesScript::new());
    let handle = service.close_handle();
    let server = Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service);

    tokio::run(handle.run_until_exit(server));
}

// fn start_tree() {
//     let mut parser = Parser::new();
//     let language = unsafe { tree_sitter_hercscript() };
//     parser.set_language(language).unwrap();

//     let source_code = "-\tscript\tTest\t\n\n-\tscript\tTest\tFAKE_NPC,{}";
//     let tree = parser.parse(source_code, None).unwrap();
//     let root_node = tree.root_node();

//     println!("{:?}", root_node.child(1));//, "source_file");
//     println!("{}", root_node.start_position().column);//, 0);
//     println!("{}", root_node.end_position().column); // 12);
// }

fn main() {
    start_language_server();
}
