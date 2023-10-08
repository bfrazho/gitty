use collaborator::{get_collaborators, ask_who_they_are_working_with, Collaborator};
use repository::get_repository_url;
use token_retriever::get_github_token_and_prompt_if_not_found;
use user_input_generator::InquireUserInputGenerator;

use crate::repository::RepositoryTrait;
mod collaborator;
mod repository;
mod token_retriever;
mod user_input_generator;

fn main() {
    let url = get_repository_url();
    println!("Org: {}, Repo: {}", url.get_org_name(), url.get_repository_name());
    let mut user_input_generator = InquireUserInputGenerator::new();
    let github_token = get_github_token_and_prompt_if_not_found(&mut user_input_generator);
    let collaborators: Vec<Collaborator> = get_collaborators(url, github_token);
    let selections = ask_who_they_are_working_with(&mut user_input_generator, collaborators);
    print!("Pairing with: ");
    selections.iter().for_each(|selection| print!("{},", selection));
    println!("");
}

#[cfg(test)]
mod test {

}
