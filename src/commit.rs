use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

use crate::repository::GitRepository;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct User {
    login: Option<String>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Author {
    name: Option<String>,
    email: Option<String>,
    user: Option<User>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Commit {
    id: String,
    message: String,
    author: Author
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


impl GitRepository{
    pub fn get_commits_matching_collaborators_since_timestamp(&self, collaborators: Vec<String>, timestamp: DateTime<Local>)-> Vec<Commit> {
        let graphql_query = format!(r#"
        {{"query": "query {{
                repository(owner: \"{org}\", name:\"{repo}\") {{
                    object(expression: \"main\") {{
                        ... on Commit {{
                            history(first: 100, since: \"{timestamp}\") {{
                                nodes {{
                                    id,
                                    message,
                                      author {{
                                        name,
                                        email,
                                        user {{
                                            login
                                        }}
                                    }},
                                }}
                            }}
                        }}
                    }}
                }}
            }}"
        }}
        "#, org=self.get_org_name(), repo=self.get_repository_name(), timestamp=timestamp.to_rfc3339()).replace("\n", "");
        let bearer_token = format!("Bearer {}", self.get_token());
        let url = format!(
            "https://api.{}/graphql",
            self.get_base_git_url());

        match ureq::post(&url)
            .set("Authorization",&bearer_token)
            .send_string(&graphql_query)
            {
                Ok(response) => serde_json::from_str::<QueryResult>(&response.into_string().unwrap())
                    .expect("failed to deserialize").data.repository.object.history.nodes,
                Err(error) => panic!("{}", error),
            }
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
        let collaborators = vec!["bfrazho".to_string()];
        let timestamp = NaiveDate::from_ymd_opt(2023, 10, 7).unwrap()
            .and_hms_opt(0, 0, 0).unwrap()
            .and_local_timezone(Local::now().timezone()).unwrap();
        let github_token = env::var("github_token").expect("No environment variable found for github_token");
        
        let repository = GitRepository::new(github_token, Url::try_from("git@github.com:bfrazho/gitty.git").unwrap());
        let commits = repository.get_commits_matching_collaborators_since_timestamp(collaborators, timestamp);
        println!("{:?}", commits);
        assert!(
            commits.contains(&Commit{
                id: "C_kwDOKaBtAtoAKGY4MTc2Njg2Njc0YzJhMDFhMTRmOTgwMGUyMWY3YTQ4NzBiOGJmNWM".to_string(),
                message: "moved logic for selecting collaborators into collaborators file".to_string(),
                author: Author { name: Some("Brian".to_string()), email: Some("bfrazho".to_string()), user:  None}
            }));
    }

}