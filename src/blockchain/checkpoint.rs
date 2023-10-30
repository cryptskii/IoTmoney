use crate::storage::Storage;

pub struct Checkpoint {
    state: Vec<u8>, // This should reflect the actual state structure of your application
}

impl Checkpoint {
    pub fn create(storage: &Storage) -> Self {
        let state = storage.dump_state();
        Self { state }
    }

    pub fn restore(&self, storage: &mut Storage) {
        storage.load_state(&self.state);
    }
}
