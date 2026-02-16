use crate::Iter;

pub struct StringTools;

impl StringTools {
    pub fn starts(txt: &mut Iter, sub: &str) -> Result<String, String> {
        for c in sub.chars() {
            if let None = txt.chars.next_if(|x| *x == c) {
                return Err(format!("Expected '{sub}'"));
            }
        }
        Ok(sub.to_string())
    }

    pub fn identifier(txt: &mut Iter) -> Result<String, String> {
        let mut s: String = String::new();

        while let Some(c) = txt.chars.peek() {
            if c.is_alphanumeric() || *c == '_' {
                s.push(*c);
                txt.chars.next();
            } else {
                break;
            }
        }
        Ok(s)
    }
}
