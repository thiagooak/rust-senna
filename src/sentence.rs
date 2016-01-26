//! Contains data structures for the representation of a sentence
//! which can be tokenized, POS tagged, etc.



pub struct Word<'t> {
    offset_start : usize,
    offset_end : usize,
    string: &'t str,
    pos: String,
}

impl<'t> Word<'t> {
    pub fn new(offset_start: usize, offset_end: usize, string: &'t str) -> Word<'t> {
        Word {
            offset_start: offset_start,
            offset_end: offset_end,
            string: string,
            pos: String::new(),
        }
    }

    pub fn set_pos(&mut self, tag: &str) {
        self.pos = tag.to_string();
    }

    pub fn get_pos(&self) -> &str {
        &self.pos
    }

    pub fn get_string(&self) -> &str {
        self.string
    }

    pub fn get_offset_start(&self) -> usize {
        self.offset_start
    }

    pub fn get_offset_end(&self) -> usize {
        self.offset_end
    }
}





pub struct Sentence<'t> {
    string: &'t str,
    words: Vec<Word<'t>>,
}

impl<'t> Sentence<'t> {
    pub fn new(string: &'t str, words: Vec<Word<'t>>) -> Sentence<'t> {
        Sentence {
            string: string,
            words: words,
        }
    }
    pub fn get_words(&self) -> &Vec<Word> {
        &self.words
    }

    pub fn get_string(&self) -> &str {
        self.string
    }
}
