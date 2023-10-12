use serde::{Serialize, Deserialize};

use crate::{repository::GitRepository, user_input_generator::UserInputGeneratorTrait};

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Collaborator {
    login: String,
}

impl Collaborator {
    pub fn get_login(&self)-> String {
        self.login.clone()
    }
}
impl GitRepository {
    //https://docs.github.com/en/rest/collaborators/collaborators?apiVersion=2022-11-28
    pub fn get_collaborators(&self) -> Vec<Collaborator> {
        let bearer_token = format!("Bearer {}", self.get_token());
        let url = format!(
            "https://api.{}/repos/{}/{}/collaborators",
            self.get_base_git_url(),
            self.get_org_name(),
            self.get_repository_name()
        );
        match ureq::get(&url)
        .set("Authorization",&bearer_token)
        .set("X-GitHub-Api-Version", "2022-11-28")
        .call()
        {
            Ok(response) => serde_json::from_str::<Vec<Collaborator>>(&response.into_string().unwrap())
                .expect("failed to deserialize"),
            Err(error) => panic!("{}", error),
        }
    }
}



pub fn ask_who_they_are_working_with(user_input_generator: &mut dyn UserInputGeneratorTrait, collaborators: Vec<Collaborator>)-> Vec<String> {
    let collaborator_names: Vec<String> = collaborators
        .iter()
        .map(|collborator| collborator.get_login())
        .collect();

    user_input_generator.get_multiselect_input(
        "Select your fellow collaborators", 
        collaborator_names
    ).unwrap()
}

#[cfg(test)]
mod test{
    use gix::Url;
    use dotenv::dotenv;
    use std::env;

    use crate::user_input_generator::testing::MockUserInputGenerator;

    use super::*;

    #[test]
    fn can_get_collaborators() {
        dotenv().ok();
        let github_token = env::var("github_token").expect("No environment variable found for github_token");
        assert_eq!(
            vec![Collaborator {
                login: "bfrazho".to_string()
            }],
            GitRepository::new(github_token, Url::try_from("git@github.com:bfrazho/gitty.git").unwrap()).get_collaborators()
        )
    }

    #[test]
    fn user_can_select_who_they_are_pairing_with() {

        let mut user_input_generator = MockUserInputGenerator::new(
            Vec::new(),
            vec![vec!["User 1".to_string()]],
        );
        let collaborators = vec![
            Collaborator{login: "User 1".to_string()}, 
            Collaborator{login: "User 2".to_string()}
        ];
        
        assert_eq!(vec!["User 1".to_string()], ask_who_they_are_working_with(&mut user_input_generator, collaborators));
    }
}