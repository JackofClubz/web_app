use serde_json::Map;
use serde_json::value::Value;

pub trait Get {

    fn get(&self, title: &String, state:Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item{
            Some(result) =>{
                println!("/n/nItem: {}", title);
                println!("Status: {} /n/n", result);
            },
            None => print!("item: {} was not found", title)
        }

    }

}

//we assume that the get trait will require a title,
// the &self paramater allows the struct to call the 
// function directly like some_struct.get(&String::from("something"))