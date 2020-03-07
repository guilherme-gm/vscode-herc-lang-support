use super::position;
use super::trigger_area;
use tree_sitter::Node;

use super::super::script_formatter::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("duplicate_def: {:?}", node));

    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until(&mut cursor, FmtNode::Named("position"), true);
    position::format(fmter, &cursor.node());
    cursor.goto_next_sibling();

    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("duplicate"), &"duplicate", Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("("), &"(", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("src_npc"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(")"), &")", Spacing::None, true);
    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("new_npc"), Spacing::None, true);
    fmter.write_edit(String::from("\t"), Spacing::None);

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Named("span_x"), FmtNode::Named("sprite")],
        true,
    );
    if fmter.is_stop(&mut cursor, &FmtNode::Named("span_x")) {
        fmter.write_node(&mut cursor, Spacing::None);
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
        fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("span_y"), Spacing::None, true);
    } else {
        // Write sprite and checks if there is more (that is, trigger)
        if fmter.write_node(&mut cursor, Spacing::None) {
            if fmter.is_stop(&mut cursor, &FmtNode::Named("trigger")) {
                trigger_area::format(fmter, &cursor.node());
                cursor.goto_next_sibling();
            }
        }
    }

    fmter.write_edit(String::from("\n"), Spacing::None);
}
