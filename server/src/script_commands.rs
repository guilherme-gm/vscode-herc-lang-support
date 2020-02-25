use std::collections::HashMap;
use serde::{Deserialize, Serialize, Deserializer};
use std::fmt::{Debug};

fn from_line_array<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(s.join("\n"))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub default: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptCommand {
    pub params: Vec<Vec<Parameter>>,
    #[serde(rename = "paramDoc")]
    pub param_doc: HashMap<String, String>,
    #[serde(deserialize_with = "from_line_array")]
    pub doc: String,
    #[serde(default)]
    pub prototype: Option<String>,
}

impl ScriptCommand {
    pub fn generate_prototype(&mut self, cmd: &String) {
        if let Some(_) = self.prototype {
            return;
        }

        let mut larger_idx: Option<&Vec<Parameter>> = None;
        let mut larger_cnt: usize = 0;

        for param_list in &self.params {
            if param_list.len() > larger_cnt {
                larger_idx = Some(&param_list);
                larger_cnt = param_list.len();
            }
        }

        let mut prot = String::new();
        prot.push_str(&cmd);
        prot.push('(');
        if let Some(params) = larger_idx {
            for param in params {
                prot.push_str(&format!("{}: {}, ", param.name, param.param_type));
            }

            if larger_cnt > 0 {
                prot.pop(); // space
                prot.pop(); // ','
            }
        }
        prot.push(')');

        self.prototype = Some(prot);
    }
}

pub fn load_prototypes(commands: &mut HashMap<String, ScriptCommand>) {
    for (cmd, cmd_data) in commands.iter_mut() {
        cmd_data.generate_prototype(&cmd);
    }
}