use super::super::script_formatter::*;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> continue_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();
    
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("continue"), "continue", Spacing::Indent, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token(";"), Spacing::None, true);
    fmter.write_newline();
}
