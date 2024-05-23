// metrics data structure
// 基本功能：inc/dec/snapshot

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Ok;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut data = self.data.lock().unwrap();
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut data = self.data.lock().unwrap();
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> HashMap<String, i64> {
        let data = self.data.lock().unwrap(); // mutexguard impl deref
        data.clone()
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
