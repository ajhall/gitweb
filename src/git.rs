use crate::logger::Logger;
use git2::{Error, ErrorCode, Repository};

/// Get the current repository.
pub fn get_repo() -> Result<Repository, Error> {
    const CURRENT_WORKING_DIRECTORY: &str = ".";

    Repository::discover(CURRENT_WORKING_DIRECTORY)
}

// Get the current branch or return master.
pub fn get_branch(repo: &Repository, logger: &Logger) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => panic!("failed to get head ref {}", e),
    };

    let head = head.as_ref().and_then(|h| h.shorthand());
    logger.verbose_print(
        format!(
            "# On branch {}",
            head.unwrap_or("Not currently on any branch")
        )
        .as_str(),
    );

    String::from(head.unwrap_or("master"))
}
