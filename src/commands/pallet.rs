use git2;

pub fn run() {
    let repo = git2::Repository::open(&Path::new(".")).unwrap();
    let statuses = repo.statuses(None).unwrap();

    // TODO: index changes
    println!("Nothing has been loaded onto your current pallet yet!");

    // TODO: unstaged changes to tracked files, deletions of tracked files... do they
    // get listed under the "changes that could be loaded onto your pallet" section?
    // differentiated? how? This just handles brand new untracked files right now...
    let mut untracked = statuses.iter().filter(|e| e.status() == git2::STATUS_WT_NEW).peekable();
    if untracked.is_empty() {
        println!("No changes that could be loaded.");
    } else {
        println!("You could load onto your pallet:");
        for entry in untracked {
            let file = entry.index_to_workdir().unwrap().old_file().path().unwrap();
            println!("    {}", file.display());
        }
    }
}

