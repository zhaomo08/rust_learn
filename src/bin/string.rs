
fn echo(s: &str){
    println!("{:?}",s);
}

struct Foo{
    // name: &str,
    name: String
}

fn main(){

    //  Hello World  这段数据它是保存在二进制文件中的,被保存在数据段的区域
    // Hello World 它叫做字符串的字面量
    
    // str 类型几乎永远不会被用到
    // 我们总是会使用  &str  
    // str 它代表的是在内存中(数据段,代码段,... 堆,栈)的字符串数据

    // &str 可以是数据段中的内容, 也可以是堆中的内容...

    let s: &'static str = "Hello World";

    let mut  t = String::from(s);
    //  String 类型,它拥有自己的数据
    //  可以修改
    //  String 类型 它是存在堆里的

    t.push_str("!!");


    println!("{:?}",t);

    echo(s);
    echo(&t);

    let foo =Foo{name: String::from(s)};
    println!("{:?}",foo.name);


    
}