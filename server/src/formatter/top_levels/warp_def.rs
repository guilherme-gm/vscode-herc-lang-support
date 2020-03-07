use super::position;
use tree_sitter::Node;

use super::super::script_formatter::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("warp_def: {:?}", node));

    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until(&mut cursor, FmtNode::Named("position"), true);
    position::format(fmter, &cursor.node());
    cursor.goto_next_sibling();
    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token("warp"), Spacing::None, true);
    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("name"), Spacing::None, true);
    fmter.write_edit(String::from("\t"), Spacing::None);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("span_x"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("span_y"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("to_map"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("to_x"), Spacing::None, true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", Spacing::None, true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("to_y"), Spacing::None, true);

    fmter.write_edit(String::from("\n"), Spacing::None);
}
