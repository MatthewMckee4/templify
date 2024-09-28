use rustyline::completion::Completer;
use rustyline::Editor;
use rustyline::Helper;

use super::completer::FilenameCompleter;

pub fn get_input<C: Completer + Helper>(
    prompt: &str,
    is_valid: impl Fn(&str) -> bool,
    completer: Option<&C>,
) -> String {
    let mut rl = Editor::new().unwrap();
    if let Some(completer) = completer {
        rl.set_helper(Some(completer));
    }

    loop {
        let input = rl.readline(prompt).unwrap();
        let input = input.trim().to_string();
        if is_valid(&input) {
            return input;
        } else {
            println!("Invalid input. Please try again.");
        }
    }
}

pub fn get_non_empty_input<C: Completer + Helper>(prompt: &str, completer: Option<&C>) -> String {
    get_input(prompt, |input| !input.is_empty(), completer)
}

pub fn get_file_path(prompt: &str) -> String {
    let completer = FilenameCompleter::new();
    get_non_empty_input(prompt, Some(&completer))
}
