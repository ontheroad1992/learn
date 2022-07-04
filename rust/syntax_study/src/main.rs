struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // fn set(&self, num: T) {
    //     &self.y = num
    // }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    let x = p.x();

    println!("p.x = {}", x);

    let p1: Point<f32> = Point { x: 1.1, y: 2.1 };

    let s = p1.distance_from_origin();
    println!("s = {}", s)
}
