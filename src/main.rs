use std::thread;
use std::time::Duration;
use git2::{Branch, FetchOptions, RemoteCallbacks, Repository, Signature};

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let branch_name = "wip";

    loop {
        // Check out "wip" branch, created if needed
        let branch_exists = repo.find_branch(branch_name, git2::BranchType::Local).is_ok();

        let mut branch = if !branch_exists {
            let head = repo.head().unwrap();
            let target = repo.find_commit(head.target().unwrap()).unwrap();
            repo.branch(branch_name, &target, false).unwrap()
        } else {
            let branch_ref = repo.find_branch(branch_name, git2::BranchType::Local).unwrap().into_reference();
            Branch::wrap(branch_ref)
        };

        let obj = repo.find_object(branch.get().target().unwrap(), None).unwrap();
        repo.checkout_tree(&obj, None).unwrap();
        repo.set_head(branch.name().unwrap().unwrap()).unwrap();

        // Make sure branch is set to upstream remote
        let mut remote = repo.find_remote("origin").unwrap();
        let upstream = format!("refs/heads/{}", branch_name);
        branch.set_upstream(Some(&upstream)).unwrap();

        // Add all untracked files
        let mut index = repo.index().unwrap();
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None).unwrap();
        index.write().unwrap();

        // Commit all files
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let head = repo.head().unwrap();
        let parent_commit = repo.find_commit(head.target().unwrap()).unwrap();
        let signature = Signature::now("gak", "gak@gak0.com").unwrap();
        let message = "Sync wip branch";
        repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[&parent_commit]).unwrap();

        // Pull/rebase from wip remote
        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.credentials(|_url, username_from_url, _allowed_types| {
            git2::Cred::ssh_key_from_agent(username_from_url.unwrap())
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(create_remote_callbacks());

        remote.fetch(&[branch_name], Some(&mut fetch_options), None).unwrap();
        let remote_ref = format!("refs/remotes/origin/{}", branch_name);

        let remote_commit = repo.find_commit(repo.refname_to_id(&remote_ref).unwrap()).unwrap();
        let local_annotated = repo.find_annotated_commit(parent_commit.id()).unwrap();
        let remote_annotated = repo.find_annotated_commit(remote_commit.id()).unwrap();
        let rebase = repo.rebase(None, Some(&local_annotated), Some(&remote_annotated), None).unwrap();

        if rebase.len() > 0 {
            panic!("Conflicts detected.");
        }

        // Push
        let mut push_options = git2::PushOptions::new();
        push_options.remote_callbacks(create_remote_callbacks());
        let refspec = format!("refs/heads/{}:{}", branch_name, branch_name);
        remote.push(&[&refspec], Some(&mut push_options)).unwrap();

        // Sleep for a minute
        thread::sleep(Duration::from_secs(60));
    }
}

fn create_remote_callbacks() -> RemoteCallbacks<'static> {
    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(|_url, username_from_url, _allowed_types| {
        git2::Cred::ssh_key_from_agent(username_from_url.unwrap())
    });
    remote_callbacks
}