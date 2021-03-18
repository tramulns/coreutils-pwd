use clap::{App, Arg};
use std::env;
use std::fs;

static OPTION_LOGICAL: &str = "logical";
static OPTION_PHYSICAL: &str = "physical";

fn main() {
    let matches = App::new("pwd")
        .arg(
            Arg::with_name(OPTION_LOGICAL)
                .short("L")
                .help("print the value of $PWD if it names the current working directory"),
        )
        .arg(
            Arg::with_name(OPTION_PHYSICAL)
                .short("P")
                .help("print the physical directory, without any symbolic links"),
        )
        .get_matches_from(std::env::args_os().into_iter());

    match env::current_dir() {
        Ok(path) => {
            if matches.is_present(OPTION_LOGICAL) {
                println!("{}", path.display());
            } else {
                match fs::canonicalize(&path) {
                    Ok(physical_path) => println!("{}", physical_path.display()),
                    Err(e) => {
                        eprintln!("failed to get absolute path {}", e);
                        std::process::exit(1)
                    }
                };
            }
        }
        Err(e) => {
            eprintln!("failed to get current directory {}", e);
            std::process::exit(1)
        }
    };

    std::process::exit(0)
}
