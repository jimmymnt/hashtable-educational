use crate::hash::hashtable::HashTable;

pub mod hash;

fn main() {
    let mut ht = HashTable::new();
    ht.insert('a', 12);
    ht.insert('b', 12);
    ht.insert('c', 12);

    dbg!(ht);
}
