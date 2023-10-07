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
}
