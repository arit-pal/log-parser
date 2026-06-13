use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LogIndexer {
    raw_data: Vec<u8>,
    all_lines: Vec<usize>,
    error_lines: Vec<usize>,
}

#[wasm_bindgen]
impl LogIndexer {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>) -> Self {
        let mut all_lines = Vec::new();
        let mut error_lines = Vec::new();
        let keyword_bytes = b"ERROR";

        let mut current_start = 0;

        all_lines.push(0);

        for index in 0..data.len() {
            if data[index] == 10 {
                let current_end = index;
                let line_bytes = &data[current_start..current_end];

                if line_bytes
                    .windows(keyword_bytes.len())
                    .any(|w| w == keyword_bytes)
                {
                    error_lines.push(all_lines.len() - 1);
                }

                current_start = index + 1;
                if current_start < data.len() {
                    all_lines.push(current_start);
                }
            }
        }

        LogIndexer {
            raw_data: data,
            all_lines,
            error_lines,
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
}
