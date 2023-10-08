use inquire::MultiSelect;

pub trait UserInputGeneratorTrait {
    fn get_text_input(&mut self, prompt: &str) -> Option<String>;
    fn get_multiselect_input(&mut self, prompt: &str, options: Vec<String>) -> Option<Vec<String>>;
}



pub struct InquireUserInputGenerator;

impl InquireUserInputGenerator {
    pub fn new()-> Self{
        Self{}
    }
}

impl UserInputGeneratorTrait for InquireUserInputGenerator{
    fn get_text_input(&mut self, prompt: &str) -> Option<String> {
        match inquire::Password::new(prompt.into()).without_confirmation().prompt(){
            Ok(result) => Some(result),
            Err(_) => None, 
        }
    }

    fn get_multiselect_input(&mut self, prompt: &str, options: Vec<String>) -> Option<Vec<String>> {
        match MultiSelect::new(
            prompt,
            options,
        ).prompt() {
            Ok(result) => Some(result),
            Err(_) => None, 
        }
    }
}

#[cfg(test)]
pub mod testing {
    use super::UserInputGeneratorTrait;


pub struct MockUserInputGenerator {
    text_inputs: Vec<String>,
    multiselect_inputs: Vec<Vec<String>>,
}

impl MockUserInputGenerator {
    pub fn new(mut text_inputs: Vec<String>, mut multiselect_inputs: Vec<Vec<String>>) -> Self {
        text_inputs.reverse();
        multiselect_inputs.reverse();
        Self {
            text_inputs,
            multiselect_inputs,
        }
    }
}

impl UserInputGeneratorTrait for MockUserInputGenerator {
    fn get_text_input(&mut self, _: &str) -> Option<String> {
        self.text_inputs.pop()
    }

    fn get_multiselect_input(&mut self, _: &str, options: Vec<String>) -> Option<Vec<String>> {
        match self.multiselect_inputs.pop() {
            Some(result)=> {
                panic_if_list_contains_unknown_option(&result, &options);
                Some(result)
            },
            None => None
        }
    }
}

fn panic_if_list_contains_unknown_option(result: &Vec<String>, options: &Vec<String>) {
    result.iter().filter(|each| !options.contains(each)).for_each(|each| panic!("Unknown option \"{}\" found for {:?}", each, options));
}
}

#[cfg(test)]
mod test {

    use crate::user_input_generator::{UserInputGeneratorTrait, testing::MockUserInputGenerator};    

    #[test]
    fn can_get_text_inputs() {
        let mut input_generator =
            MockUserInputGenerator::new(vec!["input 1".to_string()], Vec::new());
        assert_eq!("input 1", input_generator.get_text_input("the prompt").unwrap())
    }
    #[test]
    fn can_get_text_inputs_multiple() {
        let mut input_generator =
            MockUserInputGenerator::new(vec!["input 1".to_string(), "input 2".to_string()], Vec::new());
        input_generator.get_text_input("prompt");
        assert_eq!("input 2", input_generator.get_text_input("the prompt").unwrap())
    }
    #[test]
    fn can_return_none_when_no_text_inputs_exist() {
        let mut input_generator =
            MockUserInputGenerator::new(Vec::new(), Vec::new());
        assert_eq!(None, input_generator.get_text_input("the prompt"))
    }
    

    #[test]
    fn can_get_multi_select_inputs() {
        let mut input_generator =
            MockUserInputGenerator::new(Vec::new(), vec![vec!["input 1".to_string(), "input 2".to_string()]]);
        let options = vec!["input 1".to_string(), "input 2".to_string(), "input 3".to_string()];
        assert_eq!(vec!["input 1".to_string(), "input 2".to_string()], input_generator.get_multiselect_input("the prompt", options).unwrap());
    }

    #[test]
    #[should_panic]
    fn can_get_multi_select_inputs_panics_because_options_do_not_match_output() {
        let mut input_generator =
            MockUserInputGenerator::new(Vec::new(), vec![vec!["input 1".to_string(), "bad output".to_string()]]);
        let options = vec!["input 1".to_string(), "input 2".to_string(), "input 3".to_string()];
        input_generator.get_multiselect_input("the prompt", options).unwrap();
    }

    #[test]
    fn can_get_multi_select_inputs_multiple() {
        let mut input_generator =
            MockUserInputGenerator::new(Vec::new(), vec![
                vec!["input 1".to_string(), "input 2".to_string()],
                vec!["input 3".to_string(), "input 4".to_string()],
                ]);
        let options = vec!["input 1".to_string(), "input 2".to_string(), "input 3".to_string(), "input 4".to_string(), "input 5".to_string()];
        
        input_generator.get_multiselect_input("the prompt", options.clone());
        assert_eq!(vec!["input 3".to_string(), "input 4".to_string()], input_generator.get_multiselect_input("the prompt", options).unwrap());
    }

}
