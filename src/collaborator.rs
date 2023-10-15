use std::fmt::Display;

use serde::{Serialize, Deserialize};

use crate::{user_input_generator::MultiSelectGeneratorTrait, repository::GitRepository};


#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord)]
pub struct Collaborator {
    login: String,
    id: String,
}


impl Collaborator {
    #[allow(dead_code)]
    pub fn new(node_id: String, login: String)-> Self{
        Self{id: node_id, login}
    }

    pub fn get_id(&self)-> &str {
        &self.id
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
struct QueryResult {
    data: Data
}
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
struct Data {
    repository: ThisRepository
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
struct Node {
    nodes: Vec<Collaborator>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
struct ThisRepository {
    collaborators: Node
}

impl Display for Collaborator{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.login)
    }
}

impl GitRepository {
    fn build_get_collaborators_query(&self)-> String{
        format!(r#"
            {{"query": "query {{
                    repository(owner: \"{org}\", name:\"{repo}\") {{
                        collaborators{{
                            nodes{{
                              login,
                              id,  
                            }}
                          }}
                    }}
                }}"
            }}
        "#, org=self.get_org_name(), repo=self.get_repository_name()).replace("\n", "")
        
    }

    //https://docs.github.com/en/rest/collaborators/collaborators?apiVersion=2022-11-28
    pub fn get_collaborators(&self) -> Vec<Collaborator> {
        let graphql_query = self.build_get_collaborators_query();

        let mut collaborators = match ureq::post(&self.get_graphql_url())
        .set("Authorization",&self.get_bearer_token_string())
        .send_string(&graphql_query)
        {
            Ok(response) => {
                let string_response = &response.into_string().unwrap();
                serde_json::from_str::<QueryResult>(string_response)
                .expect("failed to deserialize").data.repository.collaborators.nodes
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
        assert_eq!(
            vec![Collaborator {
                id: "MDQ6VXNlcjMxMzkxNTc5".to_string(),
                login: "bfrazho".to_string()
            }],
            GitRepository::new(github_token, Url::try_from("git@github.com:bfrazho/gitty.git").unwrap()).get_collaborators()
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