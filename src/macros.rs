#[cfg(feature = "print")]
mod print {
    #[macro_export]
    macro_rules! print_display {
        ($structure:ident) => {
            use std::fmt;

            impl fmt::Display for $structure {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str(self.dense_print().as_str())
                }
            }
        };
    }
}
