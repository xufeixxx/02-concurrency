// metrics data structure
// 基本功能：inc/dec/snapshot

use core::fmt;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CmapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl CmapMetrics {
    pub fn new() -> Self {
        CmapMetrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}

impl Default for CmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
