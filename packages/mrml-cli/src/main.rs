use std::borrow::Cow;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use mrml::mjml::Mjml;
use mrml::prelude::parser::http_loader::{HttpIncludeLoader, UreqFetcher};
use mrml::prelude::parser::loader::IncludeLoader;
use mrml::prelude::parser::local_loader::LocalIncludeLoader;
use mrml::prelude::parser::multi_loader::MultiIncludeLoader;
use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
use mrml::prelude::parser::{Error as ParserError, ParseOutput, ParserOptions};
use mrml::prelude::print::Printable;
use mrml::prelude::render::RenderOptions;

fn format_parser_error(error: ParserError) -> String {
    if let Some(src) = error.source() {
        format!("{error}: {src}")
    } else {
        format!("{error}")
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum HttpLoaderMode {
    Allow,
    Deny,
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Options {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
    /// Path to your mjml file
    #[clap(index = 1)]
    pub input: Option<String>,
    /// Path to a directory containing templates that can be used with
    /// mj-include
    #[clap(long)]
    pub local_loader: Option<PathBuf>,
    #[clap(long, action = clap::ArgAction::Append)]
    pub http_loader: Vec<String>,
    #[clap(long)]
    pub http_loader_mode: Option<HttpLoaderMode>,
}

impl Options {
    fn read_file(&self, filename: &str) -> Result<String, String> {
        log::debug!("reading from file {}", filename);
        let mut file =
            File::open(filename).map_err(|err| format!("couldn't open {filename:?}: {err}"))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|err| format!("couldn't read {filename:?}: {err}"))?;
        Ok(content)
    }

    fn read_stdin(&self) -> Result<String, String> {
        log::info!("waiting for input...");
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|err| format!("couldn't read stdin: {err}"))?;
        Ok(buffer)
    }

    fn parse_json(&self, input: &str) -> Result<Mjml, String> {
        log::debug!("parsing json input");
        serde_json::from_str::<Mjml>(input)
            .map_err(|err| format!("unable to parse json input: {err:?}"))
    }

    fn http_include_loader(&self) -> Box<dyn IncludeLoader> {
        let list = HashSet::from_iter(self.http_loader.iter().cloned());
        match self.http_loader_mode {
            Some(HttpLoaderMode::Deny) => {
                Box::new(HttpIncludeLoader::<UreqFetcher>::new_deny(list))
            }
            _ => Box::new(HttpIncludeLoader::<UreqFetcher>::new_allow(list)),
        }
    }

    fn local_include_loader(&self) -> Result<Option<Box<dyn IncludeLoader>>, String> {
        Ok(match self.local_loader {
            Some(ref path) => {
                let path = if path.is_absolute() {
                    path.to_path_buf()
                } else {
                    std::env::current_dir()
                        .map_err(|err| format!("unable to detect current directory: {err:?}"))?
                        .join(path)
                };
                Some(Box::new(LocalIncludeLoader::new(path)))
            }
            None => None,
        })
    }

    fn include_loader(&self) -> Result<Box<dyn IncludeLoader>, String> {
        Ok(match self.local_include_loader()? {
            Some(local) => Box::new(
                MultiIncludeLoader::new()
                    .with_starts_with("file://", local)
                    .with_starts_with("http://", self.http_include_loader())
                    .with_starts_with("https://", self.http_include_loader())
                    .with_any(Box::<NoopIncludeLoader>::default()),
            ),
            None => self.http_include_loader(),
        })
    }

    fn parse_mjml(&self, input: &str) -> Result<ParseOutput<Mjml>, String> {
        log::debug!("parsing mjml input");
        let options = ParserOptions {
            include_loader: self.include_loader()?,
        };
        Mjml::parse_with_options(input, &options).map_err(format_parser_error)
    }

    fn parse_input(&self, input: String) -> Result<ParseOutput<Mjml>, String> {
        if let Some(ref filename) = self.input {
            if filename.ends_with(".json") {
                self.parse_json(&input).map(|element| ParseOutput {
                    element,
                    warnings: Vec::new(),
                })
            } else if filename.ends_with(".mjml") {
                self.parse_mjml(&input)
            } else {
                Err(format!("unable to detect file type for {filename:?}"))
            }
        } else {
            self.parse_mjml(&input).or_else(|_| {
                self.parse_json(&input).map(|element| ParseOutput {
                    element,
                    warnings: Vec::new(),
                })
            })
        }
    }

    fn read_input(&self) -> Result<String, String> {
        if let Some(ref filename) = self.input {
            self.read_file(filename)
        } else {
            self.read_stdin()
        }
    }

    pub fn execute(self) -> Result<(), String> {
        let root = self.read_input()?;
        let root = self.parse_input(root)?;

        self.subcmd.execute(root)
    }
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// Format template to JSON
    FormatJSON(Format),
    /// Format template to Mjml
    FormatMjml(Format),
    /// Render template to HTML
    Render(Render),
    /// Read input file and validate its structure
    Validate,
}

impl SubCommand {
    pub fn execute(self, root: ParseOutput<Mjml>) -> Result<(), String> {
        match self {
            Self::FormatJSON(opts) => {
                log::debug!("format to json");
                let output = if opts.pretty {
                    serde_json::to_string_pretty(&root.element).expect("couldn't format to JSON")
                } else {
                    serde_json::to_string(&root.element).expect("couldn't format to JSON")
                };
                println!("{}", output);
            }
            Self::FormatMjml(opts) => {
                log::debug!("format to mjml");
                let output = if opts.pretty {
                    root.element.print_pretty()
                } else {
                    root.element.print_dense()
                }
                .expect("couldn't format mjml");
                println!("{}", output);
            }
            Self::Render(render) => {
                log::debug!("render");
                let render_opts = RenderOptions::from(render);
                let output = root
                    .element
                    .render(&render_opts)
                    .expect("couldn't render template");
                println!("{}", output);
            }
            Self::Validate => {
                log::debug!("validate");
                for warning in root.warnings {
                    log::warn!("{warning}");
                }
            }
        };
        Ok(())
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
            social_icon_origin: value.social_icon_origin.map(Cow::Owned),
            ..Default::default()
        }
    }
}

