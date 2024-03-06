use std::io::{self, Read};
use rand::Rng;

fn main(){
    println!("Guesss the number");
    let secrect_number: u32 = rand::thread_rng().gen_range(1..101);
    loop {
        // 获取用户输入
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess_number: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(err) => continue,
        };

        println!("You guesssed: {}",guess_number);

        // 判断大小
        if guess_number > secrect_number {
            println!("Too big");
        }else if guess_number < secrect_number {
            println!("Too small");
        }else {
            // 判断正确的话  程序退出
            println!("You win !");
            break;
        }
    

        

    }

}