use super::super::expressions;
use super::super::script_formatter::*;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> expression_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Named("expr"), FmtNode::Token(";")],
        true,
    );

    if fmter.is_stop(&mut cursor, &FmtNode::Named("expr")) {
        let node = &cursor.node();

        // It may be a script command in the old style that doesn't have parameters (e.g.: "close;")
        if fmter.is_command(&node) {
            fmter.write_edit(format!("{}()", fmter.get_node_text(&node)), Spacing::Indent);
        } else {
            fmter.write_indent();
            expressions::resolve(fmter, &cursor.node());
        }

        cursor.goto_next_sibling();
    }

    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(";"), ";\n", Spacing::None, true);
}
