struct Point<T,U >{
    x: T,
    y: T,
    z: U,
}

fn main() {
    let integer = Point{ x: 5, y: 10, z: 16.0};

    println!("{}",integer.x)
}