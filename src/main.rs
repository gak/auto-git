use std::io::Write;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use clap::Parser;
use rustygit::types::BranchName;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Git branch to work out. Will switch to this branch automatically.
    branch: String,
}

fn main() {
    let args = Args::parse();
    let branch_name = BranchName::from_str(&args.branch).unwrap();

    println!("Auto git running on branch {}", branch_name);

    let repo = rustygit::Repository::new(".");


    let branches = repo.list_branches().unwrap();
    if !branches.contains(&branch_name.to_string()) {
        repo.create_local_branch(&branch_name).unwrap();
    }

    repo.switch_branch(&branch_name).unwrap();

    loop {
        // Add all untracked files (git add .)
        let untracked = repo.list_untracked().unwrap();
        let untracked = untracked.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        repo.add(untracked).unwrap();

        // Check if there are any changes to commit
        let modified = repo.list_modified().unwrap();
        let added = repo.list_added().unwrap();
        let staged = [&modified[..], &added[..]].concat();
        if staged.is_empty() {
            sleep();
            continue;
        }

        println!();
        println!("Modified files: {:?}", staged);
        // Commit all changes (git commit -m "commit message")
        repo.commit_all("wip").unwrap();

        // Pull and rebase
        println!("Pull rebase");
        repo.cmd(&["pull", "--rebase"]).unwrap();

        // Push
        println!("Push");
        repo.cmd(&["push", "-u", "origin", &branch_name.to_string()]).unwrap();

        println!("Pushed changes to {}", branch_name.to_string());

        sleep();
    }
}

fn sleep() {
    print!(".");
    std::io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(30));
}