pub struct Base{
    pub title: String,
    pub status:String
}

impl Base{
      /// The constructor for the Base struct.
    ///
    /// # Arguments
    /// * input_title (String): the title of the to do item
    /// * status (String): the status of the to do item
    ///
    /// # Returns
    /// (Base): the constructed Base struct
    pub fn new(input_title: String, input_status: String) -> Base {
        return Base {title: input_title, status: input_status}
    }
}

/* here, we have a standard struct with a constucture
We also have to note that there is a pub keyword before the function, struct.
and attribute definitions. This is because we aim to use this struct
outside of the file. If we did not declare them as public, then the comiler would refure to compile
if we did use them externally 
*/ 
