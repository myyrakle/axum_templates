use serde::{Deserialize, Serialize};
use super::super::vo::#name#::#Name#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Get#Name#Response {
    pub data: #Name#,
}