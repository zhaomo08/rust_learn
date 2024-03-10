// fn change(s: &mut String){
//     s.push_str(" changed");
// }

// 同一时间内至多只能有一个可变引用  内存安全机制
fn main(){
    let mut s = String::from("Hello World");

    let s1_ref = &mut s;
    let s2_ref = &mut s;

    // change(&mut s);
    println!("{}",s1_ref);
    println!("{}",s2_ref);

}