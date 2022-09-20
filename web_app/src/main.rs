mod to_do;
mod state;
use std::env;
use std::fs::read;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};
use to_do::ItemTypes;
use to_do::to_do_factory;
use to_do::structs::traits::create::Create;

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "washing");
    match to_do_item.unwrap(){
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!("it's a done item with the title: {}", item.super_struct.title)
    }

    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];
    let mut state:Map<String, Value> = read_file(String::from("./state.json"));
    println!("{:?}", state);
    state.insert(title_to_strin(), json!(status));
    write_to_file("./state.json", &mut state);
}

/*
The to_do_item variable of type Result takes in the enum ItemTypes as OK values and a referenced string as Err value. 
*/