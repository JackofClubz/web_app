/*
items of delete involve deleting the items previously created
*/

pub trait Delete{
    fn delete(&self, title: &str){
        println!("{} is being deleted", title);
    }
}