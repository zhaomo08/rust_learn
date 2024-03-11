use std::collections::HashMap;


fn main(){

    // 0 -100 , u32
    let mut transcript: HashMap<&str, u32>  = HashMap::new();

    transcript.insert("Alice", 96);
    transcript.insert("Bob", 80);
    transcript.insert("Chester", 99);


    // match transcript.get(&"Alice"){
    //     Some(data) => println!("Alice {:?}",data),
    //     None => println!("Alice not found"),
    // }

    // match transcript.get(&"Chester"){
    //     Some(data) => println!("Alice {:?}",data),
    //     None => println!("Alice not found"),
    // }


    // unorder
    for (&name, &socre) in  transcript.iter(){
        println!("{:?} {:?}",name,&socre);
    }






}