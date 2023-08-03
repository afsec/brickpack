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

use clap::ArgMatches;
use std::path::PathBuf;

pub async fn start(clap_matches: ArgMatches, dotenv_path: Option<PathBuf>) -> anyhow::Result<()> {
    use application_models::DatabaseConnection;
    use axum::{routing::get, Extension, Router};
    use axum_server::tls_rustls::RustlsConfig;
    use design_scaffold::oid::{ObjectIdReactor, OidPool};
    use std::{net::SocketAddr, str::FromStr};
    use tower::ServiceBuilder;
    use tower_cookies::CookieManagerLayer;
    use tower_http::trace::TraceLayer;

    // * Tracing
    start_tracing(&clap_matches);

    // * Dotenv message
    match dotenv_path {
        Some(path) => {
            tracing::info!("File `.env` found at {:?}", path);
        }
        None => {
            tracing::info!("File `.env` not found!");
        }
    };

    // * Define IPv4 ADDRESS and PORT for Listenning
    let mut addr: String = "127.0.0.1".into();
    let mut port: String = "8000".into();

    if clap_matches.is_present("ipv4_port") {
        let port_number_str = clap_matches.value_of("ipv4_port").unwrap();
        // Check if a valid port number [0 - 65535]
        let port_number = match port_number_str.parse::<u16>() {
            Ok(port) => port,
            Err(error) => {
                let error_msg = format!("Cannot parse IPV4_PORT value -> {}", error);
                return Err(anyhow::Error::msg(error_msg));
            }
        };
        port = port_number.to_string();
    }

    if clap_matches.is_present("ipv4_address") {
        use std::net::Ipv4Addr;
        let ipv4_addr_str = clap_matches.value_of("ipv4_address").unwrap();
        // * Check if a valid IPv4 address (0.0.0.0 - 255.255.255.255)
        let ipv4_addr: Ipv4Addr = match ipv4_addr_str.parse::<Ipv4Addr>() {
            Ok(ip4_addr) => ip4_addr,
            Err(error) => {
                let error_msg = format!("Cannot parse IPV4_ADDRESS value -> {}", error);
                return Err(anyhow::Error::msg(error_msg));
            }
        };

        addr = ipv4_addr.to_string();
    }

    // * Activate DEV_MODE
    if clap_matches.is_present("dev_mode") {
        // * Easy way to comunicate to `web_ui::main_js` that the application is under DEV MODE.
        std::env::set_var("ENABLE_DEV_MODE", "");

        let dev_mode = std::env::var("ENABLE_DEV_MODE");
        if dev_mode.is_ok() {
            tracing::warn!("DEV_MODE enabled!");
            // * Security requirement
            addr = "127.0.0.1".into();
            tracing::warn!("DEV_MODE supports listening address on {addr} only.");
        } else {
            tracing::error!("ERROR: It's not possible to enable DEV_MODE.");
        }
    }
    let listen = format!("{}:{}", addr, port);
    tracing::debug!("Parsed listen string: [{}]", &listen);
    let addr = SocketAddr::from_str(listen.as_str())?;

    // * OID Reactor
    let oid_pool: OidPool = ObjectIdReactor::new().await?;
    // let oid_pool = OidPool(oid_reactor_synced);

    // * Database connection
    let sqlite_pool = DatabaseConnection::new(Some(&oid_pool), "./database.sqlite3").await?.take();

    // * Routes
    let app = Router::new()
        .route("/", get(web_ui::index_page))
        .route("/auth", get(application_endpoints::check_auth))
        .route("/health", get(application_endpoints::health_check))
        // * Frontend
        .route("/js/main.js", get(web_ui::main_js))
        .route("/js/elm-loader.js", get(web_ui::elm_loader_js))
        // * REST APIs
        // TODO: Implement JWT session
        .nest("/api", application_endpoints::routes().await)
        .layer(CookieManagerLayer::new())
        .layer(ServiceBuilder::new().layer(Extension(oid_pool)))
        .layer(ServiceBuilder::new().layer(Extension(sqlite_pool)))
        .layer(TraceLayer::new_for_http());

    // * Show endpoints
    if clap_matches.is_present("endpoints") {
        let endpoints_str = format!("{:#?}", &app);
        show_endpoints(endpoints_str);
        Ok(())
    } else {
        // * Webserver
        // TODO:   Implement graceful shutdown by blocking all new connections
        // TODO: and wait until all pending responses complete.
        tracing::info!("Webserver started!");

        // * TLS (rustls support)
        if clap_matches.is_present("auto_generate_tls_cert")
            || (clap_matches.is_present("tls_cert_path") && clap_matches.is_present("tls_key_path"))
        {
            let tls_config: RustlsConfig;
            if clap_matches.is_present("auto_generate_tls_cert") {
                use rcgen::generate_simple_self_signed;
                let hostname = clap_matches.value_of("auto_generate_tls_cert").unwrap();
                dbg!(&hostname);
                // let hostname = "localhost";
                let cert = generate_simple_self_signed(vec![hostname.into()]).unwrap();
                let cert_pem = cert.serialize_pem().unwrap().into_bytes();
                let priv_key_pem = cert.serialize_private_key_pem().into_bytes();

                // let cert_der = cert.serialize_der().unwrap();
                // let priv_key_vec = cert.serialize_private_key_der();
                // let priv_key = rustls::PrivateKey(priv_key_vec);
                // tls_config = RustlsConfig::from_der(vec![cert_der], priv_key_vec).await?;

                // let cert_chain = vec![rustls::Certificate(cert_der.clone())];

                tls_config = RustlsConfig::from_pem(cert_pem, priv_key_pem).await?;
            } else {
                let tls_cert_path = clap_matches.value_of("tls_cert_path").unwrap();
                let tls_key_path = clap_matches.value_of("tls_key_path").unwrap();
                tls_config = RustlsConfig::from_pem_file(tls_cert_path, tls_key_path).await?
            }
            dbg!(&tls_config);
            tracing::info!("Listening on https://{}", addr);
            axum_server::bind_rustls(addr, tls_config).serve(app.into_make_service()).await?;
        } else {
            tracing::info!("Listening on http://{}", addr);
            axum::Server::bind(&addr).serve(app.into_make_service()).await?;
        }
        Ok(())
    }
}

