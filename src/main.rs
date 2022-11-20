#![allow(unused_parens)]

use std::{fs, io::{Write}, cmp::Ordering};

mod def_word;
use def_word::{WordStruct};


fn main()
    {
    println!("xdcbin");
    println!("powered with <3 by Rust");

    let file_path: &str = "test_cases/lorem_ipsum_100_paragraphs";
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
                charlen_4[index].add_find_location(index.try_into().unwrap());
                charlen_4[index].display();
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


    // write_to_file(charlen_4, "charlen4.txt");
    }

fn write_to_file(vec_slice: Vec<&str>, file_name: &str)
    {
    let mut fobj = fs::File::create(file_name).unwrap();

    let content = vec_slice.join(",");

    fobj.write_all(content.as_bytes()).unwrap();
    }
