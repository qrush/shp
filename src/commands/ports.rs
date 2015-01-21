extern crate git2;

pub fn run() {
    let repo = git2::Repository::open(&Path::new(".")).unwrap();
    let remotes = repo.remotes().unwrap();
    match remotes.len() {
        0 => println!("Argh! No other ports be known to us yet!"),
        _ => for remote in remotes.iter() { println!("{}", remote.unwrap()) },
    };
}

