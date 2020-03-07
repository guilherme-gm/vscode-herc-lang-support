use super::super::expressions::parenthesized_expression;
use super::super::script_formatter::*;
use super::super::statements;
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

    let is_block = cursor.node().kind().eq_ignore_ascii_case("block");
    if is_block {
        fmter.write_space();
    } else {
        fmter.set_indent(fmter.indent_level + 1);
        fmter.write_newline();
    }
    statements::resolve(fmter, &cursor.node());
    cursor.goto_next_sibling();

    if !is_block {
        fmter.set_indent(fmter.indent_level - 1);
    }
}
