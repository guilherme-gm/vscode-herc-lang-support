use super::super::expressions;
use super::super::script_formatter::*;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> return_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token("return"), Spacing::Indent, true);

    fmter.match_until_one(&mut cursor, &[FmtNode::Token(";"), FmtNode::Named("value")], true);
    if fmter.is_stop(&mut cursor, &FmtNode::Named("value")) {
        expressions::resolve(fmter, &cursor.node());
        cursor.goto_next_sibling();
    }

    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token(";"), Spacing::None, true);
    fmter.write_newline();
}
