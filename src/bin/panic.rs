fn add(a: u32,b:u32) ->u32{
    unimplemented!();
}

fn divide_by_three(x: u32) ->u32 {
    for i in 0..  {
        if 3 * i < i {
            panic!("u32 overflow");
        }
        if x < 3 * i {
            return i -1;
        }
    }
    // 一定无法走到这里
    unreachable!();
}

fn main(){
    // panic!  宏
    // panic!("error!");
    // panic!("here!");

    // 断言

    // assert!(1==2);

    // assert_eq!(1,2);

    // 未实现代码  unimplemented

    // 不应当被访问到的代码
    divide_by_three(100);



}