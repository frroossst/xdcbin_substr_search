#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::def_word::{WordStruct};

use std::{io::Write, fs::{File, OpenOptions}, vec};
use std::fs;

pub fn create_file(file_name: &str)
    {
    let file = OpenOptions::new().write(true).create_new(true).open(file_name).unwrap();
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