use std::env::{current_dir, args, temp_dir};
use std::fs::create_dir;
use std::os::unix::fs::symlink;
use std::process::ExitCode;

fn main() -> ExitCode {
    match current_dir() {
        Ok(pwd) => {
            let dir_prefix = pwd.to_str()
                .expect("")
                .chars()
                .skip(1)
                .collect::<String>()
                .replace("/", "-");
            for arg in args().skip(1) {
                let new_dir = [
                    temp_dir().to_str().expect(""),
                    &(dir_prefix.to_string() + "-" + &arg)
                ].join("/");

                if let Err(err) = create_dir(&new_dir) {
                    eprintln!(
                        "Failed to create directory {}: {}",
                        arg,
                        err.to_string()
                    );
                    return ExitCode::FAILURE;
                }

                let _ = symlink(
                    &new_dir,
                    String::from(pwd.to_str().expect("")) + "/" + &arg
                );
            }

            return ExitCode::SUCCESS;
        },
        Err(err) => {
            eprintln!("Failed to get PWD: {}", err.to_string());
            return ExitCode::FAILURE;
        },
    };
}
