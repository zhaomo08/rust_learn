use std::future;
use std::time::{SystemTime,Duration};

use std::thread::sleep;


//  Chrono   第三方库

fn main(){

    // SystemTime 是系统时间
    // 通过系统调用请求操作系统返回的系统时间
    let now = SystemTime::now();

    println!("{:?}",now);

    // timestamp 是自从 1970 年 1月 1 日 到现在的秒数
    let timestamp = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    println!("timestamp = {:?}",timestamp);
    

    // sleep(Duration::from_secs(4));



    // ela 航运里面的常见缩写
    println!("ela = {:?}",now.elapsed().unwrap());
    

    let future = now.checked_add(Duration::from_secs(60));
    println!("future = {:?}",future)

    
}