use std::io::Write;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use rustygit::types::BranchName;

fn main() {
    let branch_name = "wip";
    let branch_name = BranchName::from_str(branch_name).unwrap();

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
        if modified.is_empty() {
            sleep();
            continue;
        }

        println!();
        println!("Modified files: {:?}", modified);
        // Commit all changes (git commit -m "commit message")
        repo.commit_all("wip").unwrap();

        // Pull and rebase
        repo.cmd(&["pull", "--rebase"]).unwrap();

        // Push
        repo.push().unwrap();

        println!("Pushed changes to {}", branch_name.to_string());

        sleep();
    }
}

fn sleep() {
    print!(".");
    std::io::stdout().flush().unwrap();

    thread::sleep(Duration::from_secs(30));
}