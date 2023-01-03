/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {:?}", rect1);

    // More readability

    println!("rect1 is {:#?}", rect1);

    // Now using a method

    println!("rect1 area {}", rect1.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/

/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
*/

// struct Point(i32, i32);

// fn main() {
//     let p = Point(1, 2);

//     impl Point {

//         fn x(&self) -> i32 { self.0 }

//     }

//     println!("{}", p.x());
// }


/*
struct Point {

    x: i32,
  
    y: i32
  
  }
  
  
  impl Point {
  
    fn get_x(&mut self) -> &mut i32 {
  
      &mut self.x
  
    }
  
  }
  
  
  fn main() {
  
    let mut p = Point { x: 1, y: 2 };
  
    let x = p.get_x();
  
    *x += 1;
  
    println!("{} {}", p.x , p.y);

  }

  */

/*
  fn foo(x: &i32) { 

    println!("{x}");
  
  }
  
  
  fn main() {
  
    let x = null;
  
    foo(x);
  
  }
*/
