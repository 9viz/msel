/*
 * msel - select multiple items as arguments/from stdin (todo)
 * and echo all the selections to stdout in
 * a space separated list.
 *
 * made by viz (https://github.com/vizs)
 *
 * depends: termion
 *
 * licensed under BSD 2-Clause "Simplified" License
 */

const DELIM: &str = " ";
const USAGE: &str = r#"usage: msel [OPTIONS] text...
options:
    -h              print this help message and exit
    -d              change the output delimiter (todo)"#;

use msel;
use std::io::{stdout, Write};
use std::{env, process::exit};

struct Config {
    delim: String,
    items: Vec<String>,
}

fn usage() {
    eprintln!("{}", USAGE);
    exit(0);
}

fn parse_args() -> Config {
    let argv: Vec<String> = env::args().collect();

    if argv.len() == 1 {
        usage();
    }

    let possible_flags = ["-h"];
    let mut items: Vec<String> = vec![];

    for (_n, a) in argv[1..].iter().enumerate() {
        if a == "-h" {
            usage();
        } else if !possible_flags.contains(&a.as_str()) {
            items.push(a.to_string());
        }
    }

    Config { delim: DELIM.to_string(), items: items }
}

fn main() {
    let config = parse_args();
    let mut items = msel::Items::new(&config.items);
    msel::ui::run(&mut items);

    for (n, i) in items.sel_items.iter().enumerate() {
        print!("{}", i);
        if n != items.sel_items.len() { print!("{}", config.delim); }
        stdout().flush()
                .unwrap();
    }
}