pub fn show_endpoints(endpoints_str: String) {
    let mut endpoints: Vec<&str> = endpoints_str
        .lines()
        .filter(|line| line.contains("): \""))
        .filter_map(|line| line.split("\"").nth(1))
        .collect();
    if !endpoints.is_empty() {
        println!("    Endpoints:");
        endpoints.sort_by(|a, b| a.cmp(b));
        for endpoint in endpoints {
            println!("           - {}", endpoint);
        }
        println!("")
    } else {
        panic!("No endpoints found!")
    }
}

fn start_tracing(clap_matches: &ArgMatches) {
    use std::env;
    // let default_loglevel = "applications-endpoints=debug";
    let default_loglevel = "info";
    let mut was_loglevel_set_at_startup = true;
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", default_loglevel);
        was_loglevel_set_at_startup = false;
    }

    let startup_message = format!("Starting App [Brickpack v{}]:", env!("CARGO_PKG_VERSION"));

    if clap_matches.is_present("tokio_console") {
        console_subscriber::init();
        tracing::info!("{}", startup_message);
        tracing::info!("Tokio-console started at http://127.0.0.1:6669");
    } else {
        tracing_subscriber::fmt::init();
        tracing::info!("{}", startup_message);
        tracing::info!("Tracing started successfully");
    }
    if !was_loglevel_set_at_startup {
        tracing::info!(
            "RUST_LOG was not set. Setting default value: RUST_LOG={}",
            &default_loglevel
        );
    }
}
