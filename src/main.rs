extern crate plist;

use std::env;

fn main() {
    
    if env::args().count() != 2 {
        println!("you forgot the magic number my dude");
        return;
    }

    let arg = env::args().nth(1).unwrap();
    let argn: i32 = arg.parse().ok().unwrap_or(1);

    match argn {
        1 => println!("what you're looking for is {}", plist::s0001::solve()),
        2 => println!("what you're looking for is {}", plist::s0002::solve()),
        e @ _ => println!("uh oh, you might want a time machine for problem #{}", e),
    }
}