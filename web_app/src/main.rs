mod to_do;
use to_do::ItemTypes;
use to_do::to_do_factory;
use to_do::structs::traits::create::Create;

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "make");
    match to_do_item.unwrap(){
        ItemTypes::Pending(item) => println!("it's a pending item with the title: {}", item.super_struct.title),
        ItemTypes::Done(item) => println!("it's a done item with the title: {}", item.super_struct.title)
    }

}

/*
The to_do_item variable of type Result takes in the enum ItemTypes as OK values and a referenced string as Err value. 
*/