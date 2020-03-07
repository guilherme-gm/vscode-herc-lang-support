use super::super::expressions::identifier;
use super::super::script_formatter::*;
use super::super::statements;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
	fmter.info(format!("> label_stmt: {:?}", node));
	let mut cursor = node.walk();
	cursor.goto_first_child();

	fmter.match_until(&mut cursor, FmtNode::Named("label"), true);

	// TODO: Should we save the indent and restore? or start from 1?
	// let save_indent = fmter.indent_level;
	fmter.set_indent(0);
	identifier::format(fmter, &cursor.node());
	cursor.goto_next_sibling();

	fmter.match_until_and_write_node(&mut cursor, FmtNode::Token(":"), Spacing::None, true);
	fmter.write_newline();

	fmter.match_until(&mut cursor, FmtNode::Named("body"), true);

	fmter.set_indent(1);
	statements::resolve(fmter, &cursor.node());
}
