use wasm_bindgen::prelude::*;

const LEVEL_NONE: u8 = 0;
const LEVEL_DEBUG: u8 = 1;
const LEVEL_INFO: u8 = 2;
const LEVEL_WARN: u8 = 3;
const LEVEL_ERROR: u8 = 4;

const MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn to_lower(bytes: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .map(|&b| if b >= b'A' && b <= b'Z' { b + 32 } else { b })
        .collect()
}

fn is_word_boundary(line: &[u8], pos: usize, len: usize) -> bool {
    let before_ok = pos == 0 || {
        let b = line[pos - 1];
        !b.is_ascii_alphanumeric() && b != b'_'
    };
    let after_ok = pos + len >= line.len() || {
        let b = line[pos + len];
        !b.is_ascii_alphanumeric() && b != b'_'
    };
    before_ok && after_ok
}

fn has_word(lower: &[u8], pat: &[u8]) -> bool {
    lower
        .windows(pat.len())
        .enumerate()
        .any(|(i, w)| w == pat && is_word_boundary(lower, i, pat.len()))
}

fn detect_level(line: &[u8]) -> u8 {
    let lower = to_lower(line);
    if has_word(&lower, b"error") || has_word(&lower, b"erro") {
        LEVEL_ERROR
    } else if has_word(&lower, b"warn") {
        LEVEL_WARN
    } else if has_word(&lower, b"info") {
        LEVEL_INFO
    } else if has_word(&lower, b"debug") || has_word(&lower, b"dbg") {
        LEVEL_DEBUG
    } else {
        LEVEL_NONE
    }
}

fn digit(b: u8) -> Option<u32> {
    if b >= b'0' && b <= b'9' {
        Some((b - b'0') as u32)
    } else {
        None
    }
}

fn parse_digits2(line: &[u8], pos: usize) -> Option<u32> {
    Some(digit(*line.get(pos)?)? * 10 + digit(*line.get(pos + 1)?)?)
}

fn parse_digits4(line: &[u8], pos: usize) -> Option<u32> {
    Some(
        digit(*line.get(pos)?)? * 1000
            + digit(*line.get(pos + 1)?)? * 100
            + digit(*line.get(pos + 2)?)? * 10
            + digit(*line.get(pos + 3)?)?,
    )
}

fn days_since_epoch(year: u32, month: u32, day: u32) -> u64 {
    let y = year as i64 - 1;
    let m = month as i64;
    let d = day as i64 - 1;
    let leap = y / 4 - y / 100 + y / 400;
    let year_days = y * 365 + leap;
    let month_days: i64 = (0..(m - 1) as usize)
        .map(|i| MONTH_DAYS[i] as i64)
        .sum::<i64>()
        + if m > 2 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            1
        } else {
            0
        };
    (year_days + month_days + d) as u64
}

fn days_in_month(year: u32, month: u32) -> u32 {
    let base = MONTH_DAYS[(month - 1) as usize];
    if month == 2 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        base + 1
    } else {
        base
    }
}

fn parse_iso_timestamp(line: &[u8]) -> Option<f64> {
    if line.len() < 19 {
        return None;
    }
    let year = parse_digits4(line, 0)?;
    let dash1 = *line.get(4)?;
    let month = parse_digits2(line, 5)?;
    let dash2 = *line.get(7)?;
    let day = parse_digits2(line, 8)?;
    let sep = *line.get(10)?;
    let hour = parse_digits2(line, 11)?;
    let colon1 = *line.get(13)?;
    let min = parse_digits2(line, 14)?;
    let colon2 = *line.get(16)?;
    let sec = parse_digits2(line, 17)?;

    if dash1 != b'-'
        || dash2 != b'-'
        || (sep != b'T' && sep != b' ')
        || colon1 != b':'
        || colon2 != b':'
    {
        return None;
    }
    if month < 1 || month > 12 || day < 1 || day > days_in_month(year, month) || hour > 23 || min > 59 || sec > 59 {
        return None;
    }

    let days = days_since_epoch(year, month, day);
    let ts = (days * 86400 + hour as u64 * 3600 + min as u64 * 60 + sec as u64) as f64;

    let mut offset = 19;
    if line.len() > 19 && line[19] == b'.' {
        offset = 20;
        while offset < line.len() && line[offset].is_ascii_digit() {
            offset += 1;
        }
    }

    if offset < line.len() && (line[offset] == b'+' || line[offset] == b'-') {
        let sign = if line[offset] == b'+' { -1.0 } else { 1.0 };
        if offset + 5 <= line.len() {
            let tz_h = parse_digits2(line, offset + 1)? as f64;
            let tz_m = parse_digits2(line, offset + 3)? as f64;
            return Some(ts + sign * (tz_h * 3600.0 + tz_m * 60.0));
        }
    }

    if offset < line.len() && line[offset] == b'Z' {
        return Some(ts);
    }

    Some(ts)
}

