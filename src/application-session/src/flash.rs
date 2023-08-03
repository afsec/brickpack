use std::ops::Deref;

use tower_cookies::{Cookie, Cookies};
pub struct FlashCookie(Option<String>);
impl Deref for FlashCookie {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl FlashCookie {
    pub fn take(self) -> Option<String> {
        self.0
    }
}
impl From<Option<Cookie<'_>>> for FlashCookie {
    fn from(option_cookie: Option<Cookie>) -> Self {
        match option_cookie {
            Some(cookie) => Self(Some(cookie.value().to_string())),
            None => Self(None),
        }
    }
}

const FLASH_COOKIE_NAME: &str = "_flash";

pub fn get_flash_cookie(cookies: &Cookies) -> FlashCookie {
    let flash_cookie = FlashCookie::from(cookies.get(FLASH_COOKIE_NAME));
    if let Some(content) = &*flash_cookie {
        tracing::warn!("FLASH FOUND: {content}");
    }

    flash_cookie
}

pub fn session_response(cookies: &mut Cookies) -> design_scaffold::Result<()> {
    use cookie::Expiration;
    use sha2::{Digest, Sha256};
    use std::time::Instant;
    use time::OffsetDateTime;
    use ulid::Ulid;

    let monotonic_now = Instant::now();

    let mut offset_datetime_now = OffsetDateTime::now_utc();

    let new_session_id: u128 = Ulid::from_datetime(*&offset_datetime_now).into();

    let mut hasher = Sha256::new();
    let new_ulid = new_session_id.to_be_bytes();
    hasher.update(new_ulid);

    let new_session_id = hasher.finalize();
    let next_flash_cookie = format!("{new_session_id:x}");

    offset_datetime_now += time::Duration::days(2);
    let cookie_expires_at = Expiration::DateTime(offset_datetime_now);

    let flash_cookie = Cookie::build(FLASH_COOKIE_NAME, next_flash_cookie)
        .expires(cookie_expires_at)
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();
    cookies.add(flash_cookie.into());

    let elapsed = monotonic_now.elapsed().as_micros();

    let duration_cookie = Cookie::build("millis", elapsed.to_string())
        .expires(cookie_expires_at)
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    cookies.add(duration_cookie);

    Ok(())
}
