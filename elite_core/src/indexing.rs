use crate::rag::DocumentChunk;
use regex::Regex;
use std::fs;
use anyhow::Result;

pub struct MarkdownChunker;

impl MarkdownChunker {
    /// 헤더를 기준으로 마크다운을 의미론적으로 분할
    pub fn chunk_markdown(content: &str, source: &str) -> Vec<DocumentChunk> {
        let header_re = Regex::new(r"(?m)^(#{1,3})\s+(.*)$").unwrap();
        let mut chunks = Vec::new();
        
        let mut last_pos = 0;
        let mut current_headers: Vec<String> = vec![String::new(); 4]; // [H1, H2, H3]

        let matches: Vec<_> = header_re.find_iter(content).collect();
        
        if matches.is_empty() {
            // 헤더가 없는 경우 전체를 하나의 청크로 처리
            chunks.push(DocumentChunk {
                text: content.to_string(),
                source: source.to_string(),
                embedding: Vec::new(),
            });
            return chunks;
        }

        for (i, m) in matches.iter().enumerate() {
            let start = m.start();
            
            // 이전 섹션의 텍스트 추출
            if i > 0 {
                let prev_start = matches[i-1].start();
                let section_text = &content[prev_start..start].trim();
                if !section_text.is_empty() {
                    chunks.push(DocumentChunk {
                        text: section_text.to_string(),
                        source: format!("{}#{}", source, current_headers[1..].iter().filter(|h| !h.is_empty()).cloned().collect::<Vec<_>>().join("/")),
                        embedding: Vec::new(),
                    });
                }
            } else if start > 0 {
                // 첫 번째 헤더 이전의 서문(Introduction) 처리
                let intro = &content[0..start].trim();
                if !intro.is_empty() {
                    chunks.push(DocumentChunk {
                        text: intro.to_string(),
                        source: format!("{}/Intro", source),
                        embedding: Vec::new(),
                    });
                }
            }

            // 현재 헤더 레벨 및 제목 업데이트
            let caps = header_re.captures(m.as_str()).unwrap();
            let level = caps.get(1).unwrap().as_str().len();
            let title = caps.get(2).unwrap().as_str().to_string();
            
            if level <= 3 {
                current_headers[level] = title;
                // 하위 레벨 헤더 초기화
                for j in (level + 1)..4 {
                    current_headers[j] = String::new();
                }
            }
            
            last_pos = start;
        }

        // 마지막 섹션 처리
        if last_pos < content.len() {
            let section_text = &content[last_pos..].trim();
            if !section_text.is_empty() {
                chunks.push(DocumentChunk {
                    text: section_text.to_string(),
                    source: format!("{}#{}", source, current_headers[1..].iter().filter(|h| !h.is_empty()).cloned().collect::<Vec<_>>().join("/")),
                    embedding: Vec::new(),
                });
            }
        }

        chunks
    }

    /// 파일로부터 지식 데이터 마이그레이션 실행
    pub fn migrate_file(file_path: &str) -> Result<Vec<DocumentChunk>> {
        let content = fs::read_to_string(file_path)?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(file_path);
        
        Ok(Self::chunk_markdown(&content, file_name))
    }
}
