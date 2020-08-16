use std::iter::Peekable;
use std::str::Chars;

pub struct Cursor<'a> {
    characters: Peekable<Chars<'a>>,
}

const EOF_CHARACTER: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            characters: input.chars().peekable(),
        }
    }

    pub fn is_eof(&mut self) -> bool {
        self.characters.peek() == None
    }

    pub fn peek(&mut self) -> &char {
        self.characters.peek().unwrap_or(&EOF_CHARACTER)
    }

    pub fn bump(&mut self) -> Option<char> {
        self.characters.next()
    }

    pub fn bump_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut consumed = String::new();

        while predicate(self.peek().clone()) && !self.is_eof() {
            consumed.push(self.peek().clone());
            self.bump();
        }

        consumed
    }
}
