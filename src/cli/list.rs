use std::{collections::HashMap, fs::read_dir};

use clap::Parser;
use colorful::Colorful;

use crate::{DEFAULT_SOURCE_DIR, Error};

#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    /// word list source directory
    #[arg(short, long)]
    pub dir: Option<String>,
}

impl Cmd {
    pub fn run(self, settings: HashMap<String, String>) -> Result<(), Error> {
        // Setup settings
        let mut src_directory = settings
            .get("source_dir")
            .map_or(DEFAULT_SOURCE_DIR, |v| v)
            .to_string();

        if let Some(sd) = self.dir {
            src_directory = sd;
        };

        // List the word list files
        let mut title = "Word lists:".yellow().bold().underlined().to_string();
        println!("{}", title);
        for entry in read_dir(&src_directory).unwrap().flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some() && path.extension().unwrap() == "txt" {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                println!("  {}", file_name);
            }
        }

        // List the boxed puzzle word list files
        title = "Word lists for boxed puzzles:"
            .yellow()
            .bold()
            .underlined()
            .to_string();
        println!("\n{}", title);

        for entry in read_dir(&src_directory).unwrap().flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some() && path.extension().unwrap() == "slb" {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                println!("  {}", file_name);
            }
        }

        Ok(())
    }
}
