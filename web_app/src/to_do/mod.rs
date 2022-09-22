pub mod structs;
// building a factory pattern for the structs
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes{
    Pending(Pending),
    Done(Done)
}


/// This function builds and returns to do structs.
///
/// # Augments
/// * item_type (&String): the type of struct to be built and returned
/// * item_title (String): the title for the item to be built
///
/// # Returns
/// (Result<ItemTypes, &'templates str>):
pub fn to_do_factory(item_type: &String, item_title: String) -> Result<ItemTypes, &'static str> {
    if item_type == "pending" {
        let pending_item = Pending::new(item_title);
        Ok(ItemTypes::Pending(pending_item))
    }
    else if item_type == "done" {
        let done_item = Done::new(item_title);
        Ok(ItemTypes::Done(done_item))
    }
    else {
        Err("this is not accepted")
    }
}
/*
Here, we lock down the structs by removing the 'pub' definition as we will 
only allow it to be used via the interface, which is the to_do_factory function.
In this function, we check the input type and build the struct depending on that type.
We also package an error if we pass in a type that we do not have. We are also using an enum
to enable the return of the two types of items. 
*/