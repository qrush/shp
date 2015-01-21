extern crate git2;

pub fn run(args: &[String]) {
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

fn humanish_url_part(url: &str) -> &str {
    let pieces: Vec<&str> = url.split('/').collect();
    let last_part = pieces.last().unwrap_or(&url);
    let name_pieces: Vec<&str> = last_part.split('.').collect();
    *name_pieces.first().unwrap_or(last_part)
}
