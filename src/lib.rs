use rand::prelude::*;
use regex::Regex;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uwu() {
        let uwuized = uwuize!("malfunction me mom.. t-till i break~~");
        println!("\t{}", uwuized);
    }
    #[test]
    fn all_match_uwu() {
        let uwuized = uwuize!("r l R L na Na NA ove !!");
        println!("\t{}", uwuized);
    }
}

/// Macro that calls `fn uwuize()` and passes it to a variable.
#[macro_export]
macro_rules! uwuize {
    ($x:expr) => {{
        String::from($x).uwuize()
    }}
}

pub trait UwUizer {
    /// The uwuification method
    fn uwuize(&self) -> Self;
}

impl UwUizer for String {
    /// UwUizes a String
    ///
    /// # Examples
    ///
    /// ```
    /// use uwuizer::*;
    /// let uwuized = uwuize!("Example text");
    /// ```
    fn uwuize(&self) -> Self {
        let mut rng = rand::thread_rng();
        let faces = ["(・`ω´・)", "Uwu", "uwu", "oωo", "òωó", "°ω°", "UwU", ">w<", "^w^"];
        let face = &format!(" {} ", faces[rng.gen_range(0, faces.len())]).to_owned();
        let pats: Vec<(&str, &str)> = vec![
            ("(?:r|l)", "w"),
            ("(?:R|L)", "W"),
            ("n([aeiou])", "ny$1"),
            ("N([aeiou])", "Ny$1"),
            ("N([AEIOU])", "NY$1"),
            ("ove", "uv"),
            ("!+", face),
        ];

        let mut uwuized = String::from_str(&self).unwrap();

        for &(f, t) in &pats {
            let re = Regex::new(f).unwrap();
            uwuized = re.replace_all(&uwuized, t).to_string();
        }

        uwuized
    }
}
