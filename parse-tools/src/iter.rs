#[derive(Debug)]
pub struct Iter<'a> {
    pub chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Iter<'a> {
    pub fn from_string(s: &'a str) -> Self {
        Iter {
            chars: s.chars().peekable()
        }
    }

    pub fn to_string(&self) -> String {
        self.chars.clone().collect()
    }
}
