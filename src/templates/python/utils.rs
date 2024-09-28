pub fn pyproject_template(project_name: &str) -> String {
    format!(
        r#"[tool.poetry]
name = "{}"
version = "0.1.0"
description = ""
authors = ["Your Name <you@example.com>"]

[tool.isort]
profile = "black"

[tool.black]
line-length = 88
target-version = ['py38']

[tool.ruff]
exclude = [
    ".bzr",
    ".direnv",
    ".eggs",
    ".git",
    ".git-rewrite",
    ".hg",
    ".ipynb_checkpoints",
    ".mypy_cache",
    ".nox",
    ".pants.d",
    ".pyenv",
    ".pytest_cache",
    ".pytype",
    ".ruff_cache",
    ".svn",
    ".tox",
    ".venv",
    ".vscode",
    "__pypackages__",
    "_build",
    "buck-out",
    "build",
    "dist",
    "node_modules",
    "site-packages",
    "venv",
]

line-length = 88
indent-width = 4

target-version = "py38"

[tool.ruff.lint]
select = ["E", "W", "I", "N", "D", "UP", "RUF", "PERF", "ERA", "PTH"]

fixable = ["ALL"]
unfixable = []

dummy-variable-rgx = "^(_+|(_+[a-zA-Z0-9_]*[a-zA-Z0-9]+?))$"

[tool.ruff.format]
quote-style = "double"

indent-style = "space"

skip-magic-trailing-comma = false

line-ending = "auto"

[tool.mypy]
ignore_missing_imports = true
"#,
        project_name
    )
}

pub fn setup_template(project_name: &str) -> String {
    format!(
        r#"from setuptools import setup, find_packages

setup(
    name="{}",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[],
    author="Your Name",
    author_email="you@example.com",
    description="",
)
"#,
        project_name
    )
}
