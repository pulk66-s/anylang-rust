#[derive(Debug, Clone)]
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

    pub fn is_empty(&self) -> bool {
        let mut temp = self.chars.clone();
        temp.peek().is_none()
    }

    pub fn peek(&self) -> Option<char> {
        let mut temp = self.chars.clone();
        temp.peek().copied()
    }

    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn starts_with(&self, s: &str) -> bool {
        let mut temp = self.chars.clone();
        for c in s.chars() {
            if temp.next() != Some(c) {
                return false;
            }
        }
        true
    }
}
