/*
items of create, there is only one function and that is to create items. 
*/

pub trait Create{
    fn create(&self, title: &str){
        println!("{} is being created", title);
    }
}