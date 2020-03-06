use super::super::expressions;
use super::super::script_formatter::*;
use super::block;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
	fmter.info(format!("> function_decl: {:?}", node));
	let mut cursor = node.walk();
	cursor.goto_first_child();

	fmter.write_edit(format!("{}", fmter.indent));

	fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("function"), &"function ", true);
	
	fmter.match_until(&mut cursor, FmtNode::Named("name"), true);
	expressions::identifier::format(fmter, &cursor.node());
	
	fmter.match_until(&mut cursor, FmtNode::Named("body"), true);
	block::format(fmter, &cursor.node());
}
