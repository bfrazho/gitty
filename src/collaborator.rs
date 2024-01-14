use std::fmt::Display;

use serde::{Serialize, Deserialize};

use crate::{user_input_generator::MultiSelectGeneratorTrait, repository::GitRepository, http_agent::HttpProxyAgent};


#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord)]
pub struct Collaborator {
    login: String,
    node_id: String,
}


impl Collaborator {
    #[allow(dead_code)]
    pub fn new(node_id: String, login: String)-> Self{
        Self{node_id, login}
    }

    pub fn get_id(&self)-> &str {
        &self.node_id
    }
}

impl Display for Collaborator{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.login)
    }
}

impl GitRepository {
    fn build_get_collaborators_query(&self)-> String{
        format!(r#"
           {base}/repos/{org}/{repo}/collaborators
        "#, base=self.get_base_rest_url(), org=self.get_org_name(), repo=self.get_repository_name()).replace("\n", "")
        
    }

    //https://docs.github.com/en/rest/collaborators/collaborators?apiVersion=2022-11-28
    pub fn get_collaborators(&self, http_agent: &HttpProxyAgent) -> Vec<Collaborator> {
        let collaborator_query = self.build_get_collaborators_query();

        let mut collaborators = match http_agent.get(&collaborator_query)
        .set("Authorization",&self.get_bearer_token_string())
        .set("X-GitHub-Api-Version", "2022-11-28")
        .query("permission", "admin")
        .query("per_page", "100")
        .call()
        {
            Ok(response) => {
                let string_response = &response.into_string().unwrap();
                serde_json::from_str::<Vec<Collaborator>>(string_response)
                .expect("failed to deserialize")
            },
            Err(error) => panic!("{}", error),
        };
        collaborators.sort();
        collaborators
    }
}



pub fn ask_who_they_are_working_with(user_input_generator: &mut dyn MultiSelectGeneratorTrait<Collaborator>, collaborators: Vec<Collaborator>)-> Vec<Collaborator> {

    user_input_generator.get_multiselect_input(
        "Select your fellow collaborators", 
        collaborators
    ).unwrap()
}

#[cfg(test)]
mod test{
    use gix::Url;
    use dotenv::dotenv;
    use std::env;

    use crate::user_input_generator::testing::MockMultiSelectGenerator;

    use super::*;

    #[test]
    fn can_get_collaborators() {
        dotenv().ok();
        let github_token = env::var("github_token").expect("No environment variable found for github_token");
        let http_agent = HttpProxyAgent::new_with_proxy("");
        assert_eq!(
            vec![Collaborator {
                node_id: "MDQ6VXNlcjMxMzkxNTc5".to_string(),
                login: "bfrazho".to_string()
            }],
            GitRepository::new(github_token, Url::try_from("git@github.com:bfrazho/gitty.git").unwrap(), "".to_string()).get_collaborators(&http_agent)
        )
    }
 
    #[test]
    fn user_can_select_who_they_are_pairing_with() {

        let mut user_input_generator = MockMultiSelectGenerator::new(
            vec![vec![Collaborator::new("id 1".to_string(), "User 1".to_string())]],
        );
        let collaborators = vec![
            Collaborator::new("id 1".to_string(), "User 1".to_string()), 
            Collaborator::new("id 2".to_string(), "User 2".to_string())
        ];
        
        assert_eq!(vec![Collaborator::new("id 1".to_string(), "User 1".to_string())], 
            ask_who_they_are_working_with(&mut user_input_generator, collaborators));
    }
}