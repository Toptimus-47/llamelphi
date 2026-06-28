use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Deserialize, Serialize)]
struct FeedbackEntry {
    _session_id: String,
    query: String,
    answer: String,
    _timestamp: String,
}

fn main() -> anyhow::Result<()> {
    let file = File::open("vector_db/feedback.jsonl")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let entry: FeedbackEntry = serde_json::from_str(&line)?;
        println!("Training on: {}", entry.query);
    }

    Ok(())
}
