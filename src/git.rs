use crate::logger::Logger;
use git2::{ErrorCode, Repository};

pub fn get_repo() -> Repository {
    return match Repository::discover(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
}

pub fn get_branch(repo: &Repository, logger: &Logger) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => panic!("failed to get head ref {}", e),
    };

    let head = head.as_ref().and_then(|h| h.shorthand());
    logger.print(
        format!(
            "# On branch {}",
            head.unwrap_or("Not currently on any branch")
        )
        .as_str(),
    );

    String::from(head.unwrap_or("master"))
}
