use gix::Url;
use serde::{Serialize, Deserialize};

use crate::repository::RepositoryTrait;


#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Collaborator {
    login: String,
}

impl Collaborator {
    pub fn get_login(&self)-> String {
        self.login.clone()
    }
}

//https://docs.github.com/en/rest/collaborators/collaborators?apiVersion=2022-11-28
pub fn get_collaborator_names(url: Url, github_token: String) -> Vec<Collaborator> {
    let bearer_token = format!("Bearer {}", github_token);
    let url = format!(
        "https://api.{}/repos/{}/{}/collaborators",
        url.host().unwrap(),
        url.get_org_name(),
        url.get_repository_name()
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

#[cfg(test)]
mod test{
    use gix::Url;
    use dotenv::dotenv;
    use std::env;

    use super::*;

    #[test]
    fn can_get_collaborators() {
        dotenv().ok();
        let github_token = env::var("github_token").expect("No environment variable found for github_token");
        assert_eq!(
            vec![Collaborator {
                login: "bfrazho".to_string()
            }],
            get_collaborator_names(Url::try_from("git@github.com:bfrazho/gitty.git").unwrap(), github_token)
        )
    }
}