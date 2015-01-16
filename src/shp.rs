#![allow(unstable)]

extern crate getopts;
extern crate git2;
use std::os;

fn print_usage(program: &str, opts: &[getopts::OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", getopts::usage(brief.as_slice(), opts));
}

fn initialize_port(args: &[String]) {
     let path = match args.first() {
         Some(p) => Path::new(p),
         None    => Path::new(&"."),
     };
     let opts = git2::RepositoryInitOptions::new();
     match git2::Repository::init_opts(&path, &opts) {
         Ok(_) => {},
         Err(msg) => println!("Could not create a port: {}", msg),
     };
}

fn print_remotes() {
     let repo = git2::Repository::open(&Path::new(".")).unwrap();
     let remotes = repo.remotes().unwrap();
     match remotes.len() {
         0 => println!("Argh! No other ports be known to us yet!"),
         _ => for remote in remotes.iter() { println!("{}", remote.unwrap()) },
     };
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
        "ports" => print_remotes(),
        _ => print_usage(program.as_slice(), opts),
    }
}
