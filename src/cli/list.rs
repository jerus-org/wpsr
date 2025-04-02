use std::{collections::HashMap, fs::read_dir};

use clap::Parser;
const DEFAULT_SOURCE_DIR: &str = "words";

#[derive(Parser, Debug, Clone)]
pub struct CmdList {
    /// word list source directory
    #[arg(short, long)]
    pub dir: Option<String>,
}

impl CmdList {
    pub fn run(self, settings: HashMap<String, String>) {
        // Setup settings
        let mut src_directory = settings
            .get("source_dir")
            .map_or(DEFAULT_SOURCE_DIR, |v| v)
            .to_string();

        if let Some(sd) = self.dir {
            src_directory = sd;
        };

        for entry in read_dir(src_directory).unwrap().flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some() && path.extension().unwrap() == "slb" {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                println!("{}", file_name);
            }
        }
    }
}