fn main() {
    env_logger::init();
    if let Err(error) = Options::parse().execute() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::format_parser_error;

    use super::Options;
    use mrml::prelude::parser::{loader::IncludeLoaderError, Error as ParserError, Origin, Span};

    fn origin_include() -> Origin {
        Origin::Include {
            path: String::from("foo.mjml"),
        }
    }

    const fn any_span() -> Span {
        Span { start: 10, end: 20 }
    }

    #[test]
    fn format_parser_error_end_of_stream_in_root() {
        assert_eq!(
            format_parser_error(ParserError::EndOfStream {
                origin: Origin::Root
            }),
            "unexpected end of stream in root template"
        );
    }

    #[test]
    fn format_parser_error_end_of_stream_in_include() {
        assert_eq!(
            format_parser_error(ParserError::EndOfStream {
                origin: origin_include()
            }),
            "unexpected end of stream in template from \"foo.mjml\""
        );
    }

    #[test]
    fn format_parser_error_unexpected_element_in_root() {
        assert_eq!(
            format_parser_error(ParserError::UnexpectedElement {
                origin: Origin::Root,
                position: any_span()
            }),
            "unexpected element in root template at position 10..20"
        );
    }

    #[test]
    fn format_parser_error_unexpected_element_in_include() {
        assert_eq!(
            format_parser_error(ParserError::UnexpectedElement {
                origin: origin_include(),
                position: any_span()
            }),
            "unexpected element in template from \"foo.mjml\" at position 10..20"
        );
    }

    #[test]
    fn format_parser_error_invalid_attribute_in_root() {
        assert_eq!(
            format_parser_error(ParserError::InvalidAttribute {
                origin: Origin::Root,
                position: any_span()
            }),
            "invalid attribute in root template at position 10..20"
        );
    }

    #[test]
    fn format_parser_error_invalid_attribute_in_include() {
        assert_eq!(
            format_parser_error(ParserError::InvalidAttribute {
                origin: origin_include(),
                position: any_span()
            }),
            "invalid attribute in template from \"foo.mjml\" at position 10..20"
        );
    }

    #[test]
    fn format_parser_error_invalid_format_in_root() {
        assert_eq!(
            format_parser_error(ParserError::InvalidFormat {
                origin: Origin::Root,
                position: any_span()
            }),
            "invalid format in root template at position 10..20"
        );
    }

    #[test]
    fn format_parser_error_invalid_format_in_include() {
        assert_eq!(
            format_parser_error(ParserError::InvalidFormat {
                origin: origin_include(),
                position: any_span()
            }),
            "invalid format in template from \"foo.mjml\" at position 10..20"
        );
    }

    #[test]
    fn format_parser_error_include_loader_error_in_root() {
        assert_eq!(
            format_parser_error(ParserError::IncludeLoaderError {
                origin: Origin::Root,
                position: any_span(),
                source: IncludeLoaderError {
                    path: String::from("foo.mjml"),
                    reason: std::io::ErrorKind::NotFound,
                    message: None,
                    cause: None,
                }
            }),
            "unable to load included template in root template at position 10..20: foo.mjml entity not found"
        );
    }

    #[test]
    fn format_parser_error_include_loader_error_in_include() {
        assert_eq!(
            format_parser_error(ParserError::IncludeLoaderError {
                origin: Origin::Root,
                position: any_span(),
                source: IncludeLoaderError {
                    path: String::from("foo.mjml"),
                    reason: std::io::ErrorKind::NotFound,
                    message: None,
                    cause: None,
                }
            }),
            "unable to load included template in root template at position 10..20: foo.mjml entity not found"
        );
    }

    #[test]
    fn format_parser_error_missing_attribute_in_root() {
        assert_eq!(
            format_parser_error(ParserError::MissingAttribute {
                name: "name",
                origin: Origin::Root,
                position: any_span()
            }),
            "missing attribute \"name\" in element in root template at position 10..20"
        );
    }

    #[test]
    fn format_parser_error_missing_attribute_in_include() {
        assert_eq!(
            format_parser_error(ParserError::MissingAttribute {
                name: "name",
                origin: origin_include(),
                position: any_span()
            }),
            "missing attribute \"name\" in element in template from \"foo.mjml\" at position 10..20"
        );
    }

    #[test]
    fn format_parser_error_size_limit_in_root() {
        assert_eq!(
            format_parser_error(ParserError::SizeLimit {
                origin: Origin::Root
            }),
            "size limit reached in root template"
        );
    }

    #[test]
    fn format_parser_error_size_limit_in_include() {
        assert_eq!(
            format_parser_error(ParserError::SizeLimit {
                origin: origin_include()
            }),
            "size limit reached in template from \"foo.mjml\""
        );
    }

    fn execute<const N: usize>(args: [&str; N]) {
        Options::parse_from(args).execute().unwrap()
    }

    fn execute_stdin<const N: usize, I: Into<String>>(args: [&str; N], input: I) {
        let opts = Options::parse_from(args);
        let root = opts.parse_input(input.into()).unwrap();
        opts.subcmd.execute(root).unwrap()
    }

    #[test]
    #[should_panic]
    fn missing_file() {
        execute(["mrml-cli", "./cant/be/found.mjml", "validate"]);
    }

    #[test]
    #[should_panic]
    fn unknown_extension() {
        execute(["mrml-cli", "./cant/be/found.txt", "validate"]);
    }

    #[test]
    #[should_panic]
    fn unknown_extension_stdin() {
        execute_stdin(["mrml-cli", "validate"], "###");
    }

    #[test]
    fn format_json_amario_stdio() {
        let input = include_str!("../resources/amario.mjml");
        execute_stdin(["mrml-cli", "format-json"], input);
    }

    #[test]
    fn format_json_amario() {
        execute(["mrml-cli", "./resources/amario.mjml", "format-json"]);
    }

    #[test]
    fn format_json_pretty_amario() {
        execute([
            "mrml-cli",
            "./resources/amario.mjml",
            "format-json",
            "--pretty",
        ]);
    }

    #[test]
    fn format_mjml_amario() {
        execute(["mrml-cli", "./resources/amario.json", "format-mjml"]);
    }

    #[test]
    fn format_mjml_pretty_amario() {
        execute([
            "mrml-cli",
            "./resources/amario.json",
            "format-mjml",
            "--pretty",
        ]);
    }

    #[test]
    fn render_amario() {
        execute(["mrml-cli", "./resources/amario.mjml", "render"]);
    }

    #[test]
    fn validate_amario_json() {
        execute(["mrml-cli", "./resources/amario.json", "validate"]);
    }

    #[test]
    fn validate_amario_mjml() {
        execute(["mrml-cli", "./resources/amario.mjml", "validate"]);
    }

    #[test]
    fn render_with_multi_include() {
        execute([
            "mrml-cli",
            "--local-loader",
            "./resources/partials",
            "--http-loader",
            "https://gist.githubusercontent.com",
            "--http-loader-mode",
            "allow",
            "./resources/with-multi-include.mjml",
            "render",
        ]);
    }

    #[test]
    fn render_with_local_include() {
        execute([
            "mrml-cli",
            "--local-loader",
            "./resources/partials",
            "./resources/with-local-include.mjml",
            "render",
        ]);
    }

    #[test]
    fn render_with_http_include() {
        execute([
            "mrml-cli",
            "--http-loader",
            "https://gist.githubusercontent.com",
            "--http-loader-mode",
            "allow",
            "./resources/with-http-include.mjml",
            "render",
        ]);
    }

    #[test]
    fn render_with_http_include_with_multiple_values() {
        execute([
            "mrml-cli",
            "--http-loader",
            "https://github.com",
            "--http-loader",
            "https://gist.githubusercontent.com",
            "--http-loader",
            "https://whatever.com",
            "--http-loader-mode",
            "allow",
            "./resources/with-http-include.mjml",
            "render",
        ]);
    }

    #[test]
    #[should_panic]
    fn render_with_http_include_should_block_github() {
        execute([
            "mrml-cli",
            "--http-loader",
            "https://gist.githubusercontent.com",
            "--http-loader-mode",
            "deny",
            "./resources/with-http-include.mjml",
            "render",
        ]);
    }

    #[test]
    #[should_panic]
    fn render_with_http_include_block_everything_by_default() {
        execute(["mrml-cli", "./resources/with-http-include.mjml", "render"]);
    }

    #[test]
    fn render_with_http_include_allow_everything() {
        execute([
            "mrml-cli",
            "--http-loader-mode",
            "deny",
            "./resources/with-http-include.mjml",
            "render",
        ]);
    }
}
