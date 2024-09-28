pub static PYTHON_VERSIONS: &[&str] = &[
    "python",
    "python3",
    "python3.12",
    "python3.11",
    "python3.10",
    "python3.9",
    "python2",
];
pub static PYTHON_FORMATTERS: &[&str] = &["black", "ruff"];
pub static PYTHON_LINTERS: &[&str] = &["ruff", "pylint"];
pub static PYTHON_TEST_RUNNERS: &[&str] = &["pytest", "unittest"];
pub static PYTHON_STATIC_TYPE_CHECKERS: &[&str] = &["mypy"];
pub static CONFIGURATION_FILES: &[&str] = &["pyproject.toml", "setup.py"];
