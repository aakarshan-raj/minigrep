use std::env;
use std::process;
use minigrep::Config;


fn main(){

let vector:Vec<String> = env::args().collect();
let config = Config::build(&vector).unwrap_or_else(|err|{
        print!("{err}");
        process::exit(1);
 });
  minigrep::run(config);
 	

}
