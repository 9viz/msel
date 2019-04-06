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
    -d              change the output delimiter"#;

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
    let mut delim: String = String::from(DELIM);

    if argv.len() == 1 {
        usage();
    }

    let mut items: Vec<String> = vec![];
    let mut n = 1;

    while n != argv.len() {
        let a = argv[n].to_string();
        match a.as_str() {
            "-d" => { delim = argv[n+1].to_string(); n += 1; },
            "-h" => usage(),
            _ => items.push(a.to_string()),
        }
        n += 1;
    }

    Config { delim: delim, items: items }
}

fn main() {
    let config = parse_args();
    let mut items = msel::Items::new(&config.items);
    msel::ui::run(&mut items);

    for (n, i) in items.sel_items.iter().enumerate() {
        print!("{}", i);
        if n != items.sel_items.len() - 1 { print!("{}", config.delim); }
        stdout().flush()
                .unwrap();
    }
}
