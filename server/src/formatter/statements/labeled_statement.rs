use super::super::expressions::identifier;
use super::super::script_formatter::*;
use super::super::statements;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
	fmter.info(format!("> label_stmt: {:?}", node));
	let mut cursor = node.walk();
	cursor.goto_first_child();

	fmter.match_until(&mut cursor, FmtNode::Named("label"), true);

	let save_indent = fmter.indent_level;
	fmter.set_indent(0);
	identifier::format(fmter, &cursor.node());
	cursor.goto_next_sibling();

	fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(":"), ":\n", true);

	fmter.match_until(&mut cursor, FmtNode::Named("body"), true);

	statements::resolve(fmter, &cursor.node());
	fmter.set_indent(save_indent);
}
