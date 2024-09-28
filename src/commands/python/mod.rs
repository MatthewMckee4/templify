use clap::Args;

use crate::{
    config::python::{
        CONFIGURATION_FILES, PYTHON_FORMATTERS, PYTHON_LINTERS, PYTHON_STATIC_TYPE_CHECKERS,
        PYTHON_TEST_RUNNERS,
    },
    templates::{python::PythonTemplate, Template},
    utils::{
        cli::{get_file_path, get_input, get_non_empty_input},
        completer::StringCompleter,
        get_strings_as_vec,
        system::get_python_versions,
    },
};

#[derive(Args)]
pub struct PythonCommand {
    #[arg(short, long)]
    project_name: Option<String>,
    #[arg(short, long)]
    configuration_file: Option<String>,
    #[arg(short, long)]
    output_location: Option<String>,
    #[arg(short, long)]
    formatter: Option<String>,
    #[arg(short, long)]
    linter: Option<String>,
    #[arg(short, long)]
    test_runner: Option<String>,
    #[arg(short, long)]
    static_type_checker: Option<String>,
}

impl PythonCommand {
    pub fn execute(&self) {
        let project_name = self.project_name.clone().unwrap_or_else(|| {
            get_non_empty_input::<StringCompleter>("Enter the project name: ", None)
        });

        let completer = StringCompleter::new(get_strings_as_vec(CONFIGURATION_FILES));
        let configuration_file = self.configuration_file.clone().unwrap_or_else(|| {
            get_input(
                "Enter the configuration file (pyproject.toml/setup.py): ",
                |input| input == "pyproject.toml" || input == "setup.py",
                Some(&completer),
            )
        });

        let output_location = self
            .output_location
            .clone()
            .unwrap_or_else(|| get_file_path("Enter the output location: "));

        let python_versions = get_python_versions();

        let completer = StringCompleter::new(python_versions.clone());

        let python_version = get_input(
            "Enter the Python version: ",
            |input| python_versions.contains(&input.to_string()),
            Some(&completer),
        );

        let formatter = self.formatter.clone().unwrap_or_else(|| {
            get_non_empty_input::<StringCompleter>(
                "Enter the formatter: ",
                Some(&StringCompleter::new(get_strings_as_vec(PYTHON_FORMATTERS))),
            )
        });

        let linter = self.linter.clone().unwrap_or_else(|| {
            get_non_empty_input::<StringCompleter>(
                "Enter the linter: ",
                Some(&StringCompleter::new(get_strings_as_vec(PYTHON_LINTERS))),
            )
        });

        let test_runner = self.test_runner.clone().unwrap_or_else(|| {
            get_non_empty_input::<StringCompleter>(
                "Enter the test runner: ",
                Some(&StringCompleter::new(get_strings_as_vec(
                    PYTHON_TEST_RUNNERS,
                ))),
            )
        });

        let static_type_checker = self.static_type_checker.clone().unwrap_or_else(|| {
            get_non_empty_input::<StringCompleter>(
                "Enter the static type checker: ",
                Some(&StringCompleter::new(get_strings_as_vec(
                    PYTHON_STATIC_TYPE_CHECKERS,
                ))),
            )
        });

        let template = PythonTemplate::new(
            project_name,
            configuration_file,
            output_location,
            python_version,
            formatter,
            linter,
            test_runner,
            static_type_checker,
        );
        template.create();
    }
}
