use std::env;
use std::path::Path;
use std::process::Command;

fn get_authkey() -> String {
    match env::var("KAWASAKI_AUTHKEY") {
        Ok(key) => String::from(key),
        Err(_) => String::from("VARNOKEY")
    }
}

fn main() {
    let key: String = get_authkey();
    let host = "http://localhost:3830/api";

    if key == "VARNOKEY" {
        return println!("{}", "No key has been set; use 'export KAWASAKI_AUTHKEY=yourkeyhere'.")
    } else {
        let args: Vec<String> = env::args().collect();

        match args.get(1) {
            Some(string) => match Path::new(string).is_file() {
                true => {
                    println!("-F file=@'{}'", string);
                    Command::new("curl")
                        .arg("-v")
                        .arg(format!("-F key='{}'", key))
                        .arg(format!("-F file=@'{}'", string)) // <----- This line does not work.
                        .arg(format!("{}", host))
                        .spawn()
                        .expect("Cannot execute.");
                    println!("\n")
                },
                false => println!("{}", "The given path is not a file.")
            }, 
            None => println!("{}", "You did not specify a path.\n    Usage: kawasaki path/to/file")
        }
    }

}
