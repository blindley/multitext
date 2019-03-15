#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let lines = [
            "these two lines", "should be ignored",
            "###multitext header", "mh line 1", "mh line 2", "mh line 3",
            "###first thing", "ft line 1", "ft line 2", "ft line 3", "ft line 4",
            "### second thing ", "st line 1", "     ", "st line 3",
        ];

        let mt = parse_lines(lines.iter()).unwrap();
        assert_eq!(mt.len(), 3);
        assert_eq!(mt["multitext header"], "mh line 1\nmh line 2\nmh line 3\n");
        assert_eq!(mt["first thing"], "ft line 1\nft line 2\nft line 3\nft line 4\n");
        assert_eq!(mt["second thing"], "st line 1\n     \nst line 3\n");
    }
}
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Debug, Clone)]
pub struct ParseError {
    line_number: usize,
    error_message: String,
    filename: Option<String>
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "multitext ParseError : {} : ", self.error_message)?;
        if let Some(filename) = &self.filename {
            write!(f, "{}", filename)?;
        }
        write!(f, "({})", self.line_number)?;

        Ok(())
    }
}

impl std::error::Error for ParseError{}

pub fn parse_lines<I>(mut it: I) -> Result<HashMap<String, String>, ParseError>
where I: Iterator, <I as Iterator>::Item: AsRef<str>
{
    let mut map = HashMap::new();
    let line_number = 0;
    let prefix = loop {
        let line = it.next().ok_or_else(|| {
            ParseError {
                line_number,
                error_message: "missing multitext header".to_string(),
                filename: None,
            }
        })?;

        if let Some(index) = line.as_ref().find("multitext header") {
            break line.as_ref().split_at(index).0.trim_end().to_string();
        }
    };

    let mut name = "multitext header".to_string();
    let mut text = String::new();
    for line in it {
        if line.as_ref().starts_with(&prefix) {
            map.insert(name.clone(), text.clone());
            name = line.as_ref().split_at(prefix.len()).1.trim().to_string();
            text = String::new();
        } else {
            text.push_str(line.as_ref());
            text.push('\n');
        }
    }

    map.insert(name, text);

    Ok(map)
}
