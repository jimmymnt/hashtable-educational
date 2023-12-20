use hash::hashtable::HashTable;

pub mod hash;
fn main() {
    let mut hash = HashTable::new();

    hash.insert("a", 12);
    hash.insert("b", 12);

    println!("hash is {:?}", hash);
}
