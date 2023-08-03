// #![warn(clippy::all)]

#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    // missing_debug_implementations,
    // missing_docs
)]
#![deny(unreachable_pub, private_in_public)]
#![allow(elided_lifetimes_in_paths, clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]

use std::collections::HashMap;

use application_session::flash::{get_flash_cookie, session_response};
use axum::{http::StatusCode, response::Html};
use tower_cookies::Cookies;

#[derive(Debug)]
struct FlashData(String);

pub async fn index_page(mut cookies: Cookies) -> Result<Html<String>, (StatusCode, &'static str)> {
    tracing::debug!("Endpoint Found: index_page()");
    let mut ctx: HashMap<&str, String> = HashMap::new();
    if let Some(value) = get_flash_cookie(&cookies).take() {
        ctx.insert("flash", value);
    }

    // TODO: - Implement MVP
    // TODO: - Implement Form login
    // TODO: - Implement Cookie on Auth suceeded to helps
    // TODO:    on session management (JWT)
    // TODO: - Implement session management (JWT)
    let session_result = session_response(&mut cookies);
    // TODO:
    drop(session_result);

    tracing::debug!("CTX: {ctx:?}");
    let body = include_str!("../artifacts/main.html");
    Ok(Html(body.to_string()))
}

pub async fn main_js() -> Result<Html<String>, (StatusCode, &'static str)> {
    tracing::debug!("Endpoint Found: main_js()");
    let default_body = include_str!("../artifacts/js/main.js");
    // * SECURITY_TRADEOFF
    let dev_mode = std::env::var("ENABLE_DEV_MODE");

    if dev_mode.is_ok() {
        match tokio::fs::read_to_string("main.js").await {
            Ok(body) => Ok(Html(body)),
            Err(error) => {
                tracing::warn!("Error on reading main.js. Reason: {error}");
                Ok(Html(default_body.to_string()))
            }
        }
    } else {
        Ok(Html(default_body.to_string()))
    }
}

pub async fn elm_loader_js() -> Result<Html<String>, (StatusCode, &'static str)> {
    tracing::debug!("Endpoint Found: elm_loader_js()");
    let body = include_str!("../artifacts/js/elm-loader.js");
    Ok(Html(body.to_string()))
}
