use utils::{pyproject_template, setup_template};

use crate::templates::Template;
use std::{
    fs::{create_dir_all, write},
    process::Command,
};

mod utils;

pub struct PythonTemplate {
    project_name: String,
    configuration_file: String,
    output_location: String,
    python_version: String,
    formatter: String,
    linter: String,
    test_runner: String,
    static_type_checker: String,
}

#[allow(clippy::too_many_arguments)]
impl PythonTemplate {
    pub fn new(
        project_name: String,
        configuration_file: String,
        output_location: String,
        python_version: String,
        formatter: String,
        linter: String,
        test_runner: String,
        static_type_checker: String,
    ) -> Self {
        Self {
            project_name,
            configuration_file,
            output_location,
            python_version,
            formatter,
            linter,
            test_runner,
            static_type_checker,
        }
    }
}

impl Template for PythonTemplate {
    fn create(&self) {
        let expanded_project_folder_path = self.get_output_location();
        self.create_project_directory(&expanded_project_folder_path);
        self.create_configuration_file(&expanded_project_folder_path);
        self.install_uv();
        self.create_virtual_environment(&expanded_project_folder_path);
        self.install_formatter();
        self.install_linter();
        self.install_test_runner();
        self.install_static_type_checker();
        self.create_requirements_file();
        self.create_code_folder();

        println!(
            "Created Python project '{}' with '{}' in '{}'",
            self.project_name, self.configuration_file, self.output_location
        );
    }
}

impl PythonTemplate {
    fn create_project_directory(&self, path: &str) {
        create_dir_all(path).unwrap();
    }

    fn get_output_location(&self) -> String {
        let project_folder_path = format!("{}/{}", &self.output_location, &self.project_name);
        let expanded_project_folder_path = shellexpand::tilde(&project_folder_path).to_string();
        expanded_project_folder_path
    }

    fn get_source_code_folder(&self) -> String {
        format!("{}/{}", &self.get_output_location(), &self.project_name)
    }

    fn create_configuration_file(&self, project_folder_path: &str) {
        let content = if self.configuration_file == "pyproject.toml" {
            pyproject_template(&self.project_name)
        } else if self.configuration_file == "setup.py" {
            setup_template(&self.project_name)
        } else {
            String::new()
        };

        let expanded_path = shellexpand::tilde(&format!(
            "{}/{}",
            project_folder_path, &self.configuration_file
        ))
        .to_string();
        write(expanded_path, content).unwrap();
    }

    fn install_uv(&self) {
        let command_install_uv = format!("{} -m pip install uv", self.python_version);
        let output_install_uv = Command::new("sh")
            .arg("-c")
            .arg(command_install_uv)
            .output()
            .unwrap();

        if !output_install_uv.status.success() {
            eprintln!(
                "Failed to install uv: {}",
                String::from_utf8_lossy(&output_install_uv.stderr)
            );
        }
    }

    fn create_virtual_environment(&self, project_folder_path: &str) {
        let command_cd = format!("cd {}", project_folder_path);
        let command_create_venv = format!("{} -m uv venv", self.python_version);
        let full_command = format!("{} && {}", command_cd, command_create_venv);
        let output_create_venv = Command::new("sh")
            .arg("-c")
            .arg(full_command)
            .output()
            .unwrap();

        if output_create_venv.status.success() {
            println!("Virtual environment created successfully.");
        } else {
            eprintln!(
                "Failed to create virtual environment: {}",
                String::from_utf8_lossy(&output_create_venv.stderr)
            );
        }
    }

    fn get_activate_virtual_environment_command(&self) -> String {
        #[cfg(target_os = "windows")]
        let command_activate_venv = format!("{}\\.venv\\Scripts\\activate", project_folder_path);

        #[cfg(not(target_os = "windows"))]
        let command_activate_venv =
            format!("cd {} . .venv/bin/activate", self.get_output_location());

        command_activate_venv
    }

    fn install_formatter(&self) {
        if !self.formatter.is_empty() {
            self.pip_install_package(&self.formatter);
        }
    }

    fn install_linter(&self) {
        if !self.linter.is_empty() {
            self.pip_install_package(&self.linter);
        }
    }

    fn install_test_runner(&self) {
        if !self.test_runner.is_empty() {
            self.pip_install_package(&self.test_runner);
        }
    }

    fn install_static_type_checker(&self) {
        if !self.static_type_checker.is_empty() {
            self.pip_install_package(&self.static_type_checker);
        }
    }

    fn pip_install_package(&self, package: &str) {
        let command_pip_install = format!(
            "{} && uv pip install {}",
            self.get_activate_virtual_environment_command(),
            package
        );
        let output_pip_install = Command::new("sh")
            .arg("-c")
            .arg(command_pip_install)
            .output()
            .unwrap();

        if output_pip_install.status.success() {
            println!("Package '{}' installed successfully.", package);
        } else {
            println!(
                "Failed to install package '{}': {}",
                package,
                String::from_utf8_lossy(&output_pip_install.stderr)
            );
            println!(
                "Output: {}",
                String::from_utf8_lossy(&output_pip_install.stdout)
            );
        }
    }

    fn create_requirements_file(&self) {
        let command = format!(
            "{} && {}",
            self.get_activate_virtual_environment_command(),
            "uv pip freeze > requirements.txt"
        );
        let output = Command::new("sh").arg("-c").arg(command).output().unwrap();
        if output.status.success() {
            println!("Requirements file created successfully.");
        } else {
            println!(
                "Failed to create requirements file: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    fn create_code_folder(&self) {
        create_dir_all(self.get_source_code_folder()).unwrap();
        write(format!("{}/__init__.py", self.get_source_code_folder()), "").unwrap();
    }
}
