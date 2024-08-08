use crate::fiveth::say_hello_five as say_hello_Again;

pub fn say_hellotiga() {
    println!("Hello i'm third");
    say_hello_Again();
}

pub mod second{
    pub mod third{
        pub fn say_hello(){
            // crate::first::say_hellotiga(); //can use this, will reference to main.rs
        //     how using `super`
            super::super::say_hellotiga();

        }
    }
}