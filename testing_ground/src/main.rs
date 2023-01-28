// enum Location {
//     Point(i32),
//     Range(i32, i32)
// }

// fn main() {
//     let l: Location = Location::Range(0 ,5);
//     let n = match l {
//         Location::Point(_) => -1,
//         Location::Range(_, n) => n,
//         Location::Range(0, _) => 0,
//         _ => -2
//     };
//     println!("{n}");
// }


// #[derive(Debug)]

// enum Either {

//   Left(usize),

//   Right(String)

// }


// fn main() {

//   let x = Either::Right(String::from("Hello world"));

//   let value = match x {

//     Either::Left(n) => n,

//     Either::Right(s) => s.len()

//   };

//   println!("{x:?} {value}");

// }


fn main() {
  // get_or_default("Demo");
}

// fn make_separator(user_str: &str) -> &str {
//   if user_str == "" {
//     let default = "=".repeat(10);
//     &default
//   }
//   else{
//     &user_str
//   }
// }


// fn get_or_default(arg: &Option<String>) -> String {
//   if arg.is_none() {
//       return String::new();
//   }
//   let s = arg.unwrap();
//   s.clone()
// }


fn get_or_default(arg: &Option<&str>) -> String {
  if arg.is_none() {
      return String::new();
  }
  let s = arg.unwrap();
  s.to_string()
}


fn get_or_default_2(arg: &Option<String>) -> String {
  match arg {
      None => String::new(),
      Some(s) => s.clone()
  }
}
