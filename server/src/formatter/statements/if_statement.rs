use super::super::expressions::parenthesized_expression;
use super::super::script_formatter::*;
use super::super::statements;
use tree_sitter::Node;

pub fn format_sub(fmter: &mut ScriptFormatter, node: &Node, if_spacing: Spacing) {
	fmter.info(format!("> if_stmt: {:?}", node));
	let mut cursor = node.walk();
	cursor.goto_first_child();

	fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("if"), "if ", if_spacing, true);
	fmter.match_until(&mut cursor, FmtNode::Named("condition"), true);
	parenthesized_expression::format(fmter, &cursor.node());
	cursor.goto_next_sibling();

	let mut is_block = false;
	fmter.match_until(&mut cursor, FmtNode::Named("consequence"), true);
	if cursor.node().kind().eq_ignore_ascii_case("block") {
		is_block = true;
		fmter.write_space();
		statements::block::format_sub(fmter, &cursor.node(), false, true);
	} else {
		fmter.write_newline();
		fmter.set_indent(fmter.indent_level + 1);
		statements::resolve(fmter, &cursor.node());
		fmter.set_indent(fmter.indent_level - 1);
	}
	if !cursor.goto_next_sibling() || !fmter.match_until(&mut cursor, FmtNode::Token("else"), false)
	{
		if is_block {
			fmter.write_newline();
		}
		return;
	}

	let else_str;
	let spacing;
	if is_block {
		else_str = " else ";
		spacing = Spacing::None;
	} else {
		else_str = "else";
		spacing = Spacing::Indent;
	}

	fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("else"), else_str, spacing, true);

	fmter.match_until(&mut cursor, FmtNode::Named("alternative"), true);

	if cursor.node().kind().eq_ignore_ascii_case("if_statement") {
		if !is_block {
			fmter.write_space();
		}

		format_sub(fmter, &cursor.node(), Spacing::None);
	} else {
		let is_else_block;
		if !cursor.node().kind().eq_ignore_ascii_case("block") {
			is_else_block = false;
			fmter.write_newline();
			fmter.set_indent(fmter.indent_level + 1);
		} else {
			is_else_block = true;
		}

		statements::resolve(fmter, &cursor.node());
		cursor.goto_next_sibling();

		if !is_else_block {
			fmter.set_indent(fmter.indent_level - 1);
		}
	}
}

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
	format_sub(fmter, node, Spacing::Indent);
	fmter.write_newline();
}
