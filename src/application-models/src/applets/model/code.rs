use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

const MAX_CODE_LENGTH: usize = 2_097_152; // 1 MByte (2^21)

#[derive(Debug, Serialize, Deserialize)]
pub struct AppletCode(String);

impl Deref for AppletCode {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AppletCode {
    pub fn take(self) -> String {
        self.0
    }
}

// SQLx record type
impl From<String> for AppletCode {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
impl DataValidator for AppletCode {
    async fn validate(&self) -> design_scaffold::Result<()> {
        use bytesize::ByteSize;
        let code_in_base64 = &*(*self);
        if code_in_base64.is_empty() {
            return Err(design_scaffold::Error::DataValidation("Invalid code: It's empty.".into()));
        }
        let code_length = code_in_base64.len();
        if code_length > MAX_CODE_LENGTH {
            return Err(design_scaffold::Error::DataValidation(format!(
                "Invalid code: More than {}.",
                ByteSize::b(code_length.try_into()?)
            )));
        }
        // * Check if is base64 RFC3548
        for c in code_in_base64.as_bytes() {
            if c.is_ascii_alphanumeric() || *c == b'+' || *c == b'/' || *c == b'=' {
                () // Valid char
            } else {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid code: It has invalid characters for Base64(RFC3548).".into(),
                ));
            }
        }

        // * Check lua syntax
        let bytes_from_b64 = base64::decode(code_in_base64)?;
        let code = String::from_utf8(bytes_from_b64)?;
        if let Err(error) = full_moon::parse(&code) {
            return Err(design_scaffold::Error::AppletNewCodeValidation(error.to_string()));
        }

        Ok(())
    }
}
