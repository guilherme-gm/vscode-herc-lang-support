use super::script_formatter::ScriptFormatter;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

pub mod block;
pub mod break_stmt;
pub mod case_statement;
pub mod continue_stmt;
pub mod do_while_statement;
pub mod end_stmt;
pub mod expression_statement;
pub mod for_statement;
pub mod function_decl;
pub mod function_stmt;
pub mod goto_stmt;
pub mod if_statement;
pub mod labeled_statement;
pub mod old_function;
pub mod return_stmt;
pub mod switch_statement;
pub mod while_statement;

pub fn resolve(fmter: &mut ScriptFormatter, node: &Node) -> bool {
	fmter.info(format!("> resolve_stmt: {:?}", node));
	match node.kind().to_lowercase().as_str() {
		"block" => {
			block::format(fmter, node);
		}
		"break_statement" => {
			break_stmt::format(fmter, node);
		}
		"case_statement" => {
			case_statement::format(fmter, node);
		}
		"continue_statement" => {
			continue_stmt::format(fmter, node);
		}
		"do_while_statement" => {
			do_while_statement::format(fmter, node);
		}
		"end_statement" => {
			end_stmt::format(fmter, node);
		}
		"for_statement" => {
			for_statement::format(fmter, node);
		}
		"function_declaration" => {
			function_decl::format(fmter, node);
		}
		"function_statement" => {
			function_stmt::format(fmter, node);
		}
		"goto_statement" => {
			goto_stmt::format(fmter, node);
		}
		"if_statement" => {
			if_statement::format(fmter, node);
		}
		"labeled_statement" => {
			labeled_statement::format(fmter, node);
		}
		"old_function" => {
			old_function::format(fmter, node);
		}
		"return_statement" => {
			return_stmt::format(fmter, node);
		}
		"switch_statement" => {
			switch_statement::format(fmter, node);
		}
		"while_statement" => {
			while_statement::format(fmter, node);
		}
		"expression_statement" => {
			expression_statement::format(fmter, node);
		}
		_ => return false,
	}

	true
}
