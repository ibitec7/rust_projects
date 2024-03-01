struct Point<T,U>{
    x: T,
    y: U
}

impl<T,U: std::fmt::Display> Point<T,U>{
    fn swap<V,W>(self, other: Point<V,W>) -> Point<T,W>{
        Point {x: self.x, y: other.y}
    }

    fn new(x1: T, y1: U) -> Self{
        Point {x: x1, y: y1}
    }
}

fn main() {
    let a = Point::new(2,3.0);
    let b = Point::new(true,'a');
    let c = a.swap(b);
    println!("{} {}",c.x,c.y);

}~
