mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

/* Aggiungere use e un path in uno scope è simile a creare link simbolici nel filesystem. */
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

use std::collections::*; //prendi tutti gli item pubblici definiti in sdf::collections