use super::super::expressions::parenthesized_expression;
use super::super::script_formatter::*;
use super::super::statements;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> do_while_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_and_write_str(
        &mut cursor,
        FmtNode::Token("do"),
        &format!("{}do ", fmter.indent),
        true,
    );

    fmter.set_indent(fmter.indent_level + 1);
    fmter.match_until(&mut cursor, FmtNode::Named("body"), true);
    statements::resolve(fmter, &cursor.node());
    fmter.set_indent(fmter.indent_level - 1);
    fmter.match_until_and_write_str(
        &mut cursor,
        FmtNode::Token("while"),
        &format!("{}while ", fmter.indent),
        true,
    );

    parenthesized_expression::format(fmter, &cursor.node());
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(";"), ";", true);
}
