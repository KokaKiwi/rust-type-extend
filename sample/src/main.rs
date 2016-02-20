#![feature(plugin)]
#![plugin(type_extend)]

mod ext;

fn main() {
    use ext::VecExt;

    let items = vec![1, 2, 3];
    items.print();
}
