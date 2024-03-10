use std::fmt;
struct Point<T>{
    x: T,
    y: T,
}


// 现在我们约束T必须实现std::fmt::Display来允许格式化打印
impl <T: fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({} , {})",self.x, self.y)
    }
}

fn show(a: impl fmt::Display){
    println!("show {}",a)
}



fn main() {
    let point = Point{ x: 5, y: 10,};
    show(point);

    // println!("{}",point);
}