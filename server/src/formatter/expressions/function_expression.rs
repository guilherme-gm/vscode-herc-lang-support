use super::super::expressions;
use super::super::script_formatter::*;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

fn format_args(fmter: &mut ScriptFormatter, node: &Node) {
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token("("), true);

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Token(")"), FmtNode::Named("param")],
        true,
    );

    if fmter.is_stop(&mut cursor, &FmtNode::Named("param")) {
        // TODO: Reformat parameters
        if fmter.write_node(&mut cursor) {
            while fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), ", ", false) {
                if !fmter.match_until_and_write_node(&mut cursor, FmtNode::Token("param"), false) {
                    // TODO: FIXME: Ensure everything after that is valid..
                    // This will be false if you are in an incomplete (but acceptable) function.
                    // I.E: fun(a, b,)
                    break;
                }
            }
        }
    }

    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token(")"), true);
}

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> function_exp: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until(&mut cursor, FmtNode::Named("function"), true);
    expressions::resolve(fmter, &cursor.node());
    cursor.goto_next_sibling();

    fmter.match_until(&mut cursor, FmtNode::Named("arguments"), true);
    format_args(fmter, &cursor.node());
}
