pub extern crate regex;
pub extern crate rand;

/// Macro that UwUizes a String
///
/// # Examples
///
/// ```
/// use uwuizer::*;
/// let uwuized = uwuize!("Example text");
/// ```
#[macro_export]
macro_rules! uwuize {
    ($x:expr) => {{
        use regex::Regex;
        use std::str::FromStr;
        use rand::seq::IteratorRandom;

        let mut uwuized: String = String::from_str($x).unwrap();
        let mut rng = rand::thread_rng();
        [
            ("(?:r|l)", "w"),
            ("(?:R|L)", "W"),
            ("n([aeiou])", "ny$1"),
            ("N([aeiou])", "Ny$1"),
            ("N([AEIOU])", "NY$1"),
            ("ove", "uv"),
            ("!+", ["(・`ω´・)", "Uwu", "uwu", "oωo", "òωó", "°ω°", "UwU", ">w<", "^w^"].iter().choose(&mut rng).unwrap()),
        ].iter().for_each(|(f, t)| {
            let re = Regex::new(f).unwrap();
            uwuized = re.replace_all(&uwuized, *t).to_string();
        });
        uwuized
    }}
}

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
