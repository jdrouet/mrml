//! Module containing a loader where all the possible files are stored on the
//! filesystem.

use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::Arc;

use super::loader::IncludeLoaderError;
#[cfg(feature = "async")]
use crate::prelude::parser::loader::AsyncIncludeLoader;
use crate::prelude::parser::loader::IncludeLoader;

#[derive(Debug)]
/// This struct is an
/// [`IncludeLoader`](crate::prelude::parser::loader::IncludeLoader) where
/// you can read a template for the filesystem and be able to use it with
/// [`mj-include`](crate::mj_include).
///
/// # Example
/// ```rust
/// use std::path::PathBuf;
/// use mrml::prelude::parser::local_loader::LocalIncludeLoader;
/// use mrml::prelude::parser::ParserOptions;
///
/// // relative to the crate root; resolved lazily so it can stay relative
/// let root = PathBuf::from("tests").join("resources");
/// let resolver = LocalIncludeLoader::new(root);
/// let opts = ParserOptions {
///     include_loader: Box::new(resolver),
/// };
/// let template = r#"<mjml>
///   <mj-body>
///     <mj-include path="file:///mj-text-hello-world.mjml" />
///   </mj-body>
/// </mjml>"#;
/// let output = mrml::parse_with_options(template, &opts)
///     .expect("template should load")
///     .element
///     .render(&Default::default())
///     .expect("template should render");
/// assert!(output.contains("Hello World"));
/// ```
///
/// About the security: the resolved file must be inside the root directory,
/// with any `..` components and symlinks resolved first on both sides of the
/// comparison. `root` may be relative or contain a symlinked ancestor; it is
/// canonicalized before every comparison.
pub struct LocalIncludeLoader {
    root: PathBuf,
}

impl LocalIncludeLoader {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn build_path(&self, url: &str) -> Result<PathBuf, IncludeLoaderError> {
        let root = self.root.canonicalize().map_err(|err| {
            IncludeLoaderError::new(url, err.kind())
                .with_message("unable to canonicalize the loader root directory")
                .with_cause(Arc::new(err))
        })?;
        let path = self.root.join(url.trim_start_matches("file:///"));
        path.canonicalize()
            .map_err(|err| IncludeLoaderError::new(url, err.kind()))
            .and_then(|path| {
                if path.starts_with(&root) {
                    Ok(path)
                } else {
                    Err(IncludeLoaderError::new(url, ErrorKind::NotFound))
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
            Self::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")))
        }
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
        // "src/../resources/..." resolves to a sibling of the "src" root, not
        // a descendant of it, so this must be rejected even though the file
        // itself exists on disk.
        let loader = LocalIncludeLoader::new(PathBuf::from("src"));

        let err = loader
            .build_path("file:///../resources/compare/success/mj-body.mjml")
            .unwrap_err();

        assert_eq!(err.reason, ErrorKind::NotFound);
    }

    #[test]
    fn should_resolve_relative_root() {
        // Regression test: with a relative root, build_path used to compare a
        // canonicalized (absolute) target against a non-canonical (relative)
        // root, so `starts_with` never matched and every legitimate path was
        // rejected.
        let loader = LocalIncludeLoader::new(PathBuf::from("tests").join("resources"));

        loader
            .build_path("file:///mj-text-hello-world.mjml")
            .expect("a legitimate file under a relative root should resolve");
    }

    #[test]
    fn should_block_traversal_with_relative_root() {
        let loader = LocalIncludeLoader::new(PathBuf::from("tests").join("resources"));

        let err = loader
            .build_path("file:///../../../../../../../../etc/hostname")
            .unwrap_err();

        assert_eq!(err.reason, ErrorKind::NotFound);
    }

    #[test]
    fn should_block_absolute_path_smuggling_with_relative_root() {
        // `PathBuf::join` with an absolute path discards the base entirely,
        // so a URL surviving `trim_start_matches("file:///")` as an absolute
        // path (e.g. a fourth leading slash) must still be rejected by the
        // containment check rather than silently resolving outside root.
        let loader = LocalIncludeLoader::new(PathBuf::from("tests").join("resources"));

        let err = loader.build_path("file:////etc/hostname").unwrap_err();

        assert_eq!(err.reason, ErrorKind::NotFound);
    }

    #[test]
    fn should_handle_dots_with_missing_file() {
        let loader = LocalIncludeLoader::new(PathBuf::default().join("src"));

        let err = loader.build_path("file:///../partial.mjml").unwrap_err();

        assert_eq!(err.reason, ErrorKind::NotFound);
        assert_eq!(err.to_string(), "file:///../partial.mjml entity not found (the path should stay in the context of the loader)");
    }

    #[test]
    fn should_resolve_path() {
        let loader = LocalIncludeLoader::current_dir();
        let _payload = loader
            .resolve("file:///resources/compare/success/mj-body.mjml")
            .unwrap();
    }
}
