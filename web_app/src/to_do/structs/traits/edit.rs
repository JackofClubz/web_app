/*
the point of the edit trait is to store functions we can use for different processes 
*/

pub trait Edit{
    fn set_to_done(&self, title:&str){
        println!("{} is being set to done", title );
    }
    fn set_to_pending(&self, title:&str){
        println!("{} is being set to pending", title);
    }
}