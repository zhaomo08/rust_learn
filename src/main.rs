// 创建变量: let  关键字
// 变量默认不可变的
// 可变变量: 变量名称前面加 mut
// 常量: const  关键字

const A_CONST: i32 = 42;
fn main() {
    let mut x = 5;
    println!("The value of x is {}",x);

    x = 6;
    println!("The value of x is {}",x);

    println!("The value of A_CONST is {}",A_CONST);


    let c ='z';
    let z ='Z';
    let heart_eyed_cat ='😻';

    println!("{} {} {}",c,z,heart_eyed_cat);


}
