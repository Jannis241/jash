use crate::prelude::*;
pub fn handle(input: &String) -> Option<String> {
    let a = format!("Input bekommen: '{}'", input);
    Some(a.to_string())
}
