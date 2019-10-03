/*! ```text
This project contains a parser for a format I call Multitext.
Multitext is a simple text file format that allows you to store multiple text
files in one. Its original purpose was to allow all of the shader stages for an
OpenGL shader program to be stored in a single file, for easily switching
between them and making sure they match up. Of course, the format can be used
to store any kind of text file. The text in this documentation block is a valid
multitext file holding a vertex shader and a fragment shader, while also
describing the format.

@@@ multitext header
The parser looks for the first line that contains the string
"multitext header", as above. The initial characters on that same line are used
as a marker for indicating the start of each contained file. This allows you to
choose any sequence of characters that won't conflict with the format of the
files you are trying to store. The marker is trimmed for whitespace on the
right, but not the left. Note that the marker can appear in your files, as long
as it is not at the start a line. The marker I've chosen for this file is
"@@@", but you can choose any sequence of characters apart from the literal
string "multitext header".

Once the parser has identified the marker, it simply separates the file into a
set of strings, demarcated by lines starting with the marker. The remaining
text on the line that started with a marker is used (after being trimmed of
whitespace on the left and right) as a key in a hash map that matches that key
to the text that follows it until the next marker. The "multiline header" key
is included in this hash map.

If this file is parsed, will produce a hash map with 3 entries. The first entry
will have the key "multitext header", and will be matched to this text
describing the format. The second entry will have the key "vertex shader", and
will contain the glsl vertex shader code provided below. The final entry will
have the key "fragment shader", and contain the final bit of shader code. Any
text above the first marker line will be discarded.

@@@ vertex shader
#version 430 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
out vec3 v_color;
void main() {
    gl_Position = vec4(v_position, 1.0);
    v_color = color;
}

@@@ fragment shader
#version 430 core
in vec3 v_color;
out vec4 f_color;
void main() {
    f_color = vec4(v_color, 1.0);
}
```
*/

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

use std::iter::Iterator;

#[derive(Debug, Clone)]
pub struct Error {
    line_number: Option<usize>,
    filename: Option<String>,
    error_message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "multitext Error : {} : ", self.error_message)?;

        if let Some(line_number) = self.line_number {
            if let Some(filename) = &self.filename {
                write!(f, "{}", filename)?;
            }
            write!(f, "({})", line_number)?;
        } else {
            if let Some(filename) = &self.filename {
                write!(f, "{}", filename)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for Error{}

pub type Map = std::collections::HashMap<String, String>;
pub type ParseResult = Result<Map, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error {
            line_number: None,
            filename: None,
            error_message: format!("{}", e),
        }
    }
}

/// Parses lines from an iterator
/// 
/// # Examples
/// ```
/// let lines = [
///     "This line is ignored",
///     "$$ multitext header",
///     "$$ fox",
///     "The quick brown fox jumps over the lazy dog.",
///     "$$ lorem ipsum",
///     "Lorem ipsum dolor sit amet"
/// ];
/// 
/// let mt = multitext::parse_lines(lines.iter()).unwrap();
/// assert_eq!(mt["multitext header"].len(), 0);
/// assert_eq!(mt["fox"], "The quick brown fox jumps over the lazy dog.\n");
/// assert_eq!(mt["lorem ipsum"], "Lorem ipsum dolor sit amet\n")
/// ```
pub fn parse_lines<I>(mut it: I) -> ParseResult
where I: Iterator, <I as Iterator>::Item: AsRef<str>
{
    let mut map = Map::new();
    let mut line_number = 0;
    let prefix = loop {
        line_number += 1;
        let line = it.next().ok_or_else(|| {
            Error {
                line_number: Some(line_number),
                filename: None,
                error_message: "missing multitext header".to_string(),
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

/// Opens and parses a file stored in the multitext format
pub fn open_and_parse_file<P: AsRef<std::path::Path>>(path: P) -> ParseResult {
    use std::io::BufRead;
    let file = std::fs::File::open(path.as_ref())?;
    let file = std::io::BufReader::new(file);
    parse_lines(file.lines().filter_map(|s| s.ok())).or_else(|mut e| {
        e.filename = Some(path.as_ref().to_str().unwrap().to_string());
        Err(e)
    })
}