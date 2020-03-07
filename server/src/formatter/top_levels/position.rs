use super::super::script_formatter::*;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    let mut cursor = node.walk();
    cursor.goto_first_child();
    
    fmter.match_until_one(&mut cursor, &[FmtNode::Token("-"), FmtNode::Named("map")], true);
    if cursor.field_name().is_some() { // it is "map"
        fmter.write_node(&mut cursor, Spacing::None);
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
        fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("x"), Spacing::None, true);
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
        if fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("y"), Spacing::None, true) {
            fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, false);
            fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("dir"), Spacing::None, true);
        }
    } else { // it is "-"
        fmter.write_edit(String::from("-"), Spacing::None);
    }
}
