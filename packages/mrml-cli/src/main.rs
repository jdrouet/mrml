use clap::{crate_authors, crate_version, Clap};
use mrml::prelude::render::Options as RenderOptions;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;

#[derive(Clap, Debug)]
#[clap(
    version = crate_version!(),
    author = crate_authors!()
    )]
struct Options {
    #[clap(short, long, about = "Remove comments from html output")]
    pub disable_comments: bool,
    #[clap(short, long, about = "Base url for social icons")]
    pub social_icon_origin: Option<String>,
    #[clap(about = "Path to your mjml file")]
    pub input: String,
}

impl From<Options> for RenderOptions {
    fn from(value: Options) -> Self {
        Self {
            disable_comments: value.disable_comments,
            social_icon_origin: value.social_icon_origin,
        }
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
    let root = mrml::mjml::MJML::parse(content).expect("parsing template");
    let render_opts = RenderOptions::from(opts);
    let output = root.render(&render_opts).expect("rendering");
    println!("{}", output);
}
