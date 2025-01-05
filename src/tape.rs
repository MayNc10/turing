use std::{collections::HashSet, error::Error, fmt::{Debug, Display}};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim() == "L" {
            Ok(Direction::Left)
        }
        else if value.trim() == "R" {
            Ok(Direction::Right)
        }
        else {
            Err("\tdirection given was invalid, please reenter: ".into())
        }
    }
}

pub struct TapeTransition {
    pub write: Option<char>,
    pub direction: Direction,
    pub disp_string: String, // this is kind of a lol but whatever
}
impl TapeTransition {
    // we're passed an array of space seperated terms
    pub fn from_edge<'a, I: Iterator<Item = &'a str>>(s_it: I) 
    -> Result<(TapeTransition, HashSet<char>), Box<dyn Error>> 
{
        // format looks like [, label, =, "${char*}, $arrow, {$write?,dir}", ]
        let mut s_it = s_it.skip(3);
        let key_char = s_it.next().ok_or("missing inputs")?[1..].split(",")
            .map(|s| s.chars().next());
        let end = s_it.skip(1).next().ok_or("missing direction/write")?;
        println!("end: {end}");
        let end_phrase = &end[0..end.len() - 1];
        let mut split = end_phrase.split(",");
        let (write, dir_str) = 
            if end_phrase.contains(",") {
                if let Some(c) = split.next().map(|s| s.chars().next()) {
                    (Some(c.ok_or("write has no char")?), &split.next().ok_or("missing dir")?[0..1])
                }
                else {
                    Err("Unexpected comma")?
                }
            } else { (None, &end_phrase[0..1] )
        };
        let direction = Direction::try_from(dir_str)?;
        let keymap = key_char.collect::<Option<HashSet<char>>>().ok_or("missing chars between commas")?;
        let disp_string = TapeTransition::make_disp_str(
            keymap.iter().map(|c| if *c == '\u{2294}' {&' '} else {c}), 
            write, direction);
        
        let tape = TapeTransition { write, direction, disp_string };
        
        Ok((tape, keymap))
    }

    pub fn make_disp_str<'a, I: Iterator<Item = &'a char>>(inputs: I, write: Option<char>, dir: Direction) -> String {
        let mut inputs = inputs.peekable();
        let mut s = String::new();
        while let Some(ch) = inputs.next() {
            s.push(if *ch == ' ' {
                '\u{2294}'
            } else {*ch} );
            if inputs.peek().is_some() {
                s.push(',');
            }
        }
        s.push_str(" \u{2192} ");
        if let Some(write_c) = write {
            s.push(write_c);
            s.push(',');
        }
        if dir == Direction::Left {
            s.push('L');
        }
        else {
            s.push('R');
        }
        s
    }
}

impl Display for TapeTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.disp_string)
    }
}

impl Debug for TapeTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
