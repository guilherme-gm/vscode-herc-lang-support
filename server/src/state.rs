use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tree_sitter::Parser;
use tower_lsp::lsp_types::Url;
use crate::source_file::SourceFile;
use crate::script_commands::ScriptCommand;

pub struct State {
    pub sources: HashMap<Url,  Arc<Mutex<SourceFile>>>,
    pub parser: Parser,
    pub commands: Option<HashMap<String, ScriptCommand>>,
}
