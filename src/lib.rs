pub struct LogIndexer {
    pub raw_data: Vec<u8>,
    pub all_lines: Vec<(usize, usize)>,
    pub error_lines: Vec<usize>,
}

impl LogIndexer {
    pub fn new(data: Vec<u8>) -> Self {
        let mut all_lines = Vec::new();
        let mut error_lines = Vec::new();
        let keyword_bytes = b"ERROR";

        let mut current_start = 0;

        for index in 0..data.len() {
            if data[index] == 10 {
                let current_end = index;

                let line_bytes = &data[current_start..current_end];

                if line_bytes
                    .windows(keyword_bytes.len())
                    .any(|w| w == keyword_bytes)
                {
                    error_lines.push(all_lines.len());
                }

                all_lines.push((current_start, current_end));

                current_start = index + 1;
            }
        }

        if current_start < data.len() {
            let line_bytes = &data[current_start..];
            if line_bytes
                .windows(keyword_bytes.len())
                .any(|w| w == keyword_bytes)
            {
                error_lines.push(all_lines.len());
            }
            all_lines.push((current_start, data.len()));
        }

        LogIndexer {
            raw_data: data,
            all_lines,
            error_lines,
        }
    }

    pub fn get_line(&self, line_index: usize) -> Option<&str> {
        if line_index >= self.all_lines.len() {
            return None;
        }

        let (start, end) = self.all_lines[line_index];
        let line_bytes = &self.raw_data[start..end];

        std::str::from_utf8(line_bytes).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_indexer() {
        let logs = b"INFO: User logged in\nERROR: Database connection failed\nINFO: Page requested\nERROR: Timeout occurred".to_vec();

        let indexer = LogIndexer::new(logs);

        assert_eq!(indexer.all_lines.len(), 4);
        assert_eq!(indexer.error_lines.len(), 2);

        assert_eq!(indexer.get_line(0), Some("INFO: User logged in"));
        assert_eq!(
            indexer.get_line(1),
            Some("ERROR: Database connection failed")
        );

        assert_eq!(indexer.error_lines[0], 1);
        assert_eq!(indexer.error_lines[1], 3);
    }
}
