#![allow(dead_code)]

const PI: f64 = 3.14;

fn main() {
    let circle = Circle {
        x: 2f64, 
        radius: 7f64,
    };

    let rectangle = Rectangle {
        length: 5f64,
        breadth: 7f64,
    };

    get_area(&circle);
    get_area(&rectangle);
}

fn get_area(geom: &dyn Geometry) {
    println!("{}", geom.area()); 
}

trait Geometry {
    fn area(&self) -> f64;
}

impl Geometry for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.breadth
    }
}

struct Circle {
    x: f64,
    radius: f64,
}

impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

struct Rectangle {
    length: f64, 
    breadth: f64,
}

impl Rectangle {
    pub fn get_length(&self) -> f64 {
        self.length
    }

    pub fn get_breadth(&self) -> f64 {
        self.breadth
    }
}

