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
use std::path::Path;
use std::process::ExitCode;

pub fn prompt() -> ExitCode {
    let cwd = env::current_dir();

    match cwd {
        Ok(cwd) => {
            let home = dirs::home_dir().unwrap();
            let dpath = if cwd.starts_with(home) {
                let home = dirs::home_dir().unwrap();
                let newcwd = Path::new("~");
                let nohome = cwd.strip_prefix(home).unwrap();
                newcwd.join(nohome)
            } else {
                cwd
            };

            println!("{} ", dpath.display());
        }
        Err(_) => {
            println!("? ");
        }
    }

    ExitCode::SUCCESS
}
