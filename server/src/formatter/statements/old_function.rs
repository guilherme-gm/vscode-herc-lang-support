use super::super::expressions;
use super::super::script_formatter::*;
use tree_sitter::Node;

fn format_args(fmter: &mut ScriptFormatter, node: &Node) {
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Token(";"), FmtNode::Named("param")],
        true,
    );

    if fmter.is_stop(&mut cursor, &FmtNode::Named("param")) {
        // TODO: Reformat parameters
        if fmter.write_node(&mut cursor, Spacing::None) {
            while fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), ", ", Spacing::None, false) {
                if !fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("param"), Spacing::None, false) {
                    // TODO: FIXME: Ensure everything after that is valid..
                    // This will be false if you are in an incomplete (but acceptable) function.
                    // I.E: fun(a, b,)
                    break;
                }
            }
        }
    }
}

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> old_function: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until(&mut cursor, FmtNode::Named("function"), true);
    fmter.write_indent();
    expressions::resolve(fmter, &cursor.node());
    cursor.goto_next_sibling();

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Named("arguments"), FmtNode::Token(";")],
        true
    );
    
    fmter.write_edit(String::from("("), Spacing::None);
    
    if fmter.is_stop(&mut cursor, &FmtNode::Named("arguments")) {
        format_args(fmter, &cursor.node());
        cursor.goto_next_sibling();
    }
    
    fmter.write_edit(String::from(")"), Spacing::None);

    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(";"), ";\n", Spacing::None, true);
}
