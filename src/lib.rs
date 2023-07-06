
use std::fs;
use std::error::Error;

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    for line in  tests::search_sensitive(&config.query, &content){
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
   fn case_sensitve(){
      let query = "duct";
      let contents = "\
Rust:
safe, fast, productive
Pick three.";
      
      assert_eq!(vec!["safe, fast, productive"],search_sensitive(query,contents));
   }


#[test]
fn case_insensitve(){

  let query =  "many";
  let content = "\
Many more
Idk many
  ";
  assert_eq!(vec!["Many more","Idk many"],search_insensitive(query, content));

}


  pub fn search_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
      let mut found:Vec<&str> = Vec::new();
      for line in contents.lines(){
         if line.to_lowercase().contains(query){
             found.push(line);
         }
      }
      found
  }

   pub fn search_sensitive<'a>(query: &str,contents:&'a str)->Vec<&'a str>{
      let mut result:Vec<&str> = Vec::new();
      for line in contents.lines(){
         if line.contains(query){
            result.push(line);
         }
      }
        result
    }
 }
