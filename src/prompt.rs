/* marioprompt: my fish shell prompt
 * Copyright 2025 Mario Finelli
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn prompt() -> ExitCode {
    let cwd = env::current_dir();

    match cwd {
        Ok(cwd) => {
            let dpath = replace_home_with_tilde(&cwd);
            println!("{} ❯ ", dpath.display());
        }
        Err(_) => {
            println!("? ❯ ");
        }
    }

    ExitCode::SUCCESS
}

/// Replaces the home directory in a path with `~` if it starts with the home
/// directory.
fn replace_home_with_tilde(path: &Path) -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        if path.starts_with(&home) {
            if let Ok(stripped) = path.strip_prefix(&home) {
                let mut result = PathBuf::from("~");
                if !stripped.as_os_str().is_empty() {
                    result.push(stripped);
                }
                return result;
            }
        }
    }
    path.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_home_with_tilde() {
        let home = dirs::home_dir().expect("Home directory not found");

        // Test: Path starts with home directory
        let path = home.join("Documents/test.txt");
        let result = replace_home_with_tilde(&path);
        assert_eq!(result, PathBuf::from("~/Documents/test.txt"));

        // Test: Path does not start with home directory
        let path = PathBuf::from("/usr/local/bin");
        let result = replace_home_with_tilde(&path);
        assert_eq!(result, PathBuf::from("/usr/local/bin"));

        // Test: Path is exactly the home directory
        let result = replace_home_with_tilde(&home);
        assert_eq!(result, PathBuf::from("~"));
    }
}
