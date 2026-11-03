use crate::integrity::{ContentAddressedBlob, IntegrityError};

#[derive(Debug, Clone)]
pub struct OfflineRecord {
    pub id: String,
    pub sequence: u64,
    pub blob: ContentAddressedBlob,
}

#[derive(Debug, Default)]
pub struct Reconciler {
    pub applied: Vec<String>,
    pub rejected: Vec<String>,
}

impl Reconciler {
    pub fn apply_bundle_tolerant(&mut self, mut records: Vec<OfflineRecord>) {
        records.sort_by_key(|r| r.sequence);
        for rec in records {
            match rec.blob.verify() {
                Ok(_) => self.applied.push(rec.id),
                Err(_) => self.rejected.push(rec.id),
            }
        }
    }

    pub fn apply_bundle(&mut self, mut records: Vec<OfflineRecord>) -> Result<(), IntegrityError> {
        records.sort_by_key(|r| r.sequence);
        for rec in records {
            rec.blob.verify()?;
            self.applied.push(rec.id);
        }
        Ok(())
    }
}
