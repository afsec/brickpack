use anyhow::anyhow;
use clap::Command;
use design_scaffold::AppResult;

pub(crate) struct App {
    cli: Option<Command>,
}

impl App {
    pub(crate) fn new() -> Self {
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
            ;
        App { cli: Some(clap_command) }
    }

    pub(crate) fn load(&mut self) -> AppResult<()> {
        use std::fs;
        use std::path::PathBuf;
        let cli = self.take_cli().ok_or(anyhow!("CLI not defined"))?;

        let matches = cli.get_matches();

        let file_path =
            matches.get_one::<PathBuf>("dataset-name").ok_or(anyhow!("Argument not found"))?;

        // dbg!(file_path);

        let string_from_file = fs::read_to_string(file_path)?;
        dbg!(string_from_file);
        Ok(())
    }

    pub(crate) fn take_cli(&mut self) -> Option<Command> {
        self.cli.take()
    }
}
