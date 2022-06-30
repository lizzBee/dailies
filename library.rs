use std::any::type_name;

pub fn type_of<T>(_:T)-> &'static str {
  type_name::<T>()
}

fn main() {
  let x = 21;
  let y = "skrrt";
  println!("{}-{}",x, type_of(x));
  println!("{}-{}",y, type_of(y));
}
