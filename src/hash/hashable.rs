pub trait HashAble {
    fn hash_key(&self) -> usize;
}

impl HashAble for i32 {
    fn hash_key(&self) -> usize {
        *self as usize
    }
}

impl HashAble for char {
    fn hash_key(&self) -> usize {
        let hash = 5381;
        let c = *self as usize;

        ((hash << 5) + hash) + (c as usize)
    }
}

impl HashAble for &str {
    fn hash_key(&self) -> usize {
        let mut hash = 5381;

        for c in self.bytes() {
            hash = ((hash << 5) + hash) + (c as usize);
        }

        hash
    }
}

impl HashAble for String {
    fn hash_key(&self) -> usize {
        let mut hash = 5381;

        for c in self.bytes() {
            hash = ((hash << 5) + hash) + (c as usize);
        }

        hash
    }
}
