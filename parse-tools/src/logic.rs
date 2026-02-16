use crate::Iter;

pub struct LogicTools;

impl LogicTools {
    pub fn many<T>(
        txt: &mut Iter,
        f: fn(txt: &mut Iter) -> Result<T, String>,
    ) -> Result<Vec<T>, String> {
        let mut res = Vec::new();

        loop {
            match f(txt) {
                Ok(r) => res.push(r),
                _ => break,
            };
        }
        Ok(res)
    }
}
