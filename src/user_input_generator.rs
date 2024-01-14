use std::fmt::Display;

use inquire::MultiSelect;

pub trait TextInputGeneratorTrait {
    fn get_password_input(&mut self, prompt: &str) -> Option<String>;
    fn get_text_input(&mut self, prompt: &str) -> Option<String>;
}

pub trait MultiSelectGeneratorTrait<T> where T:Display {
    fn get_multiselect_input(&mut self, prompt: &str, options: Vec<T>) -> Option<Vec<T>>;
}

pub struct InquireTextInputGenerator;

impl InquireTextInputGenerator{
    pub fn new()-> Self{
        Self{}
    }
}
impl TextInputGeneratorTrait for InquireTextInputGenerator{
    fn get_password_input(&mut self, prompt: &str) -> Option<String> {
        match inquire::Password::new(prompt.into()).without_confirmation().prompt(){
            Ok(result) => Some(result),
            Err(_) => None, 
        }
    }
    fn get_text_input(&mut self, prompt: &str) -> Option<String> {
        match inquire::Text::new(prompt.into()).prompt(){
            Ok(result) => Some(result),
            Err(_) => None, 
        }
    }
}

pub struct InquireMultiSelectGenerator;

impl InquireMultiSelectGenerator {
    pub fn new()-> Self{
        Self{}
    }
}

impl<T> MultiSelectGeneratorTrait<T> for InquireMultiSelectGenerator where T:Display{
    fn get_multiselect_input(&mut self, prompt: &str, options: Vec<T>) -> Option<Vec<T>> {
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
    use std::fmt::Display;

    use super::{TextInputGeneratorTrait, MultiSelectGeneratorTrait};


pub struct MockTextInputGenerator {
    text_inputs: Vec<String>
}
impl MockTextInputGenerator {
    pub fn new(mut text_inputs: Vec<String>) -> Self {
        text_inputs.reverse();
        Self {
            text_inputs
        }
    }
}

impl TextInputGeneratorTrait for MockTextInputGenerator {
    fn get_password_input(&mut self, _: &str) -> Option<String> {
        self.text_inputs.pop()
    }
    fn get_text_input(&mut self, _: &str) -> Option<String> {
        self.text_inputs.pop()
    }
}

pub struct MockMultiSelectGenerator<T> where T:Display {
    multiselect_inputs: Vec<Vec<T>>,
}

impl <T> MockMultiSelectGenerator<T>  where T:Display{
    pub fn new(mut multiselect_inputs: Vec<Vec<T>>) -> Self {
        multiselect_inputs.reverse();
        Self {
            multiselect_inputs,
        }
    }
}

impl <T> MultiSelectGeneratorTrait<T> for MockMultiSelectGenerator<T> where T:Display+Eq+std::fmt::Debug {
    fn get_multiselect_input(&mut self, _: &str, options: Vec<T>) -> Option<Vec<T>> {
        match self.multiselect_inputs.pop() {
            Some(result)=> {
                panic_if_list_contains_unknown_option(&result, &options);
                Some(result)
            },
            None => None
        }
    }
}


fn panic_if_list_contains_unknown_option<T>(result: &Vec<T>, options: &Vec<T>) where T:Display+Eq+std::fmt::Debug{
    result.iter().filter(|each| !options.contains(each)).for_each(|each| panic!("Unknown option \"{}\" found for {:?}", each, options));
}
}

#[cfg(test)]
mod test {

    use crate::user_input_generator::{testing::{MockTextInputGenerator, MockMultiSelectGenerator}, TextInputGeneratorTrait};

    use super::MultiSelectGeneratorTrait;    

    #[test]
    fn can_get_text_inputs() {
        let mut input_generator =
            MockTextInputGenerator::new(vec!["input 1".to_string()]);
        assert_eq!("input 1", input_generator.get_password_input("the prompt").unwrap())
    }
    #[test]
    fn can_get_text_inputs_multiple() {
        let mut input_generator =
            MockTextInputGenerator::new(vec!["input 1".to_string(), "input 2".to_string()]);
        input_generator.get_password_input("prompt");
        assert_eq!("input 2", input_generator.get_password_input("the prompt").unwrap())
    }
    #[test]
    fn can_return_none_when_no_text_inputs_exist() {
        let mut input_generator =
            MockTextInputGenerator::new(Vec::new());
        assert_eq!(None, input_generator.get_password_input("the prompt"))
    }
    

    #[test]
    fn can_get_multi_select_inputs() {
        let mut input_generator =
            MockMultiSelectGenerator::new(vec![vec!["input 1".to_string(), "input 2".to_string()]]);
        let options = vec!["input 1".to_string(), "input 2".to_string(), "input 3".to_string()];
        assert_eq!(vec!["input 1".to_string(), "input 2".to_string()], input_generator.get_multiselect_input("the prompt", options).unwrap());
    }

    #[test]
    #[should_panic]
    fn can_get_multi_select_inputs_panics_because_options_do_not_match_output() {
        let mut input_generator =
        MockMultiSelectGenerator::new(vec![vec!["input 1".to_string(), "bad output".to_string()]]);
        let options = vec!["input 1".to_string(), "input 2".to_string(), "input 3".to_string()];
        input_generator.get_multiselect_input("the prompt", options).unwrap();
    }

    #[test]
    fn can_get_multi_select_inputs_multiple() {
        let mut input_generator =
        MockMultiSelectGenerator::new(vec![
                vec!["input 1".to_string(), "input 2".to_string()],
                vec!["input 3".to_string(), "input 4".to_string()],
                ]);
        let options = vec!["input 1".to_string(), "input 2".to_string(), "input 3".to_string(), "input 4".to_string(), "input 5".to_string()];
        
        input_generator.get_multiselect_input("the prompt", options.clone());
        assert_eq!(vec!["input 3".to_string(), "input 4".to_string()], input_generator.get_multiselect_input("the prompt", options).unwrap());
    }

}
