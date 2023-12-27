#[derive(Debug)]




struct  User {
    username:String,
    email:String,
    signin_count: u64,
    active:bool
}


/// Old way to interact and update the struture 
fn update_email(user: &mut User, new_email:&str){
    user.email = String::from(new_email);
}

/// Using implement 

impl User {
    fn update_email(&mut self, email: &str){
        self.email = String::from(email);
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_structs() {
       let mut user = User{
        username:String::from("name1"),
        email:String::from("email1"),
        signin_count: 1,
        active: true,
       };


       update_email(&mut user, "email_new");
       dbg!(user);

       let mut user2 = User{
        username:String::from("name2"),
        email:String::from("email2"),
        signin_count: 0,
        active: false,
       };

       user2.update_email("email3");

    dbg!(user2);

    }
}