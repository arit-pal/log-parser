use wasm_bindgen::prelude::*;

const LEVEL_NONE: u8 = 0;
const LEVEL_DEBUG: u8 = 1;
const LEVEL_INFO: u8 = 2;
const LEVEL_WARN: u8 = 3;
const LEVEL_ERROR: u8 = 4;

fn to_lower(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|&b| if b >= b'A' && b <= b'Z' { b + 32 } else { b }).collect()
}

fn detect_level(line: &[u8]) -> u8 {
    let lower = to_lower(line);
    let has = |pat: &[u8]| lower.windows(pat.len()).any(|w| w == pat);
    if has(b"error") || has(b"erro") {
        LEVEL_ERROR
    } else if has(b"warn") {
        LEVEL_WARN
    } else if has(b"info") {
        LEVEL_INFO
    } else if has(b"debug") || has(b"dbg") {
        LEVEL_DEBUG
    } else {
        LEVEL_NONE
    }
}

#[wasm_bindgen]
pub struct LogIndexer {
    raw_data: Vec<u8>,
    all_lines: Vec<usize>,
    error_lines: Vec<usize>,
    line_levels: Vec<u8>,
}

#[wasm_bindgen]
impl LogIndexer {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>) -> Self {
        let mut all_lines = Vec::new();
        let mut error_lines = Vec::new();
        let mut line_levels = Vec::new();

        let mut current_start = 0;

        all_lines.push(0);

        for index in 0..data.len() {
            if data[index] == 10 {
                let current_end = index;
                let line_bytes = &data[current_start..current_end];

                let level = detect_level(line_bytes);
                line_levels.push(level);

                if level == LEVEL_ERROR {
                    error_lines.push(all_lines.len() - 1);
                }

                current_start = index + 1;
                if current_start < data.len() {
                    all_lines.push(current_start);
                }
            }
        }

        if !data.is_empty() && data[data.len() - 1] != 10 {
            let tail = &data[current_start..];
            let level = detect_level(tail);
            line_levels.push(level);
            if level == LEVEL_ERROR {
                error_lines.push(all_lines.len() - 1);
            }
        }

        LogIndexer {
            raw_data: data,
            all_lines,
            error_lines,
            line_levels,
        }
    }

    pub fn total_lines(&self) -> usize {
        self.all_lines.len()
    }

    pub fn total_errors(&self) -> usize {
        self.error_lines.len()
    }

    pub fn get_line(&self, line_index: usize) -> Option<String> {
        if line_index >= self.all_lines.len() {
            return None;
        }

        let start = self.all_lines[line_index];

        let end = if line_index + 1 < self.all_lines.len() {
            self.all_lines[line_index + 1] - 1
        } else {
            self.raw_data.len()
        };

        let line_bytes = &self.raw_data[start..end];

        String::from_utf8(line_bytes.to_vec()).ok()
    }

    pub fn get_line_level(&self, line_index: usize) -> u8 {
        self.line_levels.get(line_index).copied().unwrap_or(LEVEL_NONE)
    }

    pub fn count_by_level(&self, level: u8) -> usize {
        self.line_levels.iter().filter(|&&l| l == level).count()
    }
}