fn parse_common_log_timestamp(line: &[u8]) -> Option<f64> {
    if line.len() < 21 {
        return None;
    }
    let day = parse_digits2(line, 0)?;
    if *line.get(2)? != b'/' {
        return None;
    }
    let month = match &line[3..6] {
        b"Jan" => 1,
        b"Feb" => 2,
        b"Mar" => 3,
        b"Apr" => 4,
        b"May" => 5,
        b"Jun" => 6,
        b"Jul" => 7,
        b"Aug" => 8,
        b"Sep" => 9,
        b"Oct" => 10,
        b"Nov" => 11,
        b"Dec" => 12,
        _ => return None,
    };
    if *line.get(6)? != b'/' {
        return None;
    }
    let year = parse_digits4(line, 7)?;
    if *line.get(11)? != b':' {
        return None;
    }
    let hour = parse_digits2(line, 12)?;
    if *line.get(14)? != b':' {
        return None;
    }
    let min = parse_digits2(line, 15)?;
    if *line.get(17)? != b':' {
        return None;
    }
    let sec = parse_digits2(line, 18)?;

    if day < 1 || day > days_in_month(year, month) || hour > 23 || min > 59 || sec > 59 {
        return None;
    }

    let days = days_since_epoch(year, month, day);
    Some((days * 86400 + hour as u64 * 3600 + min as u64 * 60 + sec as u64) as f64)
}

fn parse_unix_timestamp(line: &[u8]) -> Option<f64> {
    let mut end = 0;
    while end < line.len() && line[end].is_ascii_digit() {
        end += 1;
    }
    if end == 10 {
        let ts: u64 = line[..end]
            .iter()
            .fold(0, |acc, &b| acc * 10 + (b - b'0') as u64);
        if ts > 946684800 && ts < 4102444800 {
            return Some(ts as f64);
        }
    }
    if end >= 10 && end <= 13 {
        let ts: u64 = line[..end]
            .iter()
            .fold(0, |acc, &b| acc * 10 + (b - b'0') as u64);
        if end == 13 {
            return Some(ts as f64 / 1000.0);
        }
        return Some(ts as f64);
    }
    None
}

fn parse_timestamp(line: &[u8]) -> Option<f64> {
    if line.len() >= 20 && (line[4] == b'-' && line[7] == b'-') {
        return parse_iso_timestamp(line);
    }
    if line.len() >= 3 && line[2] == b'/' {
        return parse_common_log_timestamp(line);
    }
    if line.len() >= 10
        && line[0].is_ascii_digit()
        && line[9].is_ascii_digit()
        && (line.len() == 10 || !line[10].is_ascii_digit())
    {
        return parse_unix_timestamp(line);
    }
    None
}

fn strip_cr(line_bytes: &[u8]) -> &[u8] {
    if line_bytes.ends_with(b"\r") {
        &line_bytes[..line_bytes.len() - 1]
    } else {
        line_bytes
    }
}

#[wasm_bindgen]
pub struct LogIndexer {
    raw_data: Vec<u8>,
    all_lines: Vec<usize>,
    error_lines: Vec<usize>,
    line_levels: Vec<u8>,
    line_timestamps: Vec<Option<f64>>,
    level_counts: [usize; 5],
}

#[wasm_bindgen]
impl LogIndexer {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>) -> Self {
        let mut all_lines = Vec::new();
        let mut error_lines = Vec::new();
        let mut line_levels = Vec::new();
        let mut line_timestamps = Vec::new();
        let mut level_counts = [0usize; 5];

        if data.is_empty() {
            return LogIndexer {
                raw_data: data,
                all_lines,
                error_lines,
                line_levels,
                line_timestamps,
                level_counts,
            };
        }

        let mut current_start = 0;
        all_lines.push(0);

        for index in 0..data.len() {
            if data[index] == 10 {
                let current_end = index;
                let line_bytes = strip_cr(&data[current_start..current_end]);

                let level = detect_level(line_bytes);
                line_levels.push(level);
                line_timestamps.push(parse_timestamp(line_bytes));
                level_counts[level as usize] += 1;

                if level == LEVEL_ERROR {
                    error_lines.push(all_lines.len() - 1);
                }

                current_start = index + 1;
                if current_start < data.len() {
                    all_lines.push(current_start);
                }
            }
        }

        if data[data.len() - 1] != 10 {
            let tail = strip_cr(&data[current_start..]);
            let level = detect_level(tail);
            line_levels.push(level);
            line_timestamps.push(parse_timestamp(tail));
            level_counts[level as usize] += 1;
            if level == LEVEL_ERROR {
                error_lines.push(all_lines.len() - 1);
            }
        }

        LogIndexer {
            raw_data: data,
            all_lines,
            error_lines,
            line_levels,
            line_timestamps,
            level_counts,
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

        let line_bytes = strip_cr(&self.raw_data[start..end]);

        String::from_utf8(line_bytes.to_vec()).ok()
    }

    pub fn get_line_level(&self, line_index: usize) -> u8 {
        self.line_levels
            .get(line_index)
            .copied()
            .unwrap_or(LEVEL_NONE)
    }

    pub fn get_line_timestamp(&self, line_index: usize) -> f64 {
        self.line_timestamps
            .get(line_index)
            .copied()
            .flatten()
            .unwrap_or(-1.0)
    }

    pub fn count_by_level(&self, level: u8) -> usize {
        self.level_counts[level as usize]
    }
}
