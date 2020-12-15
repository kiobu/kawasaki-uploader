use std::env;
use std::path::Path;
use std::process::Command;

fn get_authkey() -> String {
    match env::var("KAWASAKI_AUTHKEY") {
        Ok(key) => {
            println!("Key found: {}", key);
            return String::from(key)
        },
        Err(_) => String::from("VARNOKEY")
    }
}

fn main()  {
    let key: String = get_authkey();
    let host = "http://localhost:3830/api";

    if key == "VARNOKEYZ" {
        return println!("{}", "No key has been set; use 'export KAWASAKI_AUTHKEY=yourkeyhere'.")
    } else {
        let args: Vec<String> = env::args().collect();

        match args.get(1) {
            Some(string) => match Path::new(string).is_file() {
                true => {
                    println!("Path: {}", string);
                    match Command::new("curl")
                        .arg("-v")
                        .arg("-X POST")
                        .arg(format!("-F key='{}'", key))
                        .arg(format!("-F file='@{}'", string)) // curl: (26) Failed to open/read local data from file/application
                        .arg(format!("{}", host))
                        .spawn() {
                            Ok(out) => { println!("Getting process -> {:?}", out) },
                            Err(err) => { println!("An error occured: {}", err) }
                        }
                    println!("\n")
                },
                false => println!("{}", "The given path is not a file.")
            }, 
            None => println!("{}", "You did not specify a path.\n    Usage: kawasaki path/to/file")
        }
    }

}
