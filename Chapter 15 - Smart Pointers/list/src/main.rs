use list::List;

fn main() {
    let list = List::build_from_range(0, 10).unwrap();
    list.print_elements();
}
