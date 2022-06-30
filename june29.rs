/* Compass prob */
use std::fs::File; //for i/o
use std::io::{BufRead, BufReader}; //for i/o
use regex::Regex;
//use derive_builder::Builder;
mod library;
//static strings (borrowed) keep their values btwn invocations
//#[derive(Builder)]
struct Compass<'lt> {
  // #[builder(default = vec![])]
  NtoS:Vec<&'lt str>,
  // #[builder(default = vec![])]
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
    /*for d in dir {
      println!("{}",&d);
    }*/
    let NtoS_a = self.Ntos.iter().position(|&x| x==&dir[0]);
    let EtoW_a = self.Etow.iter().position(|&x| x==&dir[0]);
    let NtoS_b = self.Ntos.iter().position(|&x| x==&dir[2]);
    let EtoW_b = self.Etow.iter().position(|&x| x==&dir[2]);
    let reNTS = Regex::new(r"[NS].*").unwrap();
    let reETW = Regex::new(r"[EW].*").unwrap();
    if reNTS.is_match(&dir[1]) {
      println!("north or south");
    }
    else if reETW.is_match(&dir[1]) {
      println!("east or west");
   } 
    /*match [] {
      "N" => { } 
    }*/
  }
}

fn main() {
  let filename= "/home/lizz/summer2022/dailies/june29.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut lines = vec![];
  let mut myCompass =Compass { ntos: vec::new(), etow : vec::new()};
  for (index,line) in reader.lines().enumerate() {
    let line = line.unwrap();
    Compass::add(line);
    lines.push(line);
  }
  //println!("l:{}, l[1]:{}", library::type_of(&lines),  library::type_of( library::type_of(&lines[0])))
  
  
}
   
