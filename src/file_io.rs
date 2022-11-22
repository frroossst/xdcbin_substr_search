use crate::def_word::{WordStruct};
use crate::helper_util::{convert_string_slice_to_u128};
use std::{io::{Write, Read}, fs::{OpenOptions}};

pub fn create_file(file_name: &str)
    {
    let _ = OpenOptions::new().write(true).create_new(true).open(file_name).unwrap();
    }

pub fn write_to_file(vec_slice: Vec<WordStruct>, file_name: &str)
    {
    let mut fobj = OpenOptions::new()
        .append(true)
        .open(file_name)
        .expect("unable to create file");

    for i in vec_slice
        {
        let mut data = i.word.as_str().to_owned() + ", ";
        for j in i.found.into_iter()
            {
            data = data + j.to_string().as_str() + ", ";
            }
        data = data + "\n";

        fobj.write_all(data.as_bytes()).expect("unable to write to file");
        }
    }

pub fn read_from_file(file_name: &str) -> Vec<WordStruct>
    {
    let mut fobj = OpenOptions::new().read(true).open(file_name).unwrap();

    let mut buffer = String::new();
    fobj.read_to_string(&mut buffer).unwrap();

    let lines = buffer.split("\n");

    let mut struct_vec: Vec<WordStruct> = Vec::new();

    for i in lines 
        {
        if !(i.is_empty() || i.contains(char::is_whitespace))
            {
            let mut split: Vec<&str> = i.split(",").collect();
            let word = split[0];
            split = split[1..split.len()-1].to_vec();
            let num_split = split.into_iter().map(|f| convert_string_slice_to_u128(f));

            let mut ws = WordStruct::new(word);

            for j in num_split
                {
                ws.add_find_location(j);
                }

            struct_vec.push(ws);
            }
        }
    return struct_vec;
    }