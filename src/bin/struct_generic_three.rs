struct Point<T>{
    x: T,
    y: T
}

impl<T: Clone + std::cmp::PartialOrd> Point<T> {

    fn largest(&self) -> T {
        if self.x > self.y {
            self.x.clone()
        }else {
            self.y.clone()
        }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    
    fn distance_from_orgin(&self) ->f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {

    let p = Point {x: 5.0, y: 20.0};
    println!("{:?}", p.largest());

    println!("{:?}", p.distance_from_orgin());


}