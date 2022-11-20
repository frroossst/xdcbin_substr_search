use std::cmp::Ordering;

#[derive(Eq)]
pub struct WordStruct
    {
    pub word: String,
    pub found: Vec<u128>,
    }

impl WordStruct
    {
    pub fn new(word: &str) -> Self
        {
        WordStruct { word: String::from(word), found: Vec::new() }
        }

    pub fn add_find_location(&mut self, x: u128)
        {
        let _ = &self.found.push(x);
        }

    pub fn display(&self)
        {
        println!("word: {:?}, found: {:?}", &self.word, &self.found);
        }
    }

impl Ord for WordStruct
    {
    fn cmp(&self, other: &Self) -> Ordering
        {
        self.word.cmp(&other.word)
        }
    }

impl PartialOrd for WordStruct
    {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> 
        {
        Some(self.cmp(other))
        }
    }

impl PartialEq for WordStruct
    {
    fn eq(&self, other: &Self) -> bool 
        {
        self.word.to_string() == other.word.to_string()
        }
    }