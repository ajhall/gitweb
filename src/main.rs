extern crate git2;
extern crate open;
extern crate regex;
extern crate structopt;

use regex::Regex;
use std::env;
use std::process::Command;
use structopt::StructOpt;

mod git;
mod logger;

#[derive(Debug, StructOpt)]
// Rename all will use the name of the field
#[structopt(rename_all = "kebab-case")]
pub struct Opt {
    /// Set the branch
    #[structopt(short, long)]
    branch: Option<String>,

    /// Set the browser
    #[structopt(short = "-B", long)]
    browser: Option<String>,

    /// Set the remote
    #[structopt(short, long)]
    remote: Option<String>,

    /// Set the verbosity of the command
    #[structopt(short, long)]
    verbose: bool,
}

/// Function to open the browser using the system shell.
fn open_browser(browser: &String, url: &String) {
    Command::new(browser)
        .arg(url)
        .spawn()
        .expect("failed to execute process");
}

const BROWSER: &str = "BROWSER";

fn main() {
    // Get the command line options
    let opt = Opt::from_args();
    let logger = logger::Logger::new(opt.verbose);

    logger.print("Verbose is active");

    // Check that the user is in a git repository.
    let repo = git::get_repo();

    // Get the branch to show in the browser.
    let branch = match opt.branch {
        Some(branch) => branch,
        None => {
            logger.print("No branch given, getting current one");

            git::get_branch(&repo, &logger)
        }
    };

    let remote_name = &opt.remote.unwrap_or("origin".to_owned());

    logger.print(format!("Getting remote for {}", remote_name).as_str());

    let optional_remote = match repo.find_remote(remote_name) {
        Ok(remote) => remote,
        Err(e) => panic!("failed to get remote {}", e),
    };

    let remote_url = match optional_remote.url() {
        Some(remote) => remote,
        None => panic!("no remote available"),
    };

    let re = Regex::new(r".*@(.*):(.*)\.git").unwrap();
    let caps = re.captures(remote_url).unwrap();

    let domain = caps.get(1).map_or("github.com", |m| m.as_str());
    let repository = caps.get(2).map_or("", |m| m.as_str());

    let url = format!(
        "https://{domain}/{repository}/tree/{branch}",
        domain = domain,
        repository = repository,
        branch = branch
    );

    // Open the browser.
    // If the option is available through the command line, open
    // the given web browser
    if opt.browser.is_some() {
        let option_browser = opt.browser.unwrap();

        logger.print(format!("Browser {} given as option", option_browser).as_str());

        open_browser(&option_browser, &url);
    } else {
        match env::var(BROWSER) {
            // If the environment variable is available, open the web browser.
            Ok(browser) => open_browser(&browser, &url),
            // Else, open the default browser of the system.
            Err(e) => {
                logger.print(format!("{} variable not available : {}", BROWSER, e).as_str());
                logger.print("Opening default browser");

                // Open the default web browser on the current system.
                match open::that(&url) {
                    Ok(res) => println!("{:?}", res),
                    Err(err) => panic!("failed to open the browser : {}", err),
                }
            }
        }
    };
}
