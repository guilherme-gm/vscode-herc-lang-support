use std::collections::HashMap;
use serde::{Deserialize, Serialize, Deserializer};
use std::fmt::{Debug};

fn from_line_array<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(s.join(" "))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    #[serde(rename = "type")]
    pub param_type: String,
    pub default: String,
    #[serde(deserialize_with = "from_line_array")]
    pub doc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptCommand {
    /**
     * signatures array is desserialized into signature_params to be later converted into signatures field
     */
    #[serde(rename(deserialize = "signatures"))]
    pub signature_params: Vec<Vec<String>>,
    pub params: HashMap<String, Parameter>,
    #[serde(deserialize_with = "from_line_array")]
    pub doc: String,
    #[serde(rename = "return")]
    pub return_type: String,
    pub deprecated: bool,
    #[serde(skip)]
    pub signatures: Vec<String>,
}

impl ScriptCommand {
    pub fn generate_signatures(&mut self, cmd: &String) {
        if self.signature_params.len() == 0 {
            self.signatures.push(format!("{}()", cmd));
            return;
        }

        for param_set in &self.signature_params {
            let mut signature = String::new();
            signature.push_str(&cmd);
            signature.push('(');

            let mut param_count = 0;
            
            for param in param_set {
                if let Some(param_info) = self.params.get(param) {
                    signature.push_str(&format!("{}: {}, ", param, param_info.param_type));
                } else {
                    signature.push_str(&format!("{}: unknown, ", param));
                }
                param_count += 1;
            }
    
            if param_count > 0 {
                signature.pop(); // space
                signature.pop(); // ','
            }
            
            signature.push(')');
            self.signatures.push(signature);
        }
    }
}

pub fn load_prototypes(commands: &mut HashMap<String, ScriptCommand>) {
    for (cmd, cmd_data) in commands.iter_mut() {
        cmd_data.generate_signatures(&cmd);
    }
}