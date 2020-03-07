use super::super::statements;
use super::position;
use super::trigger_area;
use tree_sitter::Node;

use super::super::script_formatter::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("script_def: {:?}", node));

    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until(&mut cursor, FmtNode::Named("position"), true);
    position::format(fmter, &cursor.node());
    cursor.goto_next_sibling();

    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("type"), Spacing::None, true);
    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("name"), Spacing::None, true);
    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("sprite"), Spacing::None, true);

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Named("trigger_area"), FmtNode::Token(",")],
        true,
    );

    if cursor.node().is_named() {
        // This is a trigger area
        trigger_area::format(fmter, &cursor.node());
        cursor.goto_next_sibling();
    }

    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &",", Spacing::None, true);
    if fmter.match_until(&mut cursor, FmtNode::Named("body"), true) {
        statements::block::format(fmter, &cursor.node());
    } else {
        panic!("script_def:: Could not move to block.");
    }
}
