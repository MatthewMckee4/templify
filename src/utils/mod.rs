pub mod cli;
pub mod completer;
pub mod system;

pub fn get_strings_as_vec(strings: &[&str]) -> Vec<String> {
    strings.iter().map(|&s| s.to_string()).collect()
}
