// #[warn(missing_debug_implementations, missing_docs)]
// ...

// Structs
#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimeter: &'a str,
}

// Impls
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimeter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimeter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let next_delim = self.remainder.find(self.delimeter)?;
        let until_delim = &self.remainder[..next_delim];
        self.remainder = &self.remainder[next_delim + self.delimeter.len() - 1..];
        Some(until_delim)
    }
}
