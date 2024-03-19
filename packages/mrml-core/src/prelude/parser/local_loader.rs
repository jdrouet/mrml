//! Module containing a loader where all the possible files are stored on the
//! filesystem.

use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::Arc;

use super::loader::IncludeLoaderError;
#[cfg(feature = "async")]
use crate::prelude::parser::loader::AsyncIncludeLoader;
use crate::prelude::parser::loader::IncludeLoader;

#[derive(Debug, Default)]
/// This struct is an
/// [`IncludeLoader`](crate::prelude::parser::loader::IncludeLoader) where
/// you can read a template for the filesystem and be able to use it with
/// [`mj-include`](crate::mj_include).
///
/// # Example
/// ```rust
/// use std::path::PathBuf;
/// use mrml::mj_include::body::MjIncludeBodyKind;
/// use mrml::prelude::parser::local_loader::LocalIncludeLoader;
/// use mrml::prelude::parser::ParserOptions;
///
/// let root = PathBuf::default()
///     .join("resources")
///     .join("compare")
///     .join("success");
/// let resolver = LocalIncludeLoader::new(root);
/// let opts = ParserOptions {
///     include_loader: Box::new(resolver),
/// };
/// let template = r#"<mjml>
///   <mj-body>
///     <mj-include path="file:///mj-accordion.mjml" />
///   </mj-body>
/// </mjml>"#;
/// match mrml::parse_with_options(template, &opts) {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Couldn't parse template: {err:?}"),
/// }
/// ```
///
/// About the security: this loader doesn't allow to go fetch a template that
/// is in a parent directory of the root directory.
pub struct LocalIncludeLoader {
    root: PathBuf,
}

impl LocalIncludeLoader {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn build_path(&self, url: &str) -> Result<PathBuf, IncludeLoaderError> {
        let path = url
            .strip_prefix("file:///")
            .map(|p| self.root.join(p))
            .ok_or_else(|| {
                IncludeLoaderError::new(url, ErrorKind::InvalidInput)
                    .with_message("the path should start with file:///")
            })?;
        path.canonicalize()
            .map_err(|err| IncludeLoaderError::new(url, err.kind()))
            .and_then(|path| {
                if !path.starts_with(&self.root) {
                    Err(IncludeLoaderError::new(url, ErrorKind::NotFound))
                } else {
                    Ok(path)
                }
            })
            .map_err(|err| err.with_message("the path should stay in the context of the loader"))
    }
}

impl IncludeLoader for LocalIncludeLoader {
    fn resolve(&self, url: &str) -> Result<String, IncludeLoaderError> {
        let path = self.build_path(url)?;
        std::fs::read_to_string(path).map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::InvalidData)
                .with_message("unable to load the template file")
                .with_cause(Arc::new(err))
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncIncludeLoader for LocalIncludeLoader {
    async fn async_resolve(&self, url: &str) -> Result<String, IncludeLoaderError> {
        let path = self.build_path(url)?;
        std::fs::read_to_string(path).map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::InvalidData)
                .with_message("unable to load the template file")
                .with_cause(Arc::new(err))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use std::path::PathBuf;

    use super::LocalIncludeLoader;
    use crate::prelude::parser::loader::IncludeLoader;

    impl LocalIncludeLoader {
        fn current_dir() -> Self {
            Self::new(PathBuf::from(std::env::var("PWD").unwrap()))
        }
    }

    #[test]
    fn should_start_with_file() {
        let loader = LocalIncludeLoader::default();
        let err = loader
            .build_path("/resources/compare/success/mj-body.mjml")
            .unwrap_err();

        assert_eq!(err.reason, ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "Unable to load template /resources/compare/success/mj-body.mjml: the path should start with file:/// (invalid input parameter)");
    }

    #[test]
    fn should_turn_into_path() {
        let loader = LocalIncludeLoader::current_dir();
        let path = loader
            .build_path("file:///resources/compare/success/mj-body.mjml")
            .unwrap();

        assert_eq!(
            path.as_os_str(),
            format!(
                "{}/resources/compare/success/mj-body.mjml",
                loader.root.to_string_lossy()
            )
            .as_str()
        );
    }

    #[test]
    fn should_handle_dots_with_existing_file() {
        let loader = LocalIncludeLoader::new(PathBuf::default().join("src"));

        let err = loader
            .build_path("file:///../resources/compare/success/mj-body.mjml")
            .unwrap_err();

        assert_eq!(err.reason, ErrorKind::NotFound);
    }

    #[test]
    fn should_handle_dots_with_missing_file() {
        let loader = LocalIncludeLoader::new(PathBuf::default().join("src"));

        let err = loader.build_path("file:///../partial.mjml").unwrap_err();

        assert_eq!(err.reason, ErrorKind::NotFound);
        assert_eq!(err.to_string(), "Unable to load template file:///../partial.mjml: the path should stay in the context of the loader (entity not found)");
    }

    #[test]
    fn should_resolve_path() {
        let loader = LocalIncludeLoader::current_dir();
        let _payload = loader
            .resolve("file:///resources/compare/success/mj-body.mjml")
            .unwrap();
    }
}
