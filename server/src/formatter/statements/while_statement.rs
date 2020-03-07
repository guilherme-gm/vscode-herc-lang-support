use super::super::expressions::parenthesized_expression;
use super::super::script_formatter::*;
use super::block;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> while_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("while"), "while ", Spacing::Indent, true);

    fmter.match_until(&mut cursor, FmtNode::Named("condition"), true);
    parenthesized_expression::format(fmter, &cursor.node());
    cursor.goto_next_sibling();

    fmter.match_until(&mut cursor, FmtNode::Named("body"), true);
    block::format(fmter, &cursor.node());
    cursor.goto_next_sibling();
}
