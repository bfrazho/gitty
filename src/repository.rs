use gix::Url;

pub trait RepositoryTrait {
    fn get_repository_name(&self) -> String;
    fn get_org_name(&self) -> String;
}
impl RepositoryTrait for Url {
    fn get_repository_name(&self) -> String {
        let path = self.path.to_string();
        let split_string: Vec<&str> = path.split("/").collect();
        let string_with_git = split_string[1];
        string_with_git[..(string_with_git.len() - 4)].to_string()
    }

    fn get_org_name(&self) -> String {
        let path = self.path.to_string();
        let split_string: Vec<&str> = path.split("/").collect();
        split_string[0].to_string()
    }
}

pub fn get_repository_url() -> Url {
    let repo = gix::discover(".").unwrap();
    let remote = repo
        .find_default_remote(gix::remote::Direction::Fetch)
        .unwrap()
        .unwrap();
    remote
        .url(gix::remote::Direction::Fetch)
        .unwrap()
        .to_owned()
}

pub struct GitRepository {
    token: String,
    url: Url
}

impl GitRepository{
    pub fn new(token: String, url: Url)-> Self {
        Self{token, url}
    }
    pub fn get_token(&self)-> &str{
        return &self.token
    }
    pub fn get_bearer_token_string(&self)-> String{
        format!("Bearer {}", self.get_token())
    }
    pub fn get_graphql_url(&self)-> String{
        let base_git_url = self.get_host();
        if base_git_url == "github.com"{
            format!("https://api.{}/graphql",base_git_url)
        } else {
            format!("https://{}/api/graphql",base_git_url)
        }
    }

    pub fn get_org_name(&self)-> String{
        return self.url.get_org_name()
    }
    pub fn get_repository_name(&self)-> String{
        return self.url.get_repository_name()
    }

    pub fn get_base_rest_url(&self)-> String {
        let host = self.get_host();
        if host == "github.com" {
            return format!("https://api.{}", host)
        } else {
            return format!("https://{}/api/v3", host)
        }
    }
    pub fn get_host(&self)-> &str{
        return self.url.host().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retrieve_repo_name_from_url() {
        assert_eq!(
            "gitty",
            Url::try_from("git@github.com:bfrazho/gitty.git")
                .unwrap()
                .get_repository_name()
        );
    }
    #[test]
    fn retrieve_org_name_from_url() {
        assert_eq!(
            "bfrazho",
            Url::try_from("git@github.com:bfrazho/gitty.git")
                .unwrap()
                .get_org_name()
        );
    }

    #[test]
    fn get_repository_url_gets_current() {
        assert_eq!(
            "git@github.com:bfrazho/gitty.git".to_string(),
            get_repository_url().to_bstring()
        )
    }

    #[test]
    fn can_get_graphql_url_for_standard_git(){
        let repository = GitRepository::new(
            "the token".to_string(), 
        Url::try_from("git@github.com:bfrazho/gitty.git").unwrap());
        assert_eq!("https://api.github.com/graphql", repository.get_graphql_url());
    }

    #[test]
    fn can_get_graphql_url_for_enterprise_git(){
        let repository = GitRepository::new(
            "the token".to_string(), 
        Url::try_from("git@github.some-business.com:bfrazho/gitty.git").unwrap());
        assert_eq!("https://github.some-business.com/api/graphql", repository.get_graphql_url());
    }

    #[test]
    fn can_get_rest_url_for_standard_git(){
        let repository = GitRepository::new(
            "the token".to_string(), 
        Url::try_from("git@github.com:bfrazho/gitty.git").unwrap());
        assert_eq!("https://api.github.com", repository.get_base_rest_url());
    }

    #[test]
    fn can_get_rest_url_for_enterprise_git(){
        let repository = GitRepository::new(
            "the token".to_string(), 
        Url::try_from("git@github.some-business.com:bfrazho/gitty.git").unwrap());
        assert_eq!("https://github.some-business.com/api/v3", repository.get_base_rest_url());
    }

    #[test]
    fn can_create_git_repository() {
        let repository = GitRepository::new(
            "the token".to_string(), 
        Url::try_from("git@github.com:bfrazho/gitty.git").unwrap());

        assert_eq!("the token", repository.get_token());
        assert_eq!("bfrazho".to_string(), repository.get_org_name());
        assert_eq!("github.com", repository.get_host());
        assert_eq!("gitty", repository.get_repository_name());
    }
}
