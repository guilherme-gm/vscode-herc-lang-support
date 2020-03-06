use super::super::script_formatter::*;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    let mut cursor = node.walk();
    cursor.goto_first_child();
    
    fmter.match_until_one(&mut cursor, &[FmtNode::Token("-"), FmtNode::Named("map")], true);
    if cursor.field_name().is_some() { // it is "map"
        fmter.write_node(&mut cursor);
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
        fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("x"), true);
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
        fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("y"), true);
        
        if fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", false) {
            fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("dir"), true);
        }
    } else { // it is "-"
        fmter.write_edit(String::from("-"));
    }
}
