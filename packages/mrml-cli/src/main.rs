use clap::{Parser, Subcommand};
use mrml::mjml::MJML;
use mrml::prelude::print::Print;
use mrml::prelude::render::Options as RenderOptions;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Options {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
    /// Path to your mjml file
    #[clap(index = 1)]
    pub input: Option<String>,
}

impl Options {
    fn read_file(&self, filename: &str) -> String {
        log::debug!("reading from file {}", filename);
        let mut file = File::open(filename).expect("couldn't find file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("couldn't read file");
        content
    }

    fn read_stdin(&self) -> String {
        log::info!("waiting for input...");
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("couldn't read input");
        buffer
    }

    fn parse_json(&self, input: &str) -> Result<MJML, String> {
        log::debug!("parsing json input");
        serde_json::from_str::<MJML>(input)
            .map_err(|err| format!("unable to parse json: {:?}", err))
    }

    fn parse_mjml(&self, input: &str) -> Result<MJML, String> {
        log::debug!("parsing mjml input");
        MJML::parse(input).map_err(|err| format!("unable to parse mjml: {:?}", err))
    }

    fn parse_input(&self, input: &str) -> MJML {
        if let Some(ref filename) = self.input {
            if filename.ends_with(".json") {
                self.parse_json(input).unwrap()
            } else if filename.ends_with(".mjml") {
                self.parse_mjml(input).unwrap()
            } else {
                panic!("unknown file type");
            }
        } else {
            self.parse_mjml(input)
                .or_else(|_| self.parse_json(input))
                .expect("unable to parse input")
        }
    }

    fn read_input(&self) -> String {
        if let Some(ref filename) = self.input {
            self.read_file(filename)
        } else {
            self.read_stdin()
        }
    }

    pub fn execute(self) {
        let root = self.read_input();
        let root = self.parse_input(&root);
        self.subcmd.execute(&root);
    }
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// Format template to JSON
    FormatJSON(Format),
    /// Format template to MJML
    FormatMJML(Format),
    /// Render template to HTML
    Render(Render),
    /// Read input file and validate its structure
    Validate,
}

impl SubCommand {
    pub fn execute(self, root: &MJML) {
        match self {
            Self::FormatJSON(opts) => {
                log::debug!("format to json");
                let output = if opts.pretty {
                    serde_json::to_string_pretty(root).expect("couldn't format to JSON")
                } else {
                    serde_json::to_string(root).expect("couldn't format to JSON")
                };
                println!("{}", output);
            }
            Self::FormatMJML(opts) => {
                log::debug!("format to mjml");
                let output = if opts.pretty {
                    root.pretty_print()
                } else {
                    root.dense_print()
                };
                println!("{}", output);
            }
            Self::Render(render) => {
                log::debug!("render");
                let render_opts = RenderOptions::from(render);
                let output = root.render(&render_opts).expect("couldn't render template");
                println!("{}", output);
            }
            Self::Validate => log::debug!("validate"),
        };
    }
}

#[derive(Debug, Parser)]
struct Format {
    /// Pretty print
    #[clap(long)]
    pub pretty: bool,
}

#[derive(Debug, Parser)]
struct Render {
    /// Remove comments from html output
    #[clap(short, long)]
    pub disable_comments: bool,
    /// Base url for social icons
    #[clap(short, long)]
    pub social_icon_origin: Option<String>,
}

impl From<Render> for RenderOptions {
    fn from(value: Render) -> Self {
        Self {
            disable_comments: value.disable_comments,
            social_icon_origin: value.social_icon_origin,
        }
    }
}

fn main() {
    env_logger::init();
    Options::parse().execute();
}

#[cfg(test)]
mod tests {
    use super::Options;
    use clap::Parser;

    fn execute(args: Vec<&str>) {
        Options::parse_from(args).execute();
    }

    fn execute_stdin(args: Vec<&str>, input: &str) {
        let opts = Options::parse_from(args);
        let root = opts.parse_input(input);
        opts.subcmd.execute(&root);
    }

    #[test]
    #[should_panic]
    fn missing_file() {
        execute(vec!["mrml-cli", "./cant/be/found.mjml", "validate"]);
    }

    #[test]
    #[should_panic]
    fn unknown_extension() {
        execute(vec!["mrml-cli", "./cant/be/found.txt", "validate"]);
    }

    #[test]
    #[should_panic]
    fn unknown_extension_stdin() {
        execute_stdin(vec!["mrml-cli", "validate"], "###");
    }

    #[test]
    fn format_json_amario_stdio() {
        let input = include_str!("../resources/amario.mjml");
        execute_stdin(vec!["mrml-cli", "format-json"], input);
    }

    #[test]
    fn format_json_amario() {
        execute(vec!["mrml-cli", "./resources/amario.mjml", "format-json"]);
    }

    #[test]
    fn format_json_pretty_amario() {
        execute(vec![
            "mrml-cli",
            "./resources/amario.mjml",
            "format-json",
            "--pretty",
        ]);
    }

    #[test]
    fn format_mjml_amario() {
        execute(vec!["mrml-cli", "./resources/amario.json", "format-mjml"]);
    }

    #[test]
    fn format_mjml_pretty_amario() {
        execute(vec![
            "mrml-cli",
            "./resources/amario.json",
            "format-mjml",
            "--pretty",
        ]);
    }

    #[test]
    fn render_amario() {
        execute(vec!["mrml-cli", "./resources/amario.mjml", "render"]);
    }

    #[test]
    fn validate_amario_json() {
        execute(vec!["mrml-cli", "./resources/amario.json", "validate"]);
    }

    #[test]
    fn validate_amario_mjml() {
        execute(vec!["mrml-cli", "./resources/amario.mjml", "validate"]);
    }
}
