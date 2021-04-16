use clap::{crate_authors, crate_description, crate_name, crate_version, Clap};
use mrml::mjml::MJML;
use mrml::prelude::print::Print;
use mrml::prelude::render::Options as RenderOptions;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clap)]
#[clap(name = crate_name!(), about = crate_description!(), version = crate_version!(), author = crate_authors!())]
struct Options {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
    #[clap(about = "Path to your mjml file", index = 1)]
    pub input: String,
}

impl Options {
    fn read_input_to_string(&self) -> String {
        let mut file = File::open(&self.input).expect("couldn't find file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("couldn't read file");
        content
    }

    fn read_input(&self) -> MJML {
        if self.input.ends_with(".json") {
            serde_json::from_str::<MJML>(&self.read_input_to_string()).expect("invalid json file")
        } else if self.input.ends_with(".mjml") {
            MJML::parse(self.read_input_to_string()).expect("invalid mjml file")
        } else {
            panic!("unknown file type");
        }
    }

    pub fn execute(self) {
        let root = self.read_input();
        self.subcmd.execute(&root);
    }
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about = "Format template to JSON")]
    FormatJSON(Format),
    #[clap(about = "Format template to MJML")]
    FormatMJML(Format),
    #[clap(about = "Render template to HTML")]
    Render(Render),
    #[clap(about = "Read input file and validate its structure")]
    Validate,
}

impl SubCommand {
    pub fn execute(self, root: &MJML) {
        match self {
            Self::FormatJSON(opts) => {
                let output = if opts.pretty {
                    serde_json::to_string_pretty(root).expect("couldn't format to JSON")
                } else {
                    serde_json::to_string(root).expect("couldn't format to JSON")
                };
                println!("{}", output);
            }
            Self::FormatMJML(opts) => {
                let output = if opts.pretty {
                    root.pretty_print()
                } else {
                    root.dense_print()
                };
                println!("{}", output);
            }
            Self::Render(render) => {
                let render_opts = RenderOptions::from(render);
                let output = root.render(&render_opts).expect("couldn't render template");
                println!("{}", output);
            }
            Self::Validate => (),
        };
    }
}

#[derive(Clap)]
struct Format {
    #[clap(long, about = "Pretty print")]
    pub pretty: bool,
}

#[derive(Clap)]
struct Render {
    #[clap(short, long, about = "Remove comments from html output")]
    pub disable_comments: bool,
    #[clap(short, long, about = "Base url for social icons")]
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
    Options::parse().execute();
}

#[cfg(test)]
mod tests {
    use super::Options;
    use clap::Clap;

    fn execute(args: Vec<&str>) {
        Options::parse_from(args).execute();
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
    fn format_json_amario() {
        execute(vec![
            "mrml-cli",
            "../../resources/template/amario.mjml",
            "format-json",
        ]);
    }

    #[test]
    fn format_json_pretty_amario() {
        execute(vec![
            "mrml-cli",
            "../../resources/template/amario.mjml",
            "format-json",
            "--pretty",
        ]);
    }

    #[test]
    fn format_mjml_amario() {
        execute(vec![
            "mrml-cli",
            "../../resources/template/amario.json",
            "format-mjml",
        ]);
    }

    #[test]
    fn format_mjml_pretty_amario() {
        execute(vec![
            "mrml-cli",
            "../../resources/template/amario.json",
            "format-mjml",
            "--pretty",
        ]);
    }

    #[test]
    fn render_amario() {
        execute(vec![
            "mrml-cli",
            "../../resources/template/amario.mjml",
            "render",
        ]);
    }

    #[test]
    fn validate_amario_json() {
        execute(vec![
            "mrml-cli",
            "../../resources/template/amario.json",
            "validate",
        ]);
    }

    #[test]
    fn validate_amario_mjml() {
        execute(vec![
            "mrml-cli",
            "../../resources/template/amario.mjml",
            "validate",
        ]);
    }
}
