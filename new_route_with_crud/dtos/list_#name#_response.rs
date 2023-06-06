use serde::{Deserialize, Serialize};
use super::super::vo::#name#::#Name#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List#Name#Response {
    pub list: Vec<#Name#>,
}