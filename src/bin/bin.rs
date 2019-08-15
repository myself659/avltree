use avltree::set::AvlTreeSet;

pub fn main() {
    println!("start: avltree");
    let mut set = (1..10_000 as u32).rev().collect::<AvlTreeSet<_>>();

    for i in 1..10_000 {
        set.take(&i);
    }
}
