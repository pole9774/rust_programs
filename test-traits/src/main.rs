use std::fmt::Display;
use std::fmt::Formatter;

struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y:i32) -> Self {
        let p = Point{x, y};
        println!("Creating Point at address {:p}", &p);
        p
    }

    fn swap(self) -> Self {
        Point{x: self.y, y: self.x}
    }
}

// Uguaglianza di un Point con un altro Point
impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.x==other.x && self.y == other.y;
    }
}

// Uguaglianza di un Point con un intero
impl PartialEq<i32> for Point {
    fn eq(&self, other: &i32) -> bool {
        return self.x + self.y == *other;
    }
}

//Uguaglianza di un intero con un Point
impl PartialEq<Point> for i32 {
    fn eq(&self, other: &Point) -> bool {
        return other == self;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Dropping Point at address {:p}", self)
    }
}

fn main() {
    let p1 = Point::new(3, 2);
    let p2 = Point::new(2, 1);

    if p1 == p2 {
        println!("Uguali");
    } else {
        println!("Diversi");
    }

    if p1 == 5 {
        println!("Uguale a 5 {}, {}", p1, 5);
    } else {
        println!("Diverso da 5");
    }

    if 5 == p1 {
        println!("Uguale a 5");
    } else {
        println!("Diverso da 5");
    }
}