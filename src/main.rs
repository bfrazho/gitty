use collaborator::{ask_who_they_are_working_with, Collaborator};
use repository::{get_repository_url, GitRepository};
use token_retriever::get_github_token_and_prompt_if_not_found;
use user_input_generator::{InquireUserInputGenerator, UserInputGeneratorTrait};

use crate::repository::RepositoryTrait;
mod collaborator;
mod repository;
mod token_retriever;
mod user_input_generator;
mod commit;


fn create_git_repository(user_input_generator: &mut dyn UserInputGeneratorTrait)-> GitRepository {
    let url = get_repository_url();
    println!("Org: {}, Repo: {}", url.get_org_name(), url.get_repository_name());
    let github_token = get_github_token_and_prompt_if_not_found(user_input_generator);
    GitRepository::new(github_token, url)
}

fn main() {
    let mut user_input_generator = InquireUserInputGenerator::new();

    let repository = create_git_repository(&mut user_input_generator);
    let collaborators: Vec<Collaborator> = repository.get_collaborators();
    let selections = ask_who_they_are_working_with(&mut user_input_generator, collaborators);
    print!("Pairing with: ");
    selections.iter().for_each(|selection| print!("{},", selection));
    println!("");
    

    //poll for changes made by people in collaborator list
    //hopefully can check for changes since last timestamp
    //time once every 5 minutes or so
}

#[cfg(test)]
mod test {

}
