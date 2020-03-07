use super::super::expressions;
use super::super::script_formatter::*;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
	fmter.info(format!("> parent_exp: {:?}", node));
	let mut cursor = node.walk();
	cursor.goto_first_child();

	fmter.match_until_and_write_node(&mut cursor, FmtNode::Token("("), Spacing::None, true);

	fmter.match_until(&mut cursor, FmtNode::Named("expr"), true);
	expressions::resolve(fmter, &cursor.node());
	cursor.goto_next_sibling();

	fmter.match_until_and_write_node(&mut cursor, FmtNode::Token(")"), Spacing::None, true);
}
