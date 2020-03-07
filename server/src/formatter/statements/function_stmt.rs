use super::super::expressions;
use super::super::script_formatter::*;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
	fmter.info(format!("> function_stmt: {:?}", node));
	let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("function"), &"function ", Spacing::Indent, true);

    fmter.match_until(&mut cursor, FmtNode::Named("name"), true);
    expressions::identifier::format(fmter, &cursor.node());
    cursor.goto_next_sibling();

    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(";"), &";\n", Spacing::None, true);
}
