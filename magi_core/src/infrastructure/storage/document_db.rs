use sled::Db;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use std::path::Path;

/// Generic Document Database using Sled as a backend.
/// Supports storing any serializable/deserializable data under a collection-like tree structure.
pub struct DocumentDb {
    db: Db,
}

impl DocumentDb {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path).context("Failed to open Sled database")?;
        Ok(Self { db })
    }

    /// Store a document in a specific collection with a given ID.
    pub fn insert<T: Serialize>(&self, collection: &str, id: &str, document: &T) -> Result<()> {
        let tree = self.db.open_tree(collection)?;
        let bson_val = bson::to_bson(document).context("Failed to serialize document to BSON")?;
        let bson_doc = bson_val.as_document().context("Failed to convert BSON to document")?;
        let mut data = Vec::new();
        bson_doc.to_writer(&mut data).context("Failed to encode BSON document")?;
        tree.insert(id, data)?;
        Ok(())
    }

    /// Retrieve a document by ID from a specific collection.
    pub fn get<T: for<'de> Deserialize<'de>>(&self, collection: &str, id: &str) -> Result<Option<T>> {
        let tree = self.db.open_tree(collection)?;
        if let Some(data) = tree.get(id)? {
            let cursor = std::io::Cursor::new(data.to_vec());
            let bson_doc = bson::Document::from_reader(cursor).context("Failed to decode BSON document")?;
            let doc = bson::from_bson(bson::Bson::Document(bson_doc)).context("Failed to deserialize BSON to document")?;
            Ok(Some(doc))
        } else {
            Ok(None)
        }
    }

    /// List all documents in a collection.
    pub fn list_all<T: for<'de> Deserialize<'de>>(&self, collection: &str) -> Result<Vec<T>> {
        let tree = self.db.open_tree(collection)?;
        let mut results = Vec::new();
        for item in tree.iter() {
            let (_, data) = item?;
            let cursor = std::io::Cursor::new(data.to_vec());
            let bson_doc = bson::Document::from_reader(cursor).context("Failed to decode BSON document")?;
            let doc = bson::from_bson(bson::Bson::Document(bson_doc)).context("Failed to deserialize BSON to document")?;
            results.push(doc);
        }
        Ok(results)
    }

    /// Delete a document by ID.
    pub fn delete(&self, collection: &str, id: &str) -> Result<()> {
        let tree = self.db.open_tree(collection)?;
        tree.remove(id)?;
        Ok(())
    }

    /// Flush the database to disk.
    pub async fn flush(&self) -> Result<usize> {
        self.db.flush_async().await.context("Failed to flush database")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestDoc {
        name: String,
        value: i32,
    }

    #[test]
    fn test_doc_db_basic_ops() -> Result<()> {
        let dir = tempdir()?;
        let db = DocumentDb::open(dir.path())?;
        
        let doc = TestDoc { name: "MAGI".to_string(), value: 2026 };
        db.insert("projects", "magi-1", &doc)?;

        let retrieved: Option<TestDoc> = db.get("projects", "magi-1")?;
        assert_eq!(retrieved, Some(doc));

        let all: Vec<TestDoc> = db.list_all("projects")?;
        assert_eq!(all.len(), 1);

        db.delete("projects", "magi-1")?;
        assert_eq!(db.get::<TestDoc>("projects", "magi-1")?, None);

        Ok(())
    }
}
