#![allow(unused_parens)]

use std::{fs, io::{Write}, ops::RangeFrom, cmp::Ordering};

#[derive(Eq)]
struct WordStruct
    {
    word: String,
    found: Vec<u128>,
    }

impl WordStruct
    {
    fn new(word: &str) -> Self
        {
        WordStruct { word: String::from(word), found: Vec::new() }
        }

    fn add_find_location(&mut self, x: u128)
        {
        let _ = &self.found.push(x);
        }

    fn display(&self)
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


fn main()
    {
    println!("xdcbin");
    println!("powered with <3 by Rust");

    let mut ws = WordStruct::new("lorem");
    ws.add_find_location(0);
    ws.add_find_location(1);
    ws.add_find_location(10);
    ws.display();

    let file_path: &str = "test_cases/lorem_ipsum_100_paragraphs";
    let content: String = fs::read_to_string(file_path).unwrap();
    let words = content.split(" ");

    
    let mut charlen_4: Vec<WordStruct> = Vec::new();

    let iter: RangeFrom<u128> = 1..;

    for i in words
        {
        if (i.len() == 4)
            {
            let mut ws = WordStruct::new(i);
            charlen_4.push(ws);
            }
        }

    charlen_4.sort_unstable();

    for j in charlen_4.iter()
        {
        println!("{:?}", j.display());
        }


    // write_to_file(charlen_4, "charlen4.txt");
    }

fn write_to_file(vec_slice: Vec<&str>, file_name: &str)
    {
    let mut fobj = fs::File::create(file_name).unwrap();

    let content = vec_slice.join(",");

    fobj.write_all(content.as_bytes()).unwrap();
    }
