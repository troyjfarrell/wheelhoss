//! Executable for the fileslist include-python-source-files command
use std::env;
use std::path::Path;
use std::process;

use wheelhoss::error::Error;
use wheelhoss::fileslist::FilesList;

const DEFAULT_FILESLIST_PATH: &str = "./.sandstorm/sandstorm-files.list";

fn main() -> Result<(), Error> {
    let mut args = env::args();
    if args.len() < 3 {
        let arg_path = match args.nth(1) {
            Some(path) => path,
            None => DEFAULT_FILESLIST_PATH.to_string(),
        };
        let path = Path::new(&arg_path);
        let mut fileslist = FilesList::new(path);
        match fileslist.include_python_source_files() {
            Ok(included) => {
                println!("{:?}", included);
            }
            Err(e) => {
                eprintln!("{}", e);
                process::exit(2);
            }
        }
    } else {
        usage()?;
        process::exit(1);
    }
    Ok(())
}

fn usage() -> Result<(), Error> {
    let current_exe_pathbuf = env::current_exe()?;
    let file_name = match current_exe_pathbuf.file_name() {
        Some(file_name) => match file_name.to_str() {
            Some(name) => name,
            None => env!("CARGO_BIN_NAME"),
        },
        None => env!("CARGO_BIN_NAME"),
    };
    println!("{} [fileslist_path]", file_name);
    println!();
    println!(
        "\tfileslist_path\t\tdefault: \"{}\"",
        DEFAULT_FILESLIST_PATH
    );
    Ok(())
}
