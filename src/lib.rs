
use std::fs;
use std::error::Error;

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    for line in  tests::search(&config.query, &content){
    print!("{}\n",line);
    }
    Ok(())
 }
 
 
 pub struct Config{
 
 pub query:String,
 pub file_path:String,
 
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
 
 mod tests{

   #[test]
   fn one_result(){
      let query = "duct";
      let contents = "\
Rust:
safe, fast, productive
Pick three.";
      
      assert_eq!(vec!["safe, fast, productive"],search(query,contents));
   }
   pub fn search<'a>(query: &str,contents:&'a str)->Vec<&'a str>{
      let mut result:Vec<&str> = Vec::new();
      for line in contents.lines(){
         if line.contains(query){
            result.push(line);
         }
      }
        result
    }
 }
