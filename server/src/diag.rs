use tree_sitter::Tree;
use lsp_types::{Diagnostic, Range, Position, DiagnosticSeverity, NumberOrString};
use std::vec::Vec;
use std::convert::TryInto;
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};

pub fn get_diagnostics(tree: Arc<Mutex<Tree>>) -> Vec<Diagnostic> {
    let mut ret: Vec<Diagnostic> = Vec::new();

    let tree = tree.lock().unwrap();
    for child_id in 0..tree.root_node().child_count() {
        let child = tree.root_node().child(child_id).unwrap();
        if child.kind().eq_ignore_ascii_case("error") {
            let range = Range::new(
                Position::new(child.start_position().row.try_into().unwrap(), child.start_position().column.try_into().unwrap()),
                Position::new(child.end_position().row.try_into().unwrap(), child.end_position().column.try_into().unwrap())
            );
            let diag = Diagnostic::new(range, Some(DiagnosticSeverity::Error), Some(NumberOrString::Number(1)), Some(String::from("Hercules")), String::from("Err"), None, None);
            ret.push(diag);
        }
    }

    ret
}
