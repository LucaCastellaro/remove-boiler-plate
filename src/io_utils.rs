use std::io::Error;
use dialoguer::Input;
use console::Style;
use lazy_static::lazy_static;

lazy_static! {
    static ref ERROR_RESPONSE: Style = Style::new().color256(124).bold();
    static ref VALID_RESPONSE: Style = Style::new().color256(034).bold();
}


pub fn text(label: &str) -> Result<String, Error> {
    return Input::<String>::new()
        .with_prompt(label)
        .interact_text();
}