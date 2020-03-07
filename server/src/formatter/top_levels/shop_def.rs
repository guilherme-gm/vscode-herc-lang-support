use super::position;
use tree_sitter::Node;

use super::super::script_formatter::*;

fn format_item(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.write_edit(fmter.get_node_text(node), Spacing::None);
}

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("shop_def: {:?}", node));

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

    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token(","), Spacing::None, true);
    while fmter.match_until(&mut cursor, FmtNode::Named("item"), false) {
        format_item(fmter, &cursor.node());

        if !cursor.goto_next_sibling()
            || !fmter.match_until_and_write_node(
                &mut cursor,
                FmtNode::Token(","),
                Spacing::None,
                false,
            )
        {
            break;
        }
    }

    fmter.write_edit(String::from("\n"), Spacing::None);
}
