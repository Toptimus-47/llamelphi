use crate::domain::DocumentChunk;
use regex::Regex;
use std::fs;
use anyhow::Result;

pub struct MarkdownChunker;

impl MarkdownChunker {
    /// 헤더를 기준으로 마크다운을 의미론적으로 분할
    pub fn chunk_markdown(content: &str, source: &str) -> Vec<DocumentChunk> {
        let header_re = Regex::new(r"(?m)^(#{1,3})\s+(.*)$").unwrap();
        let mut chunks = Vec::new();
        
        let matches: Vec<_> = header_re.find_iter(content).collect();
        let mut current_headers: Vec<String> = vec![String::new(); 4]; 
        
        if matches.is_empty() {
            chunks.push(DocumentChunk {
                text: content.to_string(),
                source: source.to_string(),
                embedding: Vec::new(),
                timestamp: 0,
                importance: 1.0,
                feedback_score: 0.0,
            });
            return chunks;
        }

        let mut last_pos = 0;
        for (i, m) in matches.iter().enumerate() {
            let start = m.start();
            
            if i > 0 {
                let prev_start = matches[i-1].start();
                let section_text = &content[prev_start..start].trim();
                if !section_text.is_empty() {
                    chunks.push(DocumentChunk {
                        text: section_text.to_string(),
                        source: format!("{}#{}", source, current_headers[1..].iter().filter(|h| !h.is_empty()).cloned().collect::<Vec<_>>().join("/")),
                        embedding: Vec::new(),
                        timestamp: 0,
                        importance: 1.0,
                        feedback_score: 0.0,
                    });
                }
            } else if start > 0 {
                let intro = &content[0..start].trim();
                if !intro.is_empty() {
                    chunks.push(DocumentChunk {
                        text: intro.to_string(),
                        source: format!("{}/Intro", source),
                        embedding: Vec::new(),
                        timestamp: 0,
                        importance: 1.0,
                        feedback_score: 0.0,
                    });
                }
            }

            let caps = header_re.captures(m.as_str()).unwrap();
            let level = caps.get(1).unwrap().as_str().len();
            let title = caps.get(2).unwrap().as_str().to_string();
            
            if level <= 3 {
                current_headers[level] = title;
                for header in current_headers.iter_mut().take(4).skip(level + 1) {
                    *header = String::new();
                }
            }
            last_pos = start;
        }

        if last_pos < content.len() {
            let section_text = &content[last_pos..].trim();
            if !section_text.is_empty() {
                chunks.push(DocumentChunk {
                    text: section_text.to_string(),
                    source: format!("{}#{}", source, current_headers[1..].iter().filter(|h| !h.is_empty()).cloned().collect::<Vec<_>>().join("/")),
                    embedding: Vec::new(),
                    timestamp: 0,
                    importance: 1.0,
                    feedback_score: 0.0,
                });
            }
        }

        chunks
    }

    pub fn migrate_file(file_path: &str) -> Result<Vec<DocumentChunk>> {
        let content = fs::read_to_string(file_path)?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(file_path);
        
        Ok(Self::chunk_markdown(&content, file_name))
    }
}
