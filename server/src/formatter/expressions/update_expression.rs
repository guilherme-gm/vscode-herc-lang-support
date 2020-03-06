use super::super::expressions;
use super::super::script_formatter::*;
use tree_sitter::{Node, TreeCursor};

// Debugger
use std::io::prelude::*;

fn resolve_operator_argument(fmter: &mut ScriptFormatter, cursor: &mut TreeCursor) {
    if fmter.is_stop(cursor, &FmtNode::Named("argument")) {
        expressions::resolve(fmter, &cursor.node());
        cursor.goto_next_sibling();
    } else {
        // operator
        fmter.write_node(cursor);
    }
}

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> update_expr: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Named("argument"), FmtNode::Named("operator")],
        true,
    );
    resolve_operator_argument(fmter, &mut cursor);

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Named("argument"), FmtNode::Named("operator")],
        true,
    );
    resolve_operator_argument(fmter, &mut cursor);
}
