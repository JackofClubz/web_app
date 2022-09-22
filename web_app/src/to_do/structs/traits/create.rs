/*
items of create, there is only one function and that is to create items. 
need to update the json with the write_to_file function from our state crate
*/
use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;
use crate::state::write_to_file;

pub trait Create{
    fn create(&self, title: &str, status: &String, state: &mut Map<String, Value>){
        state.insert(title.to_string(), json!(status));
        write_to_file("./state.json", state);
        println!("\n\n {} is being created \n \n", title);
    }
}