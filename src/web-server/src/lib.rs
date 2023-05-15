use anyhow::anyhow;
use clap::ArgMatches;

use design_scaffold::AppResult;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug)]
pub struct WebServer {
    config: WebServerConfig,
}

impl WebServer {
    pub fn new(config: WebServerConfig) -> Self {
        Self { config }
    }
    pub async fn run(self) -> AppResult<()> {
        let Self { config } = self;

        dbg!(config);

        println!("[{}:{}] Hello, world!", file!(), line!());

        Ok(())
    }
}

#[derive(Debug)]
pub struct WebServerConfig {
    pub dev_mode: bool,
    pub tokio_console: bool,
    pub socket: SocketAddr,
    pub tls_config: Option<WebServerTlsConfig>,
    pub auto_generate_tls_cert_hostname: Option<String>,
}

#[derive(Debug)]
pub struct WebServerTlsConfig {
    pub tls_cert_path: PathBuf,
    pub tls_key_path: PathBuf,
}

impl WebServerTlsConfig {
    pub fn from_matches(matches: &ArgMatches) -> AppResult<Option<Self>> {
        let maybe_tls_key_path = matches.get_one::<PathBuf>("tls_key_path");
        let maybe_tls_cert_path = matches.get_one::<PathBuf>("tls_cert_path");

        let maybe_app_tls_config = match (maybe_tls_key_path, maybe_tls_cert_path) {
            (Some(inner_tls_key_path), Some(inner_tls_cert_path)) => Some(WebServerTlsConfig {
                tls_cert_path: inner_tls_cert_path.to_path_buf(),
                tls_key_path: inner_tls_key_path.to_path_buf(),
            }),
            (None, None) => None,
            _ => {
                return Err(anyhow!(
                    "Both tls_key_path and tls_cert_path need to be defined together"
                ))
            }
        };
        Ok(maybe_app_tls_config)
    }
}
