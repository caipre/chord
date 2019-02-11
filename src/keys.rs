struct Key;

trait KeyStore {
    fn get(&self, id: usize) -> &[u8];
    fn put(&self, id: usize, val: &[u8]);
}
