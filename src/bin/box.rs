// 当有一个在编译时未知大小的类型,而又想要在需要确切大小的上下文中使用这个类型值的时候
// 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
// 当希望拥有一个值只关心它的类型是否实现了特定的 trait 而不是其具体类型的时候

// ConsList 每一项包含两个元素: 当前项和下一项
//                            结束项

// ConsList(0,ConsList(1,ConsList(2,Nil)))


enum List {
    Cons(i32,Box<List>),
    Nil,
}
fn main() -> Result<(),Box<dyn std::error::Error>>{
    // let list = List::Cons(0, Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))));

    let a =[0;1024 * 512];
    let a_box = Box::new(a);
    let a_box = Box::new([0;1023 * 512]);

    let f = std::fs::read("/tmp/not_exist")?;
    Ok(())
    



}