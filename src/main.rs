// 创建变量: let  关键字
// 变量默认不可变的
// 可变变量: 变量名称前面加 mut
// 常量: const  关键字

use core::slice;

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


    // tuple

    let a: i32 = 10;
    let b: char ='A';

    let mytuple: (i32,char) =(a,b);

    let (c,d) = mytuple;

    println!("c={} d={}",c,d);

    println!("mytuple.0= {}" ,mytuple.0);
    println!("mytuple.1= {}" ,mytuple.1);

    // array

    let myarray: [u32;5] = [1,2,3,4,5];

    println!("myarray[1]= {}",myarray[1]);

    let index= "5".parse::<usize>().unwrap();

    // println!("myarray[5]= {}",myarray[index]);

    let mut mybuffer: [u32; 32 * 1024] =[0;32 * 1024];
    println!("mybuffer[1024]={}",mybuffer[1024]);

    mybuffer[1024] =13;
    println!("mybuffer[1024]={}",mybuffer[1024]);


    //slice 
    let arr: [i32;5] =[1,2,3,4,5];
    let slice = &arr[0..3];  // .. 是 Rust Range 语法. & 是引用符号

    println!("slice[0]= {} {}",slice[0],slice.len());


    let slice2 =&arr[3..5];
    println!("slice2[0]= {} slice2[1]{}",slice2[0],slice2[1]);

    // if slice2.is_empty()
    

    
}
 