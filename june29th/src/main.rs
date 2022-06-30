fn main() {
    println!("Hello, world!");
}
/* Compass prob */
use std::fs::File; //for i/o
use std::io::{BufRead, BufReader}; //for i/o
use derive_builder::Builder;
mod library;
//static strings (borrowed) keep their values btwn invocations
#[derive(Builder)]
struct Compass<'lt> {
   #[builder(default = vec![])]
  NtoS:Vec<&'lt str>,
   #[builder(default = vec![])]
  EtoW:Vec<&'lt str>
}
/*impl Default for Compass<'lt> {
  fn default() -> Compass<'lt> {
    NtoS:vec![],
    EtoW:vec![]
  }
}*/
impl<'lt> Compass<'lt> {
  
  fn add(&mut self, line:&str) {
    let dir = line.split(" ");
    println!("dir:{:?}", &dir);
    for d in dir {
      println!("{}",&d);
    }
  }
}

fn main() {
  let filename= "/home/lizz/summer2022/dailies/june29.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut lines = vec![];
  for (index,line) in reader.lines().enumerate() {
    let line = line.unwrap();
    lines.push(line);
  }
  //println!("l:{}, l[1]:{}", library::type_of(&lines),  library::type_of( library::type_of(&lines[0])))
  
  let mut myCompass =Compass::default(); 

}
   
