#[derive(Debug,PartialEq,Default)]

struct Point{
    x: i32,
    y: i32,
    z: f64,
}


fn main() {
    // let p1: Point = Point{ x:10,y: 20};
    // let p2 = Point{ x:10,y: 20};

    
    // println!("{:?},",p);

    // if (p1.x == p2.x && p1.y == p2.y ) {
        
    // }

    // println!("{:?},",p1==p2);

    let p = Point::default();
    println!("{:?},",p);



}