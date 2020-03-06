use super::super::expressions;
use super::super::script_formatter::*;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> unary_expr: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();
    
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("operator"), true);
    
    fmter.match_until(&mut cursor, FmtNode::Named("argument"), true);
    expressions::resolve(fmter, &cursor.node());
    cursor.goto_next_sibling();
}
