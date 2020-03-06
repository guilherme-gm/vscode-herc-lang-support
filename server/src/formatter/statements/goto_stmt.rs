use super::super::expressions;
use super::super::script_formatter::*;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> goto_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.write_edit(format!("{}", fmter.indent));
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("goto"), &"goto ", true);
    fmter.match_until(&mut cursor, FmtNode::Named("label"), true);
    expressions::identifier::format(fmter, &cursor.node());
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(";"), &";\n", true);
}
