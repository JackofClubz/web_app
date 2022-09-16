pub struct Base{
    pub title: String,
    pub status:String
}

impl Base{
    pub fn new(input_title: &str, input_status:&str) -> Base{
        return Base {title:input_title.to_string(),
                    status:input_status.to_string()
    }
}

/* here, we have a standard struct with a constucture
We also have to note that there is a pub keyword before the function, struct.
and attribute definitions. This is because we aim to use this struct
outside of the file. If we did not declare them as public, then the comiler would refure to compile
if we did use them externally 
*/ 
