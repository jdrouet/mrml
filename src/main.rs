use clap::Clap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;

#[derive(Clap, Debug)]
#[clap(
    version = "0.2.0",
    author = "Jeremie Drouet <jeremie.drouet@gmail.com>"
)]
struct Options {
    pub input: String,
}

fn read_file(path: &String) -> Result<String, IOError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let opts = Options::parse();
    let content = match read_file(&opts.input) {
        Ok(content) => content,
        Err(err) => {
            panic!("couldn't read input file: {:?}", err);
        }
    };
    let output = match mrml::to_html(content.as_str(), mrml::Options::default()) {
        Ok(output) => output,
        Err(err) => {
            panic!("couldn't convert mjml: {:?}", err);
        }
    };
    println!("{}", output);
}
