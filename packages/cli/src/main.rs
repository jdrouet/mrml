use clap::{crate_authors, crate_version, Clap};
use mrml::util::size::Size::Pixel;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;

#[derive(Clap, Debug)]
#[clap(
    version = crate_version!(),
    author = crate_authors!()
)]
struct Options {
    #[clap(short, long, about = "Keeps comments from mjml in output")]
    pub keep_comments: bool,
    #[clap(short, long, about = "Size of the breakpoint in pixels")]
    pub breakpoint: Option<f32>,
    #[clap(short, long, about = "Base url for social icons")]
    pub social_icon_origin: Option<String>,
    #[clap(about = "Path to your mjml file")]
    pub input: String,
}

impl Into<mrml::Options> for Options {
    fn into(self) -> mrml::Options {
        let mut res = mrml::Options {
            keep_comments: self.keep_comments,
            ..Default::default()
        };
        if let Some(bp) = self.breakpoint {
            res.breakpoint = Pixel(bp);
        }
        res
    }
}

fn read_file(path: &str) -> Result<String, IOError> {
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
