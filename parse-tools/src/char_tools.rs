use crate::logic::LogicTools;
use crate::iter::Iter;

pub struct CharTools;

impl CharTools {
    pub fn parse_space(txt: &mut Iter) -> Result<char, String> {
        CharTools::parse_char(txt, ' ')
    }

    pub fn parse_comma(txt: &mut Iter) -> Result<char, String> {
        CharTools::parse_char(txt, ',')
    }

    pub fn parse_single_quote(txt: &mut Iter) -> Result<char, String> {
        CharTools::parse_char(txt, '\'')
    }

    pub fn parse_newline(txt: &mut Iter) -> Result<char, String> {
        CharTools::parse_char(txt, '\n')
    }

    pub fn parse_tab(txt: &mut Iter) -> Result<char, String> {
        CharTools::parse_char(txt, '\t')
    }

    pub fn parse_double_dot(txt: &mut Iter) -> Result<char, String> {
        CharTools::parse_char(txt, ':')
    }

    pub fn parse_semi_colon(txt: &mut Iter) -> Result<char, String> {
        CharTools::parse_char(txt, ';')
    }

    pub fn parse_pipe(txt: &mut Iter) -> Result<char, String> {
        CharTools::parse_char(txt, '|')
    }

    pub fn parse_spaces(txt: &mut Iter) -> Result<String, String> {
        LogicTools::many(
            txt, 
            |t| CharTools::parse_space(t)
                .or_else(|_| CharTools::parse_newline(t))
                .or_else(|_| CharTools::parse_tab(t))
        ).and_then(|v| Ok(v.iter().collect()))
    }

    pub fn parse_char(txt: &mut Iter, c: char) -> Result<char, String> {
        txt.chars.next_if(|x| *x == c).ok_or(format!("Expected '{}'", c))
    }
}
