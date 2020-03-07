use super::super::expressions;
use super::super::script_formatter::*;
use tree_sitter::Node;

fn format_args(fmter: &mut ScriptFormatter, node: &Node) {
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token("("), Spacing::None, true);

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Token(")"), FmtNode::Named("param")],
        true,
    );

    if fmter.is_stop(&mut cursor, &FmtNode::Named("param")) {
        expressions::resolve(fmter, &cursor.node());

        if cursor.goto_next_sibling() {
            while fmter.match_until_one(
                &mut cursor,
                &[FmtNode::Token(")"), FmtNode::Token(",")],
                true,
            ) {
                if fmter.is_stop(&mut cursor, &FmtNode::Token(",")) {
                    fmter.match_until_and_write_str(
                        &mut cursor,
                        FmtNode::Token(","),
                        ", ",
                        Spacing::None,
                        false,
                    );

                    fmter.match_until_one(
                        &mut cursor,
                        &[FmtNode::Named("param"), FmtNode::Token(")")],
                        true,
                    );

                    if fmter.is_stop(&mut cursor, &FmtNode::Named("param")) {
                        expressions::resolve(fmter, &cursor.node());
                        cursor.goto_next_sibling();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token(")"), Spacing::None, true);
}

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> function_exp: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until(&mut cursor, FmtNode::Named("function"), true);
    expressions::resolve(fmter, &cursor.node());
    fmter
        .function_stack
        .push(fmter.get_node_text(&cursor.node()));
    cursor.goto_next_sibling();

    fmter.match_until(&mut cursor, FmtNode::Named("arguments"), true);
    format_args(fmter, &cursor.node());
}
