#![allow(unstable)]
#![feature(plugin)]
#[plugin] #[no_link] extern crate regex_macros;

extern crate regex;
extern crate getopts;
extern crate git2;

use std::os;

mod commands;

fn print_usage(program: &str, opts: &[getopts::OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", getopts::usage(brief.as_slice(), opts));
}

fn humanish_url_part(url: &str) -> &str {
    let pieces: Vec<&str> = url.split('/').collect();
    let last_part = pieces.last().unwrap_or(&url);
    let name_pieces: Vec<&str> = last_part.split('.').collect();
    *name_pieces.first().unwrap_or(last_part)
}

fn clone_port(url: &String) {
    // TODO: support cloning into a directory by taking more args
    match git2::Repository::clone(url.as_slice(), &Path::new(humanish_url_part(url.as_slice()))) {
        Ok(_)    => {},
        Err(msg) => println!("Could not clone a port: {}", msg),
    };
}

fn create_new_port(p: &str) {
    let path = Path::new(p);
    let opts = git2::RepositoryInitOptions::new();
    match git2::Repository::init_opts(&path, &opts) {
        Ok(_)    => {},
        Err(msg) => println!("Could not create a port: {}", msg),
    };
}

fn initialize_port(args: &[String]) {
    match args.first() {
        Some(p) => {
            match regex!(r"(://|@)").is_match(p.as_slice()) {
                true  => clone_port(p),
                false => create_new_port(p.as_slice()),
            };
        },
        None => create_new_port(".".as_slice()),
    }
}

fn main() {
    let args = os::args();
    let ref program = args[0];

    let opts = &[
        getopts::optflag("h", "help", "Show this message"),
        getopts::optflag("v", "version", "Show version"),
    ];

    let opt_matches = match getopts::getopts(args.tail(), opts) {
        Ok(m)  => m,
        Err(f) => { panic!(f.to_string()) }
    };

    let command = if !opt_matches.free.is_empty() {
        opt_matches.free[0].clone()
    } else {
        print_usage(program.as_slice(), opts);
        return;
    };


    match command.as_slice() {
        "help"  => print_usage(program.as_slice(), opts),
        "start" => initialize_port(opt_matches.free.tail()),
        "ports" => commands::ports::print(),
        _ => print_usage(program.as_slice(), opts),
    }
}
