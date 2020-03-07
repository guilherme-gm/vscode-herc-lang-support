use super::super::script_formatter::*;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> scoped_identifier_expr: {:?}", node));
    fmter.write_edit(fmter.get_node_text(node), Spacing::None);
}
