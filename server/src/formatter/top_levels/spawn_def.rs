use tree_sitter::Node;

use super::super::script_formatter::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("spawn_def: {:?}", node));

    let mut cursor = node.walk();
    cursor.goto_first_child();
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("map"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("x1"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("y1"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("x2"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("y2"), Spacing::None, true);
    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("type"), Spacing::None, true);
    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("name"), Spacing::None, true);
    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("id"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("amount"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("delay1"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("delay2"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    let has_more = fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("event"), Spacing::None, true);

    if has_more {
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
        fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("size"), Spacing::None, true);
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
        fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("ai"), Spacing::None, true);
    }

    fmter.write_edit(String::from("\n"), Spacing::None);
}
