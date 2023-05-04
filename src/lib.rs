
use std::fs;
use std::error::Error;

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    println!("{content}");
    Ok(())
 }
 
 
 pub struct Config{
 
 query:String,
 file_path:String,
 
 }
 
 impl Config{
 
    pub fn build(vector:&Vec<String>)->Result<Config,&'static str>{
 
        if vector.len() < 3 {
          return Err("Argument error");
        }
        let query = vector[1].clone();
        let file_path = vector[2].clone();
        Ok(Config{query,file_path})
 
    }
 
 }
 