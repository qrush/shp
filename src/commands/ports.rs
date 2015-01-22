use git2;

pub fn run() {
    let repo = git2::Repository::open(&Path::new(".")).unwrap();
    let remotes = repo.remotes().unwrap();
    match remotes.len() {
        0 => panic!("No other ports have been added yet! Please add one with 'shp port add [address]'"),
        _ => for remote in remotes.iter() { println!("{}", remote.unwrap()) },
    };
}

