use super::super::script_formatter::*;
use super::super::statements;
use tree_sitter::Node;

pub fn format_sub(fmter: &mut ScriptFormatter, node: &Node, line_break: bool, indent: bool) {
	fmter.info(format!("> block: {:?}", node));
	let mut cursor = node.walk();
	cursor.goto_first_child();

	fmter.match_until_and_write_str(
		&mut cursor,
		FmtNode::Token("{"),
		&"{\n",
		Spacing::None,
		true,
	);

	if indent {
		fmter.set_indent(fmter.indent_level + 1);
	}

	loop {
		if !fmter.match_until_one(
			&mut cursor,
			&[FmtNode::Token("}"), FmtNode::Named("stmt")],
			true,
		) {
			panic!("block:: End reached  before a }");
		}

		if fmter.is_stop(&mut cursor, &FmtNode::Token("}")) {
			break;
		}

		statements::resolve(fmter, &cursor.node());
		cursor.goto_next_sibling();
	}

	if indent {
		fmter.set_indent(fmter.indent_level - 1);
	}
	
	fmter.match_until_and_write_node(&mut cursor, FmtNode::Token("}"), Spacing::Indent, true);
	if line_break {
		fmter.write_newline();
	}
}

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
	format_sub(fmter, node, true, true);
}
