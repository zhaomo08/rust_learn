// 创建变量: let  关键字
// 变量默认不可变的
// 可变变量: 变量名称前面加 mut
// 常量: const  关键字

use core::slice;
use std::net::IpAddr;

const A_CONST: i32 = 42;


enum IPAddr {
    IPv4(u8,u8,u8,u8),
    IPV6(u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8)
}

enum Symbol {
    Char(char),
    Number,
}
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
    

    // 元祖结构
    struct  Pair(i32, f32);

    // 标准的 C 结构
    #[derive(Debug)]  // 派生属性
    struct Person{
        name: String,
        age: u32,
    }

    // 单元结构(无字段, 通常范型里使用较多)
    struct Unit;

    let pair = Pair(10,4.2);

    // struct.member

    println!("{}" ,pair.0);

    let jack = Person{
        name:String::from("jack"),
        age:6
    }; 
    println!("name ={} ,age = {}" ,jack.name,jack.age);
    println!("{:?}",jack);


    let unit = Unit;




    // enum
    let localhost: IPAddr =IPAddr::IPv4(127, 0, 0, 1);

    match localhost{
        IPAddr::IPv4(a,b,c,d) =>{
            println!("{} {} {} {}",a,b,c,d);
        }
        _ => {}
    }
     

    // if 表达式  实现 C语言中的三值表达式的功能
    let cond = true;
    let a = if cond {
        println!("{}",42);
    }else {
        println!("{}",24);

    };

    // loop 表达式的break 语句后可跟着一个返回值返回
    let mut s = 0;
    let mut n = 10;
    
    let a = loop {
        if n < 0{
            break s;
        }
        s += n;
        n -= 1; 
    };
    println!("{:?}",a);


    //  iter 
    let mut myarray = [1, 2, 3]; // ×2

    for i in myarray.iter_mut() {
        *i *= 2;
    }
    for i in myarray.iter(){
        println! ("{}", i);
    } 

    
    //  
    let letter = Symbol::Char('A');

    if let Symbol::Char(x) =letter {
        println!("{}",x);
    }

}
 