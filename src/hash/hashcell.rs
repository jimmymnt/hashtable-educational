use std::fmt::Debug;

#[derive(Default, Clone)]
pub struct HashCell<Key, Value> {
    pub key: Key,
    pub value: Value,
    pub taken: bool,
}

impl<K: Debug, V: Debug> Debug for HashCell<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", &self.key, &self.value)
    }
}
