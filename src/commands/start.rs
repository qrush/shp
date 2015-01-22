extern crate git2;

use std::cell::RefCell;
use std::io::stdio;

struct State {
    progress: Option<git2::Progress<'static>>,
    total: usize,
    current: usize,
    path: Path,
    newline: bool,
}

pub fn run(args: &[String]) {
    match args.first() {
        Some(p) => {
            match regex!(r"(://|@)").is_match(p.as_slice()) {
                true  => start_port(p),
                false => create_new_port(p.as_slice()),
            };
        },
        None => create_new_port(".".as_slice()),
    }
}

fn start_port(url: &String) {
    // TODO: support cloning into a directory by taking more args

    let state = RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: Path::new("."),
        newline: false,
    });

    let mut cb = git2::RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        print(&mut *state);
        true
    });

    cb.credentials(|url, username_from_url, allowed_types| {
        println!("url {}", url);
        // TODO obviously dont hardcode this, need to look up more stuff from Cred's
        git2::Cred::ssh_key("git", Some(&Path::new("/Users/qrush/.ssh/id_rsa.pub")), &Path::new("/Users/qrush/.ssh/id_rsa"), None)
    });

    match git2::build::RepoBuilder::new().remote_callbacks(cb)
        .clone(url.as_slice(), &Path::new(humanish_url_part(url.as_slice()))) {
        Ok(_)    => {},
        Err(msg) => println!("Could not start a port: {}", msg),
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

fn humanish_url_part(url: &str) -> &str {
    let pieces: Vec<&str> = url.split('/').collect();
    let last_part = pieces.last().unwrap_or(&url);
    let name_pieces: Vec<&str> = last_part.split('.').collect();
    *name_pieces.first().unwrap_or(last_part)
}

// Very much copied from https://github.com/alexcrichton/git2-rs
fn print(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() && false {
        if !state.newline {
            println!("");
            state.newline = true;
        }
        print!("Resolving deltas {}/{}\r", stats.indexed_deltas(),
               stats.total_deltas());
    } else {
        print!("net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
                /  chk {:3}% ({:4}/{:4}) {}\r",
               network_pct, kbytes, stats.received_objects(),
               stats.total_objects(),
               index_pct, stats.indexed_objects(), stats.total_objects(),
               co_pct, state.current, state.total, state.path.display());
    }
    stdio::flush();
}
