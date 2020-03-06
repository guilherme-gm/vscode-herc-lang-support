use super::position;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

use super::super::script_formatter::*;

fn format_item(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.write_edit(fmter.get_node_text(node));
}

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("shop_def: {:?}", node));

    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until(&mut cursor, FmtNode::Named("position"), true);
    position::format(fmter, &cursor.node());
    cursor.goto_next_sibling();

    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("type"), true);
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("name"), true);
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("sprite"), true);

    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token(","), true);
    while fmter.match_until(&mut cursor, FmtNode::Named("item"), false) {
        format_item(fmter, &cursor.node());
        cursor.goto_next_sibling();

        if !fmter.match_until_and_write_node(&mut cursor, FmtNode::Token(","), true) {
            break;
        }
    }

    fmter.write_edit(String::from("\n"));
}
