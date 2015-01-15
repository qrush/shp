extern crate getopts;
use std::os;

fn print_usage(program: &str, opts: &[getopts::OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", getopts::usage(brief.as_slice(), opts));
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
        "start" => println!("You said start!"),
        _ => print_usage(program.as_slice(), opts),
    }
}
