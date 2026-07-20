pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

impl User {
    pub fn say_hello(&self, name: String){
        println!("Hello {}, I am {}", name, self.first_name);
    }
}