use std::process::Command;

use crate::config::python::PYTHON_VERSIONS;

fn python_version_exists(python_binary: &str) -> bool {
    let output = Command::new(python_binary).arg("--version").output().ok();

    if let Some(output) = output {
        output.status.success()
    } else {
        false
    }
}

pub fn get_python_versions() -> Vec<String> {
    let mut versions = Vec::new();

    for binary in PYTHON_VERSIONS {
        if python_version_exists(binary) {
            versions.push(binary.to_string());
        }
    }

    versions
}
