use collaborator::get_collaborator_names;
use inquire::MultiSelect;
use repository::get_repository_url;
use token_retriever::get_github_token_and_prompt_if_not_found;
use user_input_generator::InquireUserInputGenerator;
mod collaborator;
mod repository;
mod token_retriever;
mod user_input_generator;

fn main() {
    let url = get_repository_url();
    let mut user_input_generator = InquireUserInputGenerator::new();
    let github_token = get_github_token_and_prompt_if_not_found(&mut user_input_generator);
    let collaborator_names: Vec<String> = get_collaborator_names(url, github_token)
        .iter()
        .map(|collborator| collborator.get_login())
        .collect();

    let selections = MultiSelect::new(
        "Select your fellow collaborators",
        collaborator_names,
    )
    .prompt().unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in selections {
            println!("  {}", selection);
        }
    }
}

#[cfg(test)]
mod test {

}
