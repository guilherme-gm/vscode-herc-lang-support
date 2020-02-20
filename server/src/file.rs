use lsp_types::*;
use std::sync::{Arc, Mutex};

use crate::source_file::SourceFile;
use crate::State;

pub fn open(state: &mut State, document: TextDocumentItem) -> Arc<Mutex<SourceFile>> {
    let uri = document.uri;

    if state.sources.contains_key(&uri) {
        state.sources.remove(&uri);
    }

    let src = Arc::new(Mutex::new(SourceFile::new(document.text, &mut state.parser)));
    state.sources.insert(uri.clone(), src.clone());
    src.clone()
}

pub fn get(state: &mut State, url: &Url) -> Option<Arc<Mutex<SourceFile>>> {
    if !state.sources.contains_key(url) {
        return None;
    }

    Some(state.sources.get(url).expect("Failed to get file").clone())
}

pub fn update(state: &mut State, source: Arc<Mutex<SourceFile>>, changes: Vec<TextDocumentContentChangeEvent>) {
    let mut source = source.lock().unwrap();
    source.update(&mut state.parser, changes);
}

pub fn close(state: &mut State, url: &Url) {
    state.sources.remove(&url);
}
