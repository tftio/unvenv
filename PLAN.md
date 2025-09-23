# Plan: Python venv detector CLI (Rust)

Goal: Implement a Rust CLI that scans the current Git working tree for any non-ignored `pyvenv.cfg` files. If any are found that are NOT ignored by `.gitignore` rules, exit with a non-zero status and print a useful, actionable message detailing where they were found and key fields.

## Steps

1. Create Project
   - Initialize a Rust binary crate in the current directory without creating a Git repo: `cargo init --bin --vcs none .`

2. Add Dependencies
   - Add `git2 = "0.18"` for Git repo discovery and ignore checks.
   - Add `walkdir = "2.5"` for efficient recursive directory traversal.

3. Implement Scanner
   - Discover enclosing Git repository from `.` using `git2::Repository::discover(".")`.
   - If not in a repository or repo is bare, exit successfully (code 0) doing nothing.
   - Determine workdir root with `repo.workdir()`.
   - Traverse the working tree with `walkdir`, skipping `.git/`.
   - For each file named `pyvenv.cfg`, compute a path relative to the repo workdir.
   - Use Git ignore logic (`repo.status_should_ignore(rel_path)`) to check whether the file is ignored by `.gitignore`/excludes. If ignored, skip it.
   - For non-ignored matches, parse `pyvenv.cfg` to extract helpful fields: `home`, `version`, `include-system-site-packages`.

4. Output and Exit Codes
   - If no non-ignored matches, exit 0.
   - If any non-ignored matches exist:
     - Print a header explaining the issue.
     - List each offending relative path and show the extracted fields, if present.
     - Suggest `.gitignore` entries for the parent directories of detected `pyvenv.cfg` files (e.g., `venv/`).
     - Exit with status 2 to clearly differentiate policy failure from internal errors.
   - On unexpected internal errors (I/O, Git errors that prevent scanning), print an error to stderr and exit 1.

5. Build
   - Compile with `cargo build` to ensure dependencies and code compile.

6. Smoke Test (in repo root for simplicity)
   - Initialize a Git repo in the project root for testing: `git init`.
   - Create `venv/pyvenv.cfg` with plausible content.
   - Ensure `.gitignore` does NOT ignore `venv/` initially.
   - Run `cargo run -q` and verify non-zero exit code (expect 2) and helpful output.
   - Add `venv/` to `.gitignore` and rerun to verify exit code 0.

7. Notes / Future Work
   - Consider an optional CLI arg to scan a specified directory instead of CWD.
   - Consider printing guidance to remove committed venvs from index (`git rm -r --cached <venv>`).
