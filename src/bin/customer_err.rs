// fn bar() -> Result<u32, &'static str>{
//     Ok(0)
// }
#[derive(Debug)]
pub enum Error {
    IO(std::io::ErrorKind),
    
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error.kind())
    }
}

fn do_read_file() -> Result<(),Error> {
    let data = std::fs::read("src/bin/main.rs")?;
    let data_str = std::str::from_utf8(&data).unwrap();

    println!("{:?}",data_str);
    Ok(())
}

// fn foo() -> Result<i32, &'static str>{
//     match bar() {
//         Ok(a) => return Ok(a as i32),
//         Err(e) => return Err(e),
//     }

//     // 问号表达式很简洁
//     // let  a= bar()?;
//     // Ok(a as i32)
// }

fn main() -> Result<(),Error>{
    // println!("{:?}",foo());

    do_read_file()?;
    Ok(())
}