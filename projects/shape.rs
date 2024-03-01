use std::f64::consts::PI;

pub trait Polygon {
    fn area(&self) -> f64;
    fn draw(&self) -> f64;
    fn perimeter(&self);
    fn define(&self){
        format!("This is an {}-gon",self.N)
    }
}

pub struct Triangle {
    N: i32,
    base: f64,
    height: f64,
    a: f64,
    b: f64,
    c: f64,
}

pub struct Circle {
    N: i32,
    radius: f64
}

impl Polygon for Triangle  {
    fn area(&self) -> f64{
        0.5 * self.base * self.height
    }

    fn perimeter(&self) -> f64{
        self.a + self.b + self.c
    }

    fn draw(&self){
        println!("A Triangle is drawn!")
    }

    fn define(&self){
        println!("This is a triangle")
    }
}

impl Polygon for Circle {
    fn area(&self) -> f64{
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64{
        2.0 * PI * self.radius
    }

    fn draw(&self){
        println!("A circle is drawn!")
    }

    fn define(&self) {
        println!("This is a circle")
    }
}

impl Circle{
    fn new(r: f64) -> &Self{
        Circle{ radius = r }
    }
}
fn main(){
    a = Triangle::new()
}