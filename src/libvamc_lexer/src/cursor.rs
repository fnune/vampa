use std::{iter::Peekable, str::Chars};

pub struct Cursor<'a> {
    characters: Peekable<Chars<'a>>,
    position: usize,
}

const EOF_CHARACTER: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            characters: input.chars().peekable(),
            position: 0,
        }
    }

    pub fn offset(&self) -> usize {
        self.position
    }

    pub fn is_eof(&mut self) -> bool {
        self.characters.peek().is_none()
    }

    pub fn peek(&mut self) -> &char {
        self.characters.peek().unwrap_or(&EOF_CHARACTER)
    }

    pub fn bump(&mut self) -> Option<char> {
        let next = self.characters.next();
        if let Some(character) = next {
            self.position += character.len_utf8();
        }
        next
    }

    pub fn bump_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut consumed = String::new();

        while predicate(*self.peek()) && !self.is_eof() {
            consumed.push(*self.peek());
            self.bump();
        }

        consumed
    }
}
