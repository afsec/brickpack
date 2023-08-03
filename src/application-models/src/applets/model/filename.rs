use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppletFilename(String);

impl Deref for AppletFilename {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx record type
impl From<String> for AppletFilename {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
impl DataValidator for AppletFilename {
    async fn validate(&self) -> design_scaffold::Result<()> {
        use regex::Regex;
        const MAX_LENGTH: usize = 512;

        let filename = &*(*self);

        if filename.is_empty() {
            return Err(design_scaffold::Error::DataValidation(
                "Invalid filename: It's empty".into(),
            ));
        }
        if filename.len() > MAX_LENGTH {
            return Err(design_scaffold::Error::DataValidation(
                "Invalid filename: More than 512 Bytes".into(),
            ));
        }

        // Generated with `Rustexp` on https://rustexp.lpil.uk/
        let file_name_regex: Regex = Regex::new(r"^([a-z]{1}[a-z0-9_]{0,60})$").unwrap();

        let mut char_counter = 0;

        // * Check if is a valid ascii string
        for c in filename.chars() {
            if !c.is_ascii() {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid filename: Must have ONLY ascii alphanumeric characters OR '_' char"
                        .into(),
                ));
            }

            if char_counter == 0 && !c.is_ascii_lowercase() {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid filename: Must start with lowercase character".into(),
                ));
            }

            if !(c.is_ascii_lowercase() || c.is_ascii_digit() || (c == '_') || (c == '.')) {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid filename: It has invalid characters".into(),
                ));
            }
            char_counter += 1;
        }
        // * Check if is a valid ascii string
        if !(filename.matches('.').count() == 1) {
            return Err(design_scaffold::Error::DataValidation(
                "Invalid filename: Must have only one '.' char".into(),
            ));
        }
        let file_name_vec: Vec<&str> = filename.split(".").collect();
        let name = file_name_vec[0];
        let extension = file_name_vec[1];
        if !(extension == "lua") {
            return Err(design_scaffold::Error::DataValidation(
                "Invalid filename: The filename must ends with '.lua' extension".into(),
            ));
        }
        if !file_name_regex.is_match(name) {
            return Err(design_scaffold::Error::DataValidation(
                "Invalid filename: The name of a file must have ONLY ascii alphanumeric characters OR '_' char".into(),
            ));
        }
        Ok(())
    }
}
