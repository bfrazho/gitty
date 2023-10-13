use std::{env::current_exe, fs, path::PathBuf};

use crate::user_input_generator::TextInputGeneratorTrait;





fn get_github_token_path()-> PathBuf {
    let mut path_to_github_token = current_exe().unwrap().parent().unwrap().to_owned();
    path_to_github_token.push("github_token");
    path_to_github_token
}

pub fn get_github_token_and_prompt_if_not_found(
    user_input_generator: &mut dyn TextInputGeneratorTrait,
) -> String {
    let path_to_github_token = get_github_token_path();
    match fs::read_to_string(path_to_github_token.clone()) {
        Ok(github_token) => github_token,
        Err(_) => {
            let github_token = user_input_generator
                .get_text_input("Please enter your github token")
                .unwrap();
            fs::write(path_to_github_token, github_token.clone()).expect("failed to write token to file");
            github_token
        }
    }
}


#[cfg(test)]
mod test {
    use std::fs;

    use serial_test::serial;

    use crate::{token_retriever::get_github_token_path, user_input_generator::testing::MockTextInputGenerator};

    use super::get_github_token_and_prompt_if_not_found;


    #[test]
    #[serial]
    fn handle_when_token_not_saved() {
        let path_to_github_token = get_github_token_path();
        fs::remove_file(path_to_github_token.clone()).unwrap_or_default();

        let mut user_input_generator =
            MockTextInputGenerator::new(vec!["github_token".to_string()]);


        let github_token = get_github_token_and_prompt_if_not_found(&mut user_input_generator);

        assert_eq!(
            "github_token".to_string(),
            github_token
        );
        assert_eq!(
            "github_token",
            fs::read_to_string(path_to_github_token.clone()).unwrap()
        );
        fs::remove_file(path_to_github_token).expect("failed to delete token");
    }

    #[test]
    #[serial]
    fn handle_when_token_already_exists() {
        let path_to_github_token = get_github_token_path();
        fs::remove_file(path_to_github_token.clone()).unwrap_or_default();
        fs::write(path_to_github_token.clone(), "existing_github_token").expect("failed to create token");
        let mut user_input_generator =
            MockTextInputGenerator::new(Vec::new());
            
        let github_token = get_github_token_and_prompt_if_not_found(&mut user_input_generator);

        assert_eq!(
            "existing_github_token".to_string(),
            github_token
        );
        assert_eq!(
            "existing_github_token",
            fs::read_to_string(path_to_github_token.clone()).unwrap()
        );
        fs::remove_file(path_to_github_token).expect("failed to delete token");
    }
}
