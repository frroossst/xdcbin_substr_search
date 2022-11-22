#![allow(unused_parens)]

// modules

// custom imports
use xdcbin_substr::def_word::{WordStruct};
use xdcbin_substr::file_io::{write_to_file, create_file, read_from_file};

// std imports
use std::{fs, cmp::Ordering};




fn main()
    {
    println!("xdcbin");
    println!("powered with <3 by Rust");

    // index("test_cases/lorem_ipsum_100_paragraphs");
    search("amet");
    }

fn index(file_path: &str)
    {
    let content: String = fs::read_to_string(file_path).unwrap();
    let words: Vec<&str> = content.split(" ").collect();
    
    let mut charlen_4: Vec<WordStruct> = Vec::new();

    for (x, i) in words.iter().enumerate()
        {
        if (i.len() == 4)
            {
            let mut ws = WordStruct::new(i);

            if charlen_4.contains(&ws)
                {
                let index = charlen_4.iter().position(|ws| ws.cmp(&WordStruct::new(i)) == Ordering::Equal).unwrap();
                charlen_4[index].add_find_location(x.try_into().unwrap());
                }
            else
                {
                ws.add_find_location(x.try_into().unwrap());
                charlen_4.push(ws);
                }
            }
        }

    charlen_4.sort_unstable();

    for j in charlen_4.iter()
        {
        j.display();
        }

    create_file("charlen4.txt");
    write_to_file(charlen_4, "charlen4.txt");
    }

fn search(search_term: &str)
    {
    let len: u32 = search_term.len().try_into().unwrap();
    let file_name_constructed = "charlen".to_string() + &len.to_string().as_str().to_owned() + ".txt";

    let content: Vec<WordStruct> = read_from_file(file_name_constructed.as_str());

    println!("{:?}", content.len());
    }