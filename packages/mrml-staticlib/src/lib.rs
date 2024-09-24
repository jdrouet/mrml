use std::ffi::CStr;
use std::os::raw::c_char;

/// Structure representing a vector
#[repr(C)]
pub struct Slice<T> {
    pointer: *const T,
    length: usize,
}

impl<T> Slice<T> {
    pub fn from_vec(mut list: Vec<T>) -> Self {
        list.shrink_to_fit();
        let mut boxed_slice: Box<[T]> = list.into_boxed_slice();
        let pointer: *mut T = boxed_slice.as_mut_ptr();
        let length: usize = boxed_slice.len();
        std::mem::forget(boxed_slice);
        Self { pointer, length }
    }
}

/// Structure representing a string
#[repr(C)]
pub struct SliceCChar {
    pointer: *const c_char,
    length: usize,
}

impl SliceCChar {
    #[inline(always)]
    pub fn from_static_str(input: &'static str) -> Self {
        Self {
            pointer: input.as_ptr() as *const c_char,
            length: input.len(),
        }
    }

    pub fn from_string(input: String) -> Self {
        let result = Self {
            pointer: input.as_ptr() as *const c_char,
            length: input.len(),
        };
        std::mem::forget(input);
        result
    }
}

#[repr(C)]
pub struct Span {
    start: usize,
    end: usize,
}

#[repr(C)]
pub enum Origin {
    Root,
    Include { path: SliceCChar },
}

impl From<mrml::prelude::parser::Origin> for Origin {
    fn from(value: mrml::prelude::parser::Origin) -> Self {
        match value {
            mrml::prelude::parser::Origin::Root => Self::Root,
            mrml::prelude::parser::Origin::Include { path } => Self::Include {
                path: SliceCChar::from_string(path),
            },
        }
    }
}

/// Structure representing a warning that could be raised when parsing a template
#[repr(C)]
pub struct Warning {
    /// A basic text description of the warning
    kind: SliceCChar,
    /// The origin of the template, could be the root template or an include
    origin: Origin,
    /// Location in the template when this warning happens
    span: Span,
}

impl From<mrml::prelude::parser::Warning> for Warning {
    fn from(value: mrml::prelude::parser::Warning) -> Self {
        Self {
            kind: SliceCChar::from_static_str(value.kind.as_str()),
            origin: Origin::from(value.origin),
            span: Span {
                start: value.span.start,
                end: value.span.end,
            },
        }
    }
}

#[repr(C)]
pub struct Success {
    output: SliceCChar,
    warnings: Slice<Warning>,
}

#[repr(C)]
pub struct Error {
    message: SliceCChar,
}

const NULL_POINTER_ERROR: &str = "provided null pointer";

impl Error {
    fn null_pointer() -> Self {
        Self {
            message: SliceCChar::from_static_str(NULL_POINTER_ERROR),
        }
    }
}

#[repr(C)]
pub enum Result {
    Ok(Success),
    Err(Error),
}

#[no_mangle]
pub extern "C" fn render(pointer: *const c_char) -> Result {
    if pointer.is_null() {
        return Result::Err(Error::null_pointer());
    }

    let input = unsafe { CStr::from_ptr(pointer).to_string_lossy() };
    let parsed = match mrml::parse(input) {
        Ok(res) => res,
        Err(err) => {
            return Result::Err(Error {
                message: SliceCChar::from_string(err.to_string()),
            })
        }
    };
    let render_opts = mrml::prelude::render::RenderOptions::default();
    let output = match parsed.element.render(&render_opts) {
        Ok(res) => res,
        Err(err) => {
            return Result::Err(Error {
                message: SliceCChar::from_string(err.to_string()),
            })
        }
    };
    let warnings = Slice::from_vec(
        parsed
            .warnings
            .into_iter()
            .map(Warning::from)
            .collect::<Vec<_>>(),
    );
    let output = SliceCChar::from_string(output);
    Result::Ok(Success { output, warnings })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! str_to_c_char {
        ($input:expr) => {{
            ::std::ffi::CString::new($input).unwrap()
        }};
    }

    macro_rules! slice_c_char_to_str {
        ($input:ident) => {
            unsafe {
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(::std::slice::from_raw_parts(
                    $input.pointer as *const u8,
                    $input.length + 1,
                ))
                .to_str()
                .unwrap()
            }
        };
    }

    impl super::SliceCChar {
        fn as_str(&self) -> &str {
            unsafe {
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(::std::slice::from_raw_parts(
                    self.pointer as *const u8,
                    self.length + 1,
                ))
                .to_str()
                .unwrap()
            }
        }
    }

    impl super::Result {
        fn assert_ok(self) -> Success {
            match self {
                Self::Ok(inner) => inner,
                Self::Err(error) => {
                    let Error { message } = error;
                    let message = slice_c_char_to_str!(message);
                    panic!("unexpected error: {message:?}");
                }
            }
        }
    }

    impl super::Success {
        fn warnings(&self) -> &[Warning] {
            unsafe { std::slice::from_raw_parts(self.warnings.pointer, self.warnings.length) }
        }
    }

    #[test]
    fn should_render() {
        let input =
            str_to_c_char!("<mjml><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>");
        let output = render(input.as_ptr());
        let success = output.assert_ok();
        let output = success.output.as_str();
        assert!(output.starts_with("<!doctype html><html"), "{output:?}");
        let warnings = success.warnings();
        assert!(warnings.is_empty());
    }

    #[test]
    fn should_render_with_warnings() {
        let input = str_to_c_char!(
            "<mjml whatever=\"foo\"><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>"
        );
        let output = render(input.as_ptr());
        let success = output.assert_ok();
        let output = success.output.as_str();
        assert!(output.starts_with("<!doctype html><html"), "{output:?}");
        let warnings = success.warnings();
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].kind.as_str(), "unexpected-attribute");
        assert_eq!(warnings[0].span.start, 6);
        assert_eq!(warnings[0].span.end, 20);
    }
}
