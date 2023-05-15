use anyhow::anyhow;
use clap::{Arg, Command};
use design_scaffold::AppResult;

#[derive(Debug, Default)]
pub(crate) struct App {
    // config: (),
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
                Arg::new("endpoints")
                    .short('e')
                    .long("endpoints")
                    .help("Show endpoint names")
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
        use web_server::WebServer;
        WebServer::new().await?.run().await
    }
}
pub(crate) struct PreApp {
    cli: Option<Command>,
}
impl PreApp {
    pub(crate) fn load_cli(self) -> AppResult<App> {
        use std::fs;
        use std::path::PathBuf;
        let cli = self.take_cli().ok_or(anyhow!("CLI not defined"))?;

        let matches = cli.get_matches();

        let file_path =
            matches.get_one::<PathBuf>("dataset-name").ok_or(anyhow!("Argument not found"))?;

        // dbg!(file_path);

        let string_from_file = fs::read_to_string(file_path)?;
        dbg!(string_from_file);
        let app = App::default();
        Ok(app)
    }

    pub(crate) fn take_cli(self) -> Option<Command> {
        self.cli
    }
}
