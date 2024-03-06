use std::thread;
fn main() {

   let times3 = |n: u32| ->  u32 {n *3};

   println!("{:?}",times3(10));


   // move:  将环境中的值移到闭包内部
   // 使用场景-多线程: 从主线程移动值到子线程

   let hello_message = "hello world";

   thread::spawn(move || {
        println!("{}",hello_message);
   }).join();

}
