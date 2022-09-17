mod to_do;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;

fn main() {
    let done: Done = Done::new("shopping");
    print!("{}", done.super_struct.title);
    print!("{}", done.super_struct.status);
    let pending:Pending = Pending::new("laundry");
    print!("{}", pending.super_struct.title);
    print!("{}", pending.super_struct.title);

}
