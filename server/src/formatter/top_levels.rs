use tree_sitter::Node;

use crate::formatter::script_formatter::ScriptFormatter;

// TODO: pub mod mapflag_def;
pub mod duplicate_def;
pub mod function_def;
pub mod position;
pub mod script_def;
pub mod shop_def;
pub mod spawn_def;
pub mod trigger_area;
pub mod warp_def;

pub fn resolve(fmter: &mut ScriptFormatter, node: &Node) -> bool {
    fmter.info(format!("> resolve_top_level: {:?}", node));
    match node.kind().to_lowercase().as_str() {
        "duplicate_def" => {
            duplicate_def::format(fmter, &node);
        }
        "function_def" => {
            function_def::format(fmter, &node);
        }
        "script_def" => {
            script_def::format(fmter, &node);
        }
        "shop_def" => {
            shop_def::format(fmter, &node);
        }
        "spawn_def" => {
            spawn_def::format(fmter, &node);
        }
        "warp_def" => {
            warp_def::format(fmter, &node);
        }
        _ => return false,
    }

    true
}
