use std::fs::File;
use std::fs;
use std::io::Read;
use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

pub fn read_file(file_name:&str) -> Map<String, Value>{
    let mut file = File::open(file_name.to_string()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    return state
}

pub fn write_to_file(file_name: &str, state:&mut Map<String, Value>){
    let new_data = json!(state); //we convert the filepath map into json using the macro and then to string to write in the file
    fs::write(file_name.to_string(), new_data.to_string()).expect("unable to write file");
}

/*
the read function, takes a path as a string and use the std library to open it. 
we directly unwrap it. We define the mutable string under the name of data and read
the file to that string (remember, strings are references to string literals)

when we use the serde to converte that string into a JSON value and then define that value
as an object and clone it to get a serde JSON map. If we do not lcone it, we will
merely be returning a reference. We go through the trouble of convertin it to a map to get the extra functionality
*/