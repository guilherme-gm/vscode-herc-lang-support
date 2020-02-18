use lsp_types::*;
use std::sync::{Arc, Mutex};
use tree_sitter::{Tree, InputEdit};
use tower_lsp::Printer;

use crate::State;

pub fn open(state: &mut State, document: TextDocumentItem) -> Arc<Mutex<Tree>> {
    let uri = document.uri;

    if state.sources.contains_key(&uri) {
        state.sources.remove(&uri);
    }

    let tree = Arc::new(Mutex::new(state.parser.parse(document.text, None).unwrap()));
    
    state.sources.insert(uri.clone(), tree.clone());
    tree.clone()
}

pub fn get(state: &mut State, url: &Url) -> Option<Arc<Mutex<Tree>>> {
    if !state.sources.contains_key(url) {
        return None;
    }

    Some(state.sources.get(url).expect("Failed to get file").clone())
}

pub fn update( printer: &Printer, tree: Arc<Mutex<Tree>>, changes: Vec<TextDocumentContentChangeEvent>) {
    let tree = tree.lock().unwrap();

    for change in changes {
        printer.log_message(MessageType::Error, format!("Change: \n{:?}\n", change));
        // println!("{:?}", change);
        // if let Some(range) = change.range {
            // let edit = InputEdit {
            //     start_byte: range.start,
            //     old_end_byte: 8,
            //     new_end_byte: 14,
            //     start_position: Point::new(0, 8),
            //     old_end_position: Point::new(0, 8),
            //     new_end_position: Point::new(0, 14),
            // };

            // tree.edit()
        // }
    }

}