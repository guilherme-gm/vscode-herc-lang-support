use tree_sitter::Node;

use crate::formatter::script_formatter::ScriptFormatter;

pub mod assignment_expression;
pub mod binary_expression;
pub mod conditional_expression;
pub mod exp_false;
pub mod exp_number_literal;
pub mod exp_string;
pub mod exp_true;
pub mod function_expression;
pub mod identifier;
pub mod parenthesized_expression;
pub mod scoped_identifier;
pub mod subscript_expression;
pub mod unary_expression;
pub mod update_expression;

pub fn resolve(fmter: &mut ScriptFormatter, node: &Node) -> bool {
	fmter.info(format!("> exp_resolve: {:?}", node));
	match node.kind().to_lowercase().as_str() {
		"conditional_expression" => conditional_expression::format(fmter, node),
		"assignment_expression" => assignment_expression::format(fmter, node),
		"unary_expression" => unary_expression::format(fmter, node),
		"binary_expression" => binary_expression::format(fmter, node),
		"false" => exp_false::format(fmter, node),
		"true" => exp_true::format(fmter, node),
		"number_literal" => exp_number_literal::format(fmter, node),
		"string" => exp_string::format(fmter, node),
		"function_expression" => function_expression::format(fmter, node),
		"subscript_expression" => subscript_expression::format(fmter, node),
		"update_expression" => update_expression::format(fmter, node),
		"identifier" => identifier::format(fmter, node),
		"scoped_identifier" => scoped_identifier::format(fmter, node),
		"parenthesized_expression" => parenthesized_expression::format(fmter, node),
		_ => return false,
	}

	true
}
