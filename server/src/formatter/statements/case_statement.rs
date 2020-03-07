use super::super::script_formatter::*;
use super::super::statements;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> case_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Token("case"), FmtNode::Token("default")],
        true,
    );
    if fmter.is_stop(&mut cursor, &FmtNode::Token("case")) {
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("case"), &"case ", Spacing::Indent, true);
        fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("value"), Spacing::None, true);
    } else {
        fmter.write_node(&mut cursor, Spacing::Indent); // default
    }

    if fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(":"), &":\n", Spacing::None, true) {
        if fmter.match_until(&mut cursor, FmtNode::Named("body"), false) {
            fmter.set_indent(fmter.indent_level + 1);
            statements::resolve(fmter, &cursor.node());
            cursor.goto_next_sibling();

            while fmter.match_until(&mut cursor, FmtNode::Named("body"), false) {
                statements::resolve(fmter, &cursor.node());
                if !cursor.goto_next_sibling() {
                    break;
                }
            }

            fmter.set_indent(fmter.indent_level - 1);
        }
    }
}
