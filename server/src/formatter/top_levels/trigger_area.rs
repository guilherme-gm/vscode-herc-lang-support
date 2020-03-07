use super::super::script_formatter::*;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    let mut cursor = node.walk();
    cursor.goto_first_child();
    
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("span_x"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("span_y"), Spacing::None, true);
}
