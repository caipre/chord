use chord_rpc::v1::*;

//struct Key;
//
//trait KeyStore {
//    fn get(&self, id: usize) -> &[u8];
//    fn put(&self, id: usize, val: &[u8]);
//}

pub fn keymeta(key: Key) -> KeyMeta {
    KeyMeta {
        create_time: None,
        update_time: None,
        name: key.name,
        data: key.data,
        labels: key.labels,
    }
}
