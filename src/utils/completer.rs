use rustyline::completion::{Completer, FilenameCompleter as FilenameHelper, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::Helper;

pub struct StringCompleter {
    options: Vec<String>,
}

impl StringCompleter {
    pub fn new(options: Vec<String>) -> Self {
        Self { options }
    }
}

impl Completer for StringCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        _line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Pair>), rustyline::error::ReadlineError> {
        let candidates = self
            .options
            .iter()
            .map(|option| Pair {
                display: option.to_string(),
                replacement: option.to_string(),
            })
            .collect();
        Ok((0, candidates))
    }
}

impl Hinter for StringCompleter {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for StringCompleter {}

impl Validator for StringCompleter {
    fn validate(&self, _ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Helper for StringCompleter {}

pub struct FilenameCompleter(FilenameHelper);

impl FilenameCompleter {
    pub fn new() -> Self {
        FilenameCompleter(FilenameHelper::new())
    }
}

impl Default for FilenameCompleter {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator for FilenameCompleter {
    fn validate(&self, _ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }

    fn validate_while_typing(&self) -> bool {
        false
    }
}

impl Highlighter for FilenameCompleter {}

impl Hinter for FilenameCompleter {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        None
    }
}

impl Completer for FilenameCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        _line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.0.complete(_line, _pos, _ctx)
    }
}

impl Helper for FilenameCompleter {}
