use super::super::expressions::parenthesized_expression;
use super::super::script_formatter::*;
use super::block;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> switch_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("switch"), "switch ", Spacing::Indent, true);

    fmter.match_until(&mut cursor, FmtNode::Named("condition"), true);
    parenthesized_expression::format(fmter, &cursor.node());
    cursor.goto_next_sibling();
    fmter.write_space();

    fmter.match_until(&mut cursor, FmtNode::Named("body"), true);
    block::format_sub(fmter, &cursor.node(), true, false);
    cursor.goto_next_sibling();
}
