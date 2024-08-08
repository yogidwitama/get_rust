pub struct User{
    pub first_name:String,
    pub last_name:String,
    pub username:String,
    pub email:String,
    pub age:u8
}

impl User {
   pub fn sayHello(&self, name:&str){
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}