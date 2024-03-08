fn function(){
    println!("function")
}
mod mod1 {
    pub fn function(){
       super::function();
    }

    pub mod mod2{
        pub fn function() {
            println!("mod1::mod2::function");   
        }

        pub fn call(){
            super::function();
        }
    }
}
fn main(){
    // mod1::mod2::function();
    mod1::mod2::call();


}