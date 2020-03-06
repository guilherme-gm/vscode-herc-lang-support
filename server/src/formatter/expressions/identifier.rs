use super::super::script_formatter::*;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> identifier_expr: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.write_node(&mut cursor);
}
