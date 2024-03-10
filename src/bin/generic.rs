fn largest<T: std::cmp::PartialOrd>(a: T,b: T) -> T{
    if a > b {
        a
    }else {
        b
    }
}





fn main(){
    println!("{}",largest(10, 20));
    println!("{}",largest(10.0, 900.0));

}