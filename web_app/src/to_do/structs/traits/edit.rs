/*
the point of the edit trait is to store functions we can use for different processes 
our traits can now interact with the json file. However diretly interacting with the traits in 
the main file is not scalable
*/
use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;
use crate::state::write_to_file;

pub trait Edit{
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("done")));
        write_to_file(String::from("./state.json"), state);
        println!("\n\n{} is being set to done\n\n", title);
    }
    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("pending")));
        write_to_file(String::from("./state.json"), state);
        println!("\n\n{} is being set to pending\n\n", title);
    }
}