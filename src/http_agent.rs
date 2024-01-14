use std::{
    env::current_exe,
    fs,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};

use ureq::{Agent, AgentBuilder, Error, Proxy, Request, Response};

use crate::user_input_generator::TextInputGeneratorTrait;

pub struct HttpRequest {
    use_proxy: Arc<AtomicBool>,
    non_proxy_request: Request,
    proxy_request: Request,
}

impl HttpRequest {
    pub fn set(mut self, header: &str, value: &str) -> Self {
        self.proxy_request = self.proxy_request.set(header, value);
        self.non_proxy_request = self.non_proxy_request.set(header, value);
        self
    }

    pub fn query(mut self, header: &str, value: &str) -> Self {
        self.proxy_request = self.proxy_request.query(header, value);
        self.non_proxy_request = self.non_proxy_request.query(header, value);
        self
    }

    pub fn get_active_request(&self, use_proxy: bool) -> &Request {
        if use_proxy {
            &self.proxy_request
        } else {
            &self.non_proxy_request
        }
    }

    pub fn send_string(&mut self, data: &str) -> Result<Response, Error> {
        let use_proxy = self.use_proxy.load(std::sync::atomic::Ordering::Relaxed);
        let result = self.get_active_request(use_proxy).clone().send_string(data);
        if result.is_ok() {
            result
        } else {
            self.use_proxy
                .store(!use_proxy, std::sync::atomic::Ordering::Relaxed);
            self.get_active_request(!use_proxy)
                .clone()
                .send_string(data)
        }
    }
    pub fn call(&mut self) -> Result<Response, Error> {
        let use_proxy = self.use_proxy.load(std::sync::atomic::Ordering::Relaxed);
        let result = self.get_active_request(use_proxy).clone().call();
        if result.is_ok() {
            result
        } else {
            self.use_proxy
                .store(!use_proxy, std::sync::atomic::Ordering::Relaxed);
            self.get_active_request(!use_proxy).clone().call()
        }
    }
}

pub struct HttpProxyAgent {
    use_proxy: Arc<AtomicBool>,
    non_proxy_agent: Agent,
    proxy_agent: Agent,
}

impl HttpProxyAgent {
    pub fn new(user_input_generator: &mut dyn TextInputGeneratorTrait) -> Self {
        let proxy = get_proxy_and_prompt_if_not_found(user_input_generator);
        HttpProxyAgent {
            use_proxy: Arc::new(AtomicBool::new(false)),
            non_proxy_agent: Agent::new(),
            proxy_agent: AgentBuilder::new()
                .proxy(Proxy::new(proxy).unwrap())
                .build(),
        }
    }

    pub fn new_with_proxy(proxy: &str) -> Self {
        HttpProxyAgent {
            use_proxy: Arc::new(AtomicBool::new(false)),
            non_proxy_agent: Agent::new(),
            proxy_agent: AgentBuilder::new()
                .proxy(Proxy::new(proxy).unwrap())
                .build(),
        }
    }

    pub fn get(&self, path: &str) -> HttpRequest {
        HttpRequest {
            use_proxy: self.use_proxy.clone(),
            proxy_request: self.proxy_agent.get(path),
            non_proxy_request: self.non_proxy_agent.get(path),
        }
    }

    pub fn post(&self, path: &str) -> HttpRequest {
        HttpRequest {
            use_proxy: self.use_proxy.clone(),
            proxy_request: self.proxy_agent.post(path),
            non_proxy_request: self.non_proxy_agent.post(path),
        }
    }

    pub fn put(&self, path: &str) -> HttpRequest {
        HttpRequest {
            use_proxy: self.use_proxy.clone(),
            proxy_request: self.proxy_agent.put(path),
            non_proxy_request: self.non_proxy_agent.put(path),
        }
    }

    pub fn delete(&self, path: &str) -> HttpRequest {
        HttpRequest {
            use_proxy: self.use_proxy.clone(),
            proxy_request: self.proxy_agent.delete(path),
            non_proxy_request: self.non_proxy_agent.delete(path),
        }
    }
}

fn get_proxy_path() -> PathBuf {
    let mut path_to_github_token = current_exe().unwrap().parent().unwrap().to_owned();
    path_to_github_token.push("proxy");
    path_to_github_token
}

fn get_proxy_and_prompt_if_not_found(
    user_input_generator: &mut dyn TextInputGeneratorTrait,
) -> String {
    let path_to_github_token = get_proxy_path();
    match fs::read_to_string(path_to_github_token.clone()) {
        Ok(github_token) => github_token,
        Err(_) => {
            let github_token = user_input_generator
                .get_text_input("Please enter your proxy, leave blank if you don't have one")
                .unwrap();
            fs::write(path_to_github_token, github_token.clone())
                .expect("failed to write proxy to file");
            github_token
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use serial_test::serial;

    use super::*;
    use crate::user_input_generator::testing::MockTextInputGenerator;

    #[test]
    #[serial]
    fn handle_when_proxy_not_saved() {
        let path_to_github_token = get_proxy_path();
        fs::remove_file(path_to_github_token.clone()).unwrap_or_default();

        let mut user_input_generator = MockTextInputGenerator::new(vec!["proxy".to_string()]);

        let github_token = get_proxy_and_prompt_if_not_found(&mut user_input_generator);

        assert_eq!("proxy".to_string(), github_token);
        assert_eq!(
            "proxy",
            fs::read_to_string(path_to_github_token.clone()).unwrap()
        );
        fs::remove_file(path_to_github_token).expect("failed to delete token");
    }

    #[test]
    #[serial]
    fn handle_when_token_already_exists() {
        let path_to_github_token = get_proxy_path();
        fs::remove_file(path_to_github_token.clone()).unwrap_or_default();
        fs::write(path_to_github_token.clone(), "existing_github_token")
            .expect("failed to create token");
        let mut user_input_generator = MockTextInputGenerator::new(Vec::new());

        let github_token = get_proxy_and_prompt_if_not_found(&mut user_input_generator);

        assert_eq!("existing_github_token".to_string(), github_token);
        assert_eq!(
            "existing_github_token",
            fs::read_to_string(path_to_github_token.clone()).unwrap()
        );
        fs::remove_file(path_to_github_token).expect("failed to delete token");
    }
}
