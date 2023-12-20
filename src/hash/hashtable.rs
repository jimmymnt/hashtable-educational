use std::fmt::Debug;

use super::{hashable::HashAble, hashcell::HashCell};

/// We know HashTable aka HashMap in Computing
/// we have a list of value with (key, value).
///
/// E.g:
/// ```
/// [
///    ("jimmy", "0999888777"),
///    ("lee", "0999888717"),
///    ("david", "0999888768"),
///    ("thomas", "0999888880"),
///    ("smile", "0999888123"),
/// ]
/// ```
///
/// We need to store this into a hashtable.
///
/// A HashTable would contains 3 different things:
/// * Key
/// * Hash function: This function will be proceed to generate the key of
/// * Buckets (also know as the place to store the values input)
#[derive(Debug)]
pub struct HashTable<Key, Value> {
    cells: Vec<HashCell<Key, Value>>,
    taken_count: usize,
}

impl<Key, Value> HashTable<Key, Value>
where
    Key: Default + Clone + Debug + HashAble + PartialEq,
    Value: Default + Clone + Debug,
{
    pub fn new() -> Self {
        const INIT_CAPACITY: usize = 11;
        Self {
            cells: vec![HashCell::<_, _>::default(); INIT_CAPACITY],
            taken_count: 0,
        }
    }

    /// In case we need to insert more item into our HashTable and we have run out of slot
    /// then we need to extend our slot
    ///
    /// In this case, we will doubling slot for our HashTable.
    pub fn extend(&mut self) {
        let mut new_self = HashTable {
            cells: vec![HashCell::<_, _>::default(); self.cells.len() * 2 + 1],
            taken_count: 0,
        };

        for cell in self.cells.iter() {
            if cell.taken == true {
                new_self.insert(cell.key.clone(), cell.value.clone());
            }
        }

        *self = new_self;
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn len_of_used(&self) -> usize {
        self.taken_count
    }

    /// Search through the vector in cells.
    /// If there's a slot with `taken` is `false`
    /// we will insert the (key, value) into that cell.
    ///
    /// We are going to use [OPEN ADRESSING](https://en.wikipedia.org/wiki/Open_addressing).
    /// then if current index has been taken, we will move to next one again and over again till we
    /// found the index as has not been taken.
    ///
    /// E.g:
    ///
    /// ```
    /// use crate::hash::hashtable::HashTable;
    ///
    /// let ht: HashTable<i32, i32> = HashTable::new();
    /// ht.insert("a", 11);
    /// ```
    pub fn insert(&mut self, key: Key, value: Value) {
        if let Some(old_value) = self.get_mut(&key) {
            // If we find the value existed in our HashTable with same key
            // then we replace the old value with new one.
            *old_value = value;
            return;
        }

        // We need to check if our HashTable can store the (key, value) or not
        if self.taken_count >= self.cells.len() {
            // If NOT, then we need to extend our HashTable cells.
            self.extend();
        }

        let cells_len = self.cells.len();
        let mut index: usize = key.hash_key() % cells_len;

        while self.cells[index].taken == true {
            index = (index + 1) % cells_len;
        }

        self.cells[index].taken = true;
        self.cells[index].key = key;
        self.cells[index].value = value;
        self.taken_count += 1;
    }

    /// Get `value` from the HashTable by `key`.
    ///
    /// E.g:
    ///
    /// ```
    /// use crate::hash::hashtable::HashTable;
    ///
    /// let ht: HashTable<i32, i32> = HashTable::new();
    /// ht.insert(1, 12);
    ///
    /// let value = ht.get(1);
    /// ```
    pub fn get(&self, key: &Key) -> Option<&Value> {
        if let Some(index) = self.get_index(key) {
            Some(&self.cells[index].value)
        } else {
            None
        }
    }

    /// Get `index` from HashTable by `key`.
    ///
    /// E.g:
    ///
    /// ```
    /// use crate::hash::hashtable::HashTable;
    ///
    /// let ht: HashTable<i32, i32> = HashTable::new();
    /// ht.insert(1, 12);
    ///
    /// let value = ht.get_index(1);
    /// ```
    pub fn get_index(&self, key: &Key) -> Option<usize> {
        let mut index = key.hash_key() % self.cells.len();

        // TODO: Implement Binary Search for finding `key`
        for _ in 0..self.cells.len() {
            if self.cells[index].taken == false {
                break;
            }

            if self.cells[index].key == *key {
                break;
            }

            index = (index + 1) % self.cells.len();
        }

        if self.cells[index].taken == true && self.cells[index].key == *key {
            Some(index)
        } else {
            None
        }
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hash::hashtable::HashTable;
    ///
    /// let mut letters = HashMap::new();
    ///
    /// for ch in "there is nothing to be afraid".chars() {
    ///     letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    /// }
    ///
    /// assert_eq!(letters[&'s'], 2);
    /// assert_eq!(letters[&'t'], 3);
    /// assert_eq!(letters[&'u'], 1);
    /// assert_eq!(letters.get(&'y'), None);
    /// ```
    pub fn entry(&mut self, key: Key) {
        todo!()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hash::hashtable::HashTable;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// if let Some(x) = map.get_mut(&1) {
    ///     *x = "b";
    /// }
    /// assert_eq!(map[&1], "b");
    /// ```
    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        if let Some(index) = self.get_index(key) {
            Some(&mut self.cells[index].value)
        } else {
            None
        }
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hash::hashtable::HashTable;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        if let Some(index) = self.get_index(&key) {
            let return_value = self.cells[index].value.clone();

            self.cells[index] = HashCell::default();
            self.taken_count -= 1;

            return Some(return_value);
        }

        None
    }
}
