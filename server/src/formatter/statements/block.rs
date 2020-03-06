use super::super::script_formatter::*;
use super::super::statements;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
	fmter.info(format!("> block: {:?}", node));
	let mut cursor = node.walk();
	cursor.goto_first_child();

	
	fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("{"), &"{\n", true);
	fmter.set_indent(fmter.indent_level + 1);
	
	loop {
		if !fmter.match_until_one(&mut cursor, &[FmtNode::Token("}"), FmtNode::Named("stmt")], true) {
			panic!("block:: End reached  before a }");
		}

		if fmter.is_stop(&mut cursor, &FmtNode::Token("}")) {
			break;
		}

		statements::resolve(fmter, &cursor.node());
		cursor.goto_next_sibling();
	}

	fmter.set_indent(fmter.indent_level - 1);
	fmter.write_edit(String::from(format!("{}}}\n", fmter.indent)));
}
