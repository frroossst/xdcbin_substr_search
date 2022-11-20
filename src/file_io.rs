use crate::def_word::{WordStruct};

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

    let struct_vec: Vec<WordStruct> = Vec::new();

    for i in lines 
        {
        let mut split: Vec<&str> = i.split(",").collect();
        let ws: WordStruct = WordStruct::new(split[0]);
        split.remove(0);
        
        let split_128: Vec<u128> = split.iter().map(|&e| e as u32).collect();

        ws.display();
        }

    return struct_vec;
    }