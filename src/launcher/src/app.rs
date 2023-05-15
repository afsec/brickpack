use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};

use std::net::SocketAddr;
use std::str::FromStr;

use design_scaffold::{AppError, AppResult};
use web_server::{WebServer, WebServerConfig, WebServerTlsConfig};

#[derive(Debug, Default)]
pub(crate) struct App {
    config: AppConfig,
}

impl App {
    pub(crate) fn new() -> PreApp {
        let clap_command = Command::new(clap::crate_name!())
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about(clap::crate_description!())
            // .arg(
            //     Arg::new("dataset-name")
            //         .value_name("DATASET_NAME")
            //         .value_hint(ValueHint::FilePath)
            //         .env("DATASET_NAME")
            //         .value_parser(clap::builder::ValueParser::new(dataset_file_parse_wrap))
            //         .help("Dataset file .json file to load")
            //         .required(true)
            //         .num_args(1),
            // )
            .arg(
                Arg::new("show_endpoints")
                    .short('e')
                    .long("endpoints")
                    .help("Show endpoint names")
                    .num_args(0)
                    .exclusive(true),
            )
            .arg(
                Arg::new("ipv4_address")
                    .long("ipv4-address")
                    .value_name("IPV4_ADDRESS")
                    .require_equals(true)
                    .help("IPv4 address to listen")
                    .env("IPV4_ADDRESS")
                    .num_args(1)
                    .required(false),
            )
            .arg(
                Arg::new("ipv4_port")
                    .long("ipv4-port")
                    .value_name("IPV4_PORT")
                    .require_equals(true)
                    .help("Port number to listen")
                    .env("IPV4_PORT")
                    .num_args(1)
                    .required(false),
            )
            .arg(
                Arg::new("dev_mode")
                    .long("enable-dev-mode")
                    .env("ENABLE_DEV_MODE")
                    .num_args(0)
                    .help("Endpoint `/js/main.js` serve a local file `./main.js` instead of default frontend.")
                    .required(false),
            )
            .arg(
                Arg::new("tokio_console")
                    .long("enable-tokio-console")
                    .env("ENABLE_TOKIO_CONSOLE")
                    .num_args(0)
                    .help("Enable tokio-console")
                    .required(false),
            )
            .arg(
                Arg::new("tls_cert_path")
                    .long("tls-cert-path")
                    .value_name("CERT_PATH")
                    .require_equals(true)
                    .num_args(1)
                    .help("TLS certificate file")
                    .env("TLS_CERT_PATH")
                    .required(false),
            )
            .arg(
                Arg::new("tls_key_path")
                    .long("tls-key-path")
                    .value_name("KEY_PATH")
                    .require_equals(true)
                    .num_args(1)
                    .help("TLS private key file")
                    .env("TLS_KEY_PATH")
                    .required(false),
            )
            .arg(
                Arg::new("auto_generate_tls_cert")
                    .long("auto-tls-for-hostname")
                    .value_name("HOSTNAME")
                    .require_equals(true)
                    .num_args(1)
                    .help("Hostname for auto-generated TLS cert")
                    .env("AUTO_TLS_FOR_HOSTNAME")
                    .required(false),
            )
            ;
        PreApp { cli: Some(clap_command) }
    }
    pub(crate) async fn run(self) -> AppResult<()> {
        let app_config = self.config;

        if app_config.show_endpoints {
            println!("RUN: Show endpoints");
            Ok(())
        } else {
            let wb_config = WebServerConfig::from(app_config);
            WebServer::new(wb_config).run().await
        }
    }
}
pub(crate) struct PreApp {
    cli: Option<Command>,
}
impl PreApp {
    pub(crate) fn load_cli(self) -> AppResult<App> {
        let cli = self.take_cli().ok_or(anyhow!("CLI not defined"))?;

        let matches = cli.get_matches();

        // * Tracing
        start_tracing(&matches);

        let tls_config = WebServerTlsConfig::from_matches(&matches)?;

        let auto_generate_tls_cert_hostname =
            match matches.get_one::<String>("auto_generate_tls_cert") {
                Some(possible_hostname) => {
                    use std::ops::Not;
                    if hostname_validator::is_valid(possible_hostname).not() {
                        return Err(anyhow!("Invalid hostname for auto_generate_tls_cert"));
                    }
                    Some(possible_hostname.clone())
                }
                None => None,
            };

        let socket = AppSocket::from_matches(&matches)?;

        Ok(App {
            config: AppConfig {
                dev_mode: matches.get_flag("dev_mode"),
                show_endpoints: matches.get_flag("show_endpoints"),
                tokio_console: matches.get_flag("tokio_console"),
                socket,
                tls_config,
                auto_generate_tls_cert_hostname,
            },
        })
    }

    pub(crate) fn take_cli(self) -> Option<Command> {
        self.cli
    }
}

#[derive(Debug, Default)]
struct AppConfig {
    show_endpoints: bool,
    dev_mode: bool,
    tokio_console: bool,
    socket: AppSocket,
    tls_config: Option<WebServerTlsConfig>,
    auto_generate_tls_cert_hostname: Option<String>,
}

#[derive(Debug)]
struct AppSocket(SocketAddr);
impl FromStr for AppSocket {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
    }
}
impl Default for AppSocket {
    fn default() -> Self {
        use std::net::IpAddr;
        use std::net::Ipv4Addr;
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        Self(socket)
    }
}
impl AppSocket {
    fn take_inner(self) -> SocketAddr {
        self.0
    }
    fn from_matches(matches: &ArgMatches) -> AppResult<Self> {
        use std::net::IpAddr;
        use std::net::Ipv4Addr;
        let maybe_ipv4_address = matches.get_one::<Ipv4Addr>("ipv4_address");
        let maybe_ipv4_port = matches.get_one::<u16>("ipv4_port");

        let socket = match (maybe_ipv4_address, maybe_ipv4_port) {
            (Some(inner_ipv4_address), Some(inner_ipv4_port)) => {
                let socket_addr =
                    SocketAddr::new(IpAddr::V4(*inner_ipv4_address), *inner_ipv4_port);
                Ok(AppSocket(socket_addr))
            }
            (None, None) => Ok(AppSocket::default()),
            _ => Err(anyhow!("Both ipv4_address and ipv4_port need to be defined together")),
        }?;

        Ok(socket)
    }
}

impl From<AppConfig> for WebServerConfig {
    fn from(app_config: AppConfig) -> Self {
        let AppConfig {
            dev_mode,
            tokio_console,
            socket,
            tls_config,
            auto_generate_tls_cert_hostname,
            ..
        } = app_config;
        Self {
            dev_mode,
            tokio_console,
            socket: socket.take_inner(),
            tls_config,
            auto_generate_tls_cert_hostname,
        }
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
    if clap_matches.get_flag("tokio_console") {
        // if clap_matches.is_present("tokio_console") {
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
