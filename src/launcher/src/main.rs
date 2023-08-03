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

use clap::{crate_authors, crate_description, App as ClapApp, Arg as ClapArg};

pub trait App {
    fn name(&self) -> &'static str;

    fn version(&self) -> &'static str;
}

//
struct MyApp;
impl App for MyApp {
    fn name(&self) -> &'static str {
        "BrickPack"
    }
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // * Load App's Config from `.env` file.
    let dotenv_path = dotenv::dotenv().ok();

    // * Start Clap subsystem
    let clap_matches = ClapApp::new(MyApp.name())
        .version(MyApp.version())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            ClapArg::new("endpoints")
                .short('e')
                .long("endpoints")
                .help("Show endpoint names")
                .exclusive(true),
        )
        .arg(
            ClapArg::new("ipv4_address")
                .long("ipv4-address")
                .value_name("IPV4_ADDRESS")
                .require_equals(true)
                .help("IPv4 address to listen")
                .env("IPV4_ADDRESS")
                .forbid_empty_values(true)
                .required(false),
        )
        .arg(
            ClapArg::new("ipv4_port")
                .long("ipv4-port")
                .value_name("IPV4_PORT")
                .require_equals(true)
                .help("Port number to listen")
                .env("IPV4_PORT")
                .forbid_empty_values(true)
                .required(false),
        )
        .arg(
            ClapArg::new("dev_mode")
                .long("enable-dev-mode")
                .env("ENABLE_DEV_MODE")
                .takes_value(false)
                .help("Endpoint `/js/main.js` serve a local file `./main.js` instead of default frontend.")
                .required(false),
        )
        .arg(
            ClapArg::new("tokio_console")
                .long("enable-tokio-console")
                .env("ENABLE_TOKIO_CONSOLE")
                .takes_value(false)
                .help("Enable tokio-console")
                .required(false),
        )
        .arg(
            ClapArg::new("tls_cert_path")
                .long("tls-cert-path")
                .value_name("CERT_PATH")
                .require_equals(true)
                .forbid_empty_values(true)
                .help("TLS certificate file")
                .env("TLS_CERT_PATH")
                .required(false),
        )
        .arg(
            ClapArg::new("tls_key_path")
                .long("tls-key-path")
                .value_name("KEY_PATH")
                .require_equals(true)
                .forbid_empty_values(true)
                .help("TLS private key file")
                .env("TLS_KEY_PATH")
                .required(false),
        )
        .arg(
            ClapArg::new("auto_generate_tls_cert")
                .long("auto-tls-for-hostname")
                .value_name("HOSTNAME")
                .require_equals(true)
                .forbid_empty_values(true)
                .help("Hostname for auto-generated TLS cert")
                .env("AUTO_TLS_FOR_HOSTNAME")
                .required(false),
        )
        .get_matches();
    web_server::start(clap_matches, dotenv_path).await
}
