use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tree_sitter::{Parser, Tree};
use tower_lsp::lsp_types::Url;

pub struct State {
    pub sources: HashMap<Url,  Arc<Mutex<Tree>>>,
    pub parser: Parser,
}