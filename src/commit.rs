use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

use crate::{repository::GitRepository, collaborator::Collaborator};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize,Clone)]
pub struct User {
    id: Option<String>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Author {
    user: Option<User>
}
impl Author{
    fn get_user(&self)->Option<&User>{
        self.user.as_ref()
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Commit {
    oid: String,
    message: String,
    author: Author
}
impl Commit{
    fn get_author_id(&self)->Option<&String>{
        match self.author.get_user(){
            Some(user)=>user.id.as_ref(),
            None=> None
        }
    }
    fn get_id(&self)->&str{
        &self.oid
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
    let collaborator_ids: Vec<&str> = collaborators.iter().map(|collaborator| collaborator.get_id()).collect();
    commits.into_iter()
        .filter(|commit| commit.get_author_id().is_some() && collaborator_ids.contains(&commit.get_author_id().as_ref().unwrap().as_str()))
        .collect()
} 



impl GitRepository{
    fn build_get_commits_after_timestamp_query(&self, timestamp: DateTime<Local>)-> String{
        format!(r#"
            {{"query": "query {{
                    repository(owner: \"{org}\", name:\"{repo}\") {{
                        object(expression: \"{main_branch_name}\") {{
                            ... on Commit {{
                                history(first: 100, since: \"{timestamp}\") {{
                                    nodes {{
                                        oid,
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
        "#, org=self.get_org_name(), repo=self.get_repository_name(), main_branch_name=self.get_main_branch_name(), timestamp=timestamp.to_rfc3339()).replace("\n", "")
        
    }

    pub fn get_commits_matching_collaborators_since_timestamp(&self, collaborators: &Vec<Collaborator>, timestamp: DateTime<Local>)-> Vec<Commit> {
        let graphql_query = self.build_get_commits_after_timestamp_query(timestamp);
        let bearer_token = self.get_bearer_token_string();
        let url = self.get_graphql_url();

        let commits = match ureq::post(&url)
            .set("Authorization",&bearer_token)
            .send_string(&graphql_query)
            {
                Ok(response) =>  {
                    let string_response = &response.into_string().unwrap();
                    serde_json::from_str::<QueryResult>(string_response)
                    .expect("failed to deserialize").data.repository.object.history.nodes
                },
                Err(error) => panic!("{}", error),
            };
        filter_any_commits_that_do_not_match_collaborators(commits, collaborators)
        
    }

    pub fn post_comment_on_commit_that_you_approve_it(&self, commit: &Commit){
        let url = format!("{}/repos/{}/{}/commits/{}/comments", self.get_base_rest_url(), self.get_org_name(), self.get_repository_name(),commit.get_id());
        ureq::post(&url)
            .set("Authorization",&self.get_bearer_token_string())
            .set("X-GitHub-Api-Version", "2022-11-28")
            .send_string("{\"body\": \"I approve this\"}").unwrap();
    }
}


#[cfg(test)]
mod tests{
    use std::env;
    use dotenv::dotenv;
    use chrono::{Local, NaiveDate};
    use gix::Url;

    use super::*;
    #[derive(PartialEq, Eq, Debug, Serialize, Deserialize,Clone)]
    struct CommentResponse{
        id: u64,
        node_id: String,
        body: String
    }

    impl CommentResponse{
        fn get_id(&self)->u64{
            self.id.clone()
        }

        fn get_body(&self)->&str{
            &self.body
        }
    }

    impl GitRepository{

        fn get_comments(&self, commit: &Commit)-> Vec<CommentResponse>{
            let comments_url = format!("{}/repos/{}/{}/commits/{}/comments", self.get_base_rest_url(), self.get_org_name(), self.get_repository_name(), commit.get_id());
            match ureq::get(&comments_url)
            .set("Authorization",&self.get_bearer_token_string())
            .set("X-GitHub-Api-Version", "2022-11-28")
        .call()
            {
                Ok(response) => serde_json::from_str::<Vec<CommentResponse>>(&response.into_string().unwrap())
                    .expect("failed to deserialize"),
                Err(error) => panic!("{}", error),
            }
        }
        fn delete_comments(&self, comments: &Vec<CommentResponse>){
            comments.iter().for_each(|comment| {
                ureq::delete(
                    &format!("{}/repos/{}/{}/comments/{}", 
                    &self.get_base_rest_url(), 
                    self.get_org_name(), 
                    self.get_repository_name(), 
                    comment.get_id()
                ))
                .set("Authorization",&self.get_bearer_token_string())
                .set("X-GitHub-Api-Version", "2022-11-28")
                .call().expect("failed to delete comment");
            })
        }
        
    }


    #[test]
    fn can_get_commits_matching_collaborators_since_timestamp() {
        dotenv().ok();
        let collaborators = vec![Collaborator::new("MDQ6VXNlcjMxMzkxNTc5".to_string(), "bfrazho".to_string())];
        let timestamp = NaiveDate::from_ymd_opt(2023, 10, 7).unwrap()
            .and_hms_opt(0, 0, 0).unwrap()
            .and_local_timezone(Local::now().timezone()).unwrap();
        let github_token = env::var("github_token").expect("No environment variable found for github_token");
        
        let repository = {
            let token = github_token;let url = Url::try_from("git@github.com:bfrazho/gitty.git").unwrap();
            GitRepository::new(token, url, "main".to_string())
        };
        let commits = repository.get_commits_matching_collaborators_since_timestamp(&collaborators, timestamp);
        println!("{:?}", commits);
        assert!(
            commits.contains(&Commit{
                oid: "00299481367f99df4d3e4a6aa638f1a228b3a26a".to_string(),
                message: "can retrieve commits based on timestamp".to_string(),
                author: Author { user: Some(User{id: Some("MDQ6VXNlcjMxMzkxNTc5".to_string())})}
            }));
    }
    #[test]
    fn can_add_comment(){
        let commit = Commit{
            oid: "00299481367f99df4d3e4a6aa638f1a228b3a26a".to_string(),
            message: "can retrieve commits based on timestamp".to_string(),
            author: Author { user: Some(User{id: Some("MDQ6VXNlcjMxMzkxNTc5".to_string())})}
        };

        dotenv().ok();
        let github_token = env::var("github_token").expect("No environment variable found for github_token");
        let repository = {
            let token = github_token;let url = Url::try_from("git@github.com:bfrazho/gitty.git").unwrap();
            GitRepository::new(token, url, "main".to_string())
        };


        repository.post_comment_on_commit_that_you_approve_it(&commit);

        let comments = repository.get_comments(&commit);
        assert_eq!("I approve this", comments.get(0).unwrap().get_body());
        
        repository.delete_comments(&comments);
    }

}