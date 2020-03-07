use super::super::expressions;
use super::super::script_formatter::*;
use super::super::statements;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> for_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_and_write_str(
        &mut cursor,
        FmtNode::Token("for"),
        "for ",
        Spacing::Indent,
        true,
    );

    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("("), "(", Spacing::None, true);

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Named("initializer"), FmtNode::Token(";")],
        true,
    );
    if fmter.is_stop(&mut cursor, &FmtNode::Named("initializer")) {
        expressions::resolve(fmter, &cursor.node());
        cursor.goto_next_sibling();
    }
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(";"), ";", Spacing::None, true);

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Named("condition"), FmtNode::Token(";")],
        true,
    );
    if fmter.is_stop(&mut cursor, &FmtNode::Named("condition")) {
        fmter.write_space();
        expressions::resolve(fmter, &cursor.node());
        cursor.goto_next_sibling();
    }
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(";"), ";", Spacing::None, true);

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Named("update"), FmtNode::Token(")")],
        true,
    );
    if fmter.is_stop(&mut cursor, &FmtNode::Named("update")) {
        fmter.write_space();
        expressions::resolve(fmter, &cursor.node());
        cursor.goto_next_sibling();
    }

    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(")"), ")", Spacing::None, true);
    fmter.match_until(&mut cursor, FmtNode::Named("body"), true);

    let is_block = cursor.node().kind().eq_ignore_ascii_case("block");

    if !is_block {
        fmter.set_indent(fmter.indent_level + 1);
        fmter.write_newline();
    } else {
        fmter.write_space();
    }
    statements::resolve(fmter, &cursor.node());
    cursor.goto_next_sibling();

    if !is_block {
        fmter.set_indent(fmter.indent_level - 1);
    }
}
