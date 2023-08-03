use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn get(&self) -> &String {
        &self.0
    }
}

impl Deref for UserEmail {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx record type
impl From<String> for UserEmail {
    fn from(value: String) -> Self {
        Self(value)
    }
}
#[async_trait]
impl DataValidator for UserEmail {
    async fn validate(&self) -> design_scaffold::Result<()> {
        {
            const MAX_LENGTH: usize = 512;
            use regex::Regex;

            // Regex patterns from https://docs.rs/validator/0.14.0/src/validator/validation/email.rs.html#12-13

            let user_re: Regex = Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap();
            let domain_re: Regex = Regex::new(
        r"(?i)^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
    ).unwrap();

            let email_address = &self.0;
            if email_address.is_empty() {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid email: It's empty".into(),
                ));
            }
            if !email_address.contains('@') {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid email: There is no @".into(),
                ));
            }
            if email_address.matches('@').count() > 1 {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid email: There is more than one @".into(),
                ));
            }
            if email_address.len() > MAX_LENGTH {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid email: More than 512 Bytes".into(),
                ));
            }

            let parts: Vec<&str> = email_address.split('@').collect();
            let user = parts[0];
            let domain = parts[1];

            if !user_re.is_match(user) {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid email on part(user) before @".into(),
                ));
            }

            if !domain_re.is_match(domain) {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid email on part(domain) after @".into(),
                ));
            }

            Ok(())
        }
    }
}
