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
        format!("https://api.{}/graphql",self.get_base_git_url())
    }

    pub fn get_org_name(&self)-> String{
        return self.url.get_org_name()
    }
    pub fn get_repository_name(&self)-> String{
        return self.url.get_repository_name()
    }
    pub fn get_base_git_url(&self)-> &str{
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
    fn can_create_git_repository() {
        let repository = GitRepository::new(
            "the token".to_string(), 
        Url::try_from("git@github.com:bfrazho/gitty.git").unwrap());

        assert_eq!("the token", repository.get_token());
        assert_eq!("bfrazho".to_string(), repository.get_org_name());
        assert_eq!("github.com", repository.get_base_git_url());
        assert_eq!("gitty", repository.get_repository_name());
    }
}
