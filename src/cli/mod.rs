use std::{
    fs::{self, File},
    io::BufReader,
};

use clap::Parser;

use self::errors::Errors;

mod errors;

///Flake's interpreter
#[derive(Debug, Parser)]
pub struct Args {
    ///A path file with extension .fk
    path: String,
}

pub fn process(args: Args) -> Result<(), Errors> {
    let Ok(file) = fs::read_to_string(args.path) else {
        return Err(Errors::FileNotFound);
    };

    println!("{:?}", file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_not_found_file() {
        let result = process(Args {
            path: "./flake.fs".to_string(),
        });
        assert!(result.is_err())
    }

    #[test]
    fn it_should_return_ok_file_founded() {
        let result = process(Args {
            path: "/home/mawfy/projects/flake/flake.fk".to_string(),
        });
        assert!(result.is_ok())
    }
}
