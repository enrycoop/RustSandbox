pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

impl List {
    pub fn build_from_range(start: i32, end: i32) -> Result<List,&'static str> {
        let mut list = Nil;
        let mut item = end;
        if start > end {
            return Err("start must be >= of end");
        }
        while item != start - 1 {
            let node = Cons(item, Box::new(list));
            list = node;
            item = item - 1;
        }
        Ok(list)
    }

    pub fn at(&self, index: usize) -> Result<i32, &'static str> {
        let mut cursor = self;
        let mut item = 0;
        for _ in 0..=index {
            match cursor {
                Cons(x, y) => {
                    item = *x;
                    cursor = &(*y);
                }
                Nil => return Err("Out of range"),
            } 
        }
        Ok(item)
    }

    pub fn print_elements(&self) -> () {
        let mut node = self;
        print!("[ ");
        loop {
            match node {
                Cons(x, y) => {
                    print!("{} ", x);
                    node = &(*y);
                }
                Nil => break,
            }
        }
        println!("]");
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn instantiate_a_list() {
        let mut list = List::build_from_range(1, 3).unwrap();
        for i in 1..=3 {
            match list {
                Cons(x, y) => {
                    if x != i {
                        panic!("test failed");
                    }
                    list = *y;
                },
                Nil => break,
            }
        }
    }

    #[test]
    fn access_at_elements() {
        let i = 0;
        let list = List::build_from_range(0, 3).unwrap();
        let readed = list.at(i).unwrap_or_else(|err| {
            println!("Error: {err}");
            -1
        });
        assert_eq!(readed, 0);
        let i = 3;
        let list = List::build_from_range(0, 3).unwrap();
        let readed = list.at(i).unwrap_or_else(|err| {
            println!("Error: {err}");
            -1
        });
        assert_eq!(readed, 3);
        let i = 2;
        let list = List::build_from_range(0, 3).unwrap();
        let readed = list.at(i).unwrap_or_else(|err| {
            println!("Error: {err}");
            -1
        });
        assert_eq!(readed, 2);
    }

    #[test]
    fn access_out_of_bound() {
        let i = 10;
        let list = List::build_from_range(0, 3).unwrap();
        let readed = list.at(i).unwrap_or_else(|err| {
            println!("Error: {err}");
            -1
        });
        assert_eq!(readed, -1);
    }

}