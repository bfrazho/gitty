use std::time::Duration;
use colored::Colorize;
use chrono::Local;
use collaborator::{ask_who_they_are_working_with, Collaborator};
use repository::{get_repository_url, GitRepository};
use token_retriever::get_github_token_and_prompt_if_not_found;
use user_input_generator::TextInputGeneratorTrait;

use crate::{repository::RepositoryTrait, user_input_generator::{InquireTextInputGenerator, InquireMultiSelectGenerator}};
mod collaborator;
mod repository;
mod token_retriever;
mod user_input_generator;
mod commit;


fn create_git_repository(user_input_generator: &mut dyn TextInputGeneratorTrait)-> GitRepository {
    let url = get_repository_url();
    println!("Org: {}, Repo: {}", url.get_org_name(), url.get_repository_name());
    let github_token = get_github_token_and_prompt_if_not_found(user_input_generator);
    GitRepository::new(github_token, url)
}

fn main() {
    let mut user_input_generator = InquireTextInputGenerator::new();
    let mut collaborator_input_generator = InquireMultiSelectGenerator::new();

    let repository = create_git_repository(&mut user_input_generator);
    let collaborators: Vec<Collaborator> = repository.get_collaborators();
    let selected_collaborators = ask_who_they_are_working_with(&mut collaborator_input_generator, collaborators);
    let mut timestamp = Local::now();

    print_nyan_cat();

    loop{
        let commits = repository.get_commits_matching_collaborators_since_timestamp(&selected_collaborators, timestamp);
        timestamp = Local::now();
        commits.iter().for_each(|commit| {
            println!("commit: {:?}", commit);
            repository.post_comment_on_commit_that_you_approve_it(commit);
        });
        std::thread::sleep(Duration::new(300, 0))
    }
}


fn print_nyan_cat() {
    println!(r#"
{r        }▄▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄
{r2      }█  ▄▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄  █
{y       }█ █      ▄    ▄       █ █
{y       }█ █   ▄        ▄▀▀▄   █ █▄▀▀▄
{g }▄▀▀▄  █ █        ▄   █   ▀▄▄█▄▀   █
{g }▀▄  ▀▀█ █     ▄      █            █
{b   }▀▀▄▄█ █  ▄        █   ▄█   ▄ ▄█  █
{b2      }█ █        ▄  █ ██ ▄  ▄  ▄ ███
{p      }▄█  ▀▄▄▄▄▄▄▄▄▄▄▄▀▄  ▀▀▀▀▀▀▀ ▄▀
{p2   }▄▀ ▄▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀▀▀▀▀▀▀▀▀▀
{n }   █▄▄▀  █▄▄▀        █▄▄▀ ▀▄▄█
"#,
r ="███████████████████████████".red(),
r2="██████████████████████████".red(),
y ="██████████████████████████".yellow(),
g ="████████████████████".green(),
b ="██████████████████████".cyan(),
b2="██████████████████████████".cyan(),
p ="█████████████████████████".purple(),
p2="███████████████████████".purple(),
n ="                    "
);
}

#[cfg(test)]
mod test {

}

