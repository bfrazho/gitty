use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

use crate::{repository::GitRepository, collaborator::Collaborator};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize,Clone)]
pub struct User {
    id: Option<String>
}
impl User{
    fn get_id(&self)->Option<String>{
        self.id.clone()
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Author {
    user: Option<User>
}
impl Author{
    fn get_user(&self)->Option<User>{
        self.user.clone()
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Commit {
    id: String,
    message: String,
    author: Author
}
impl Commit{
    fn get_author_id(&self)->Option<String>{
        match self.author.get_user(){
            Some(user)=>user.id,
            None=> None
        }
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
struct ThisRepository {
    object: ThisObject
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
struct ThisObject {
    history: History
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
struct History {
    nodes: Vec<Commit>
}

fn filter_any_commits_that_do_not_match_collaborators(commits: Vec<Commit>, collaborators: &Vec<Collaborator>)->Vec<Commit>{
    let collaborator_ids: Vec<String> = collaborators.iter().map(|collaborator| collaborator.get_id()).collect();
    commits.into_iter()
        .filter(|commit| commit.get_author_id().is_some() && collaborator_ids.contains(&commit.get_author_id().unwrap()))
        .collect()
} 



impl GitRepository{
    fn build_get_commits_after_timestamp_query(&self, timestamp: DateTime<Local>)-> String{
        format!(r#"
            {{"query": "query {{
                    repository(owner: \"{org}\", name:\"{repo}\") {{
                        object(expression: \"main\") {{
                            ... on Commit {{
                                history(first: 100, since: \"{timestamp}\") {{
                                    nodes {{
                                        id,
                                        message,
                                        author {{
                                            user {{
                                                id
                                            }}
                                        }},
                                    }}
                                }}
                            }}
                        }}
                    }}
                }}"
            }}
        "#, org=self.get_org_name(), repo=self.get_repository_name(), timestamp=timestamp.to_rfc3339()).replace("\n", "")
        
    }

    pub fn get_commits_matching_collaborators_since_timestamp(&self, collaborators: &Vec<Collaborator>, timestamp: DateTime<Local>)-> Vec<Commit> {
        let graphql_query = self.build_get_commits_after_timestamp_query(timestamp);
        let bearer_token = self.get_bearer_token_string();
        let url = self.get_graphql_url();

        let commits = match ureq::post(&url)
            .set("Authorization",&bearer_token)
            .send_string(&graphql_query)
            {
                Ok(response) => serde_json::from_str::<QueryResult>(&response.into_string().unwrap())
                    .expect("failed to deserialize").data.repository.object.history.nodes,
                Err(error) => panic!("{}", error),
            };
        filter_any_commits_that_do_not_match_collaborators(commits, collaborators)
        
    }
}



#[cfg(test)]
mod tests{
    use std::env;
    use dotenv::dotenv;
    use chrono::{Local, NaiveDate};
    use gix::Url;

    use super::*;

    #[test]
    fn can_get_commits_matching_collaborators_since_timestamp() {
        dotenv().ok();
        let collaborators = vec![Collaborator::new("MDQ6VXNlcjMxMzkxNTc5".to_string(), "bfrazho".to_string())];
        let timestamp = NaiveDate::from_ymd_opt(2023, 10, 7).unwrap()
            .and_hms_opt(0, 0, 0).unwrap()
            .and_local_timezone(Local::now().timezone()).unwrap();
        let github_token = env::var("github_token").expect("No environment variable found for github_token");
        
        let repository = GitRepository::new(github_token, Url::try_from("git@github.com:bfrazho/gitty.git").unwrap());
        let commits = repository.get_commits_matching_collaborators_since_timestamp(&collaborators, timestamp);
        println!("{:?}", commits);
        assert!(
            commits.contains(&Commit{
                id: "C_kwDOKaBtAtoAKDAwMjk5NDgxMzY3Zjk5ZGY0ZDNlNGE2YWE2MzhmMWEyMjhiM2EyNmE".to_string(),
                message: "can retrieve commits based on timestamp".to_string(),
                author: Author { user: Some(User{id: Some("MDQ6VXNlcjMxMzkxNTc5".to_string())})}
            }));
    }

}