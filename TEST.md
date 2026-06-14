# Test Cases

## Core indexing (Rust/Wasm)

| # | Test | Expected |
|---|------|----------|
| 1 | Load `test.log` (1050 lines) | `total_lines() == 1050` |
| 2 | Load empty file (0 bytes) | `total_lines() == 0`, "No lines found" shown |
| 3 | Load file with no trailing newline | All lines indexed correctly |
| 4 | Load file with `\r\n` line endings | No `\r` in displayed lines |
| 5 | Load binary/non-UTF8 file | Empty lines shown, no crash |
| 6 | Load file with very long lines (>10KB) | Lines rendered without truncation |

## Level detection

| # | Test | Expected |
|---|------|----------|
| 7 | Line `ERROR: connection failed` | Detected as ERROR |
| 8 | Line `error_code = 500` | Detected as NONE (word boundary) |
| 9 | Line `debugger attached` | Detected as NONE (word boundary) |
| 10 | Line `infobox rendered` | Detected as NONE (word boundary) |
| 11 | Line `NO_ERROR found` | Detected as NONE (word boundary) |
| 12 | Line `[WARN] retrying` | Detected as WARN |
| 13 | Line `{"level":"info","msg":"x"}` | Detected as INFO |
| 14 | Line `2025-01-15 ERROR com.foo.Bar` | Detected as ERROR |
| 15 | Level chip counts match actual counts | All 5 counts correct |
| 16 | Toggle ERROR chip OFF | ERROR lines hidden, count badge dims |
| 17 | Toggle all chips OFF | "No matching lines" shown |

## Timestamp parsing

| # | Test | Expected |
|---|------|----------|
| 18 | `2025-01-15T10:30:00Z` | Parsed correctly |
| 19 | `2025-01-15T10:30:00+05:30` | Offset subtracted correctly |
| 20 | `2025-01-15T10:30:00-05:00` | Offset added correctly |
| 21 | `2025-01-15 10:30:00` (space separator) | Parsed correctly |
| 22 | `15/Jan/2025:10:30:00 +0000` | Common log format parsed |
| 23 | `1705312200` (10-digit Unix) | Parsed as epoch seconds |
| 24 | `1705312200123` (13-digit Unix) | Parsed as milliseconds |
| 25 | Line with no timestamp | Returns -1.0 |
| 26 | `2024-02-31` (invalid date) | Rejected |

## Search

| # | Test | Expected |
|---|------|----------|
| 27 | Type `ERROR` in search | Only ERROR lines shown |
| 28 | Type `connection` | Lines containing "connection" shown |
| 29 | Type regex `error\|warn` | Lines with error OR warn shown |
| 30 | Clear search button | All lines restored |
| 31 | Search + level filter combined | Intersection of both filters |
| 32 | Search highlights match in amber | `<mark>` tags visible |

## Time-range filter

| # | Test | Expected |
|---|------|----------|
| 33 | Click start time input | Flatpickr calendar opens |
| 34 | Select start and end time | Lines filtered to range |
| 35 | Adjust time range | Chart and line list update live |
| 36 | Click "Clear time" | Full range restored |
| 37 | Auto-populated range matches data | Start = min timestamp, End = max |

## Charts

| # | Test | Expected |
|---|------|----------|
| 38 | Level distribution bars | 5 bars, widths proportional to counts |
| 39 | Errors-over-time chart | Line/area chart with time labels |
| 40 | Narrow time range | Chart zooms to selected range |
| 41 | File with no timestamps | "No timestamps in error lines" shown |
| 42 | File with no errors | Empty chart with 0-width bars |

## Line detail panel

| # | Test | Expected |
|---|------|----------|
| 43 | Click any log line | Panel opens below with line details |
| 44 | Click Copy button | "Copied!" feedback, text in clipboard |
| 45 | Click X / press Esc | Panel closes |
| 46 | Navigate with j/k while panel open | Panel updates to new line |
| 47 | Click JSON line | "JSON" toggle appears |
| 48 | Click JSON toggle | Syntax-highlighted pretty-print shown |
| 49 | Click non-JSON line | No JSON section |

## Keyboard shortcuts

| # | Test | Expected |
|---|------|----------|
| 50 | `j` key | Selects next line |
| 51 | `k` key | Selects previous line |
| 52 | `j` at page boundary | Advances to next page |
| 53 | `k` at page start | Goes to previous page |
| 54 | `/` key | Focuses search input |
| 55 | `Esc` in search | Clears search, blurs input |
| 56 | `Esc` outside search | Clears everything, closes panel |
| 57 | `Enter` on selected line | Opens detail panel |

## Pagination

| # | Test | Expected |
|---|------|----------|
| 58 | Prev/Next buttons | Pages through 100 lines at a time |
| 59 | Page indicator | Shows "1 / N" correctly |
| 60 | Prev disabled on page 1 | Button grayed out |
| 61 | Next disabled on last page | Button grayed out |

## Export

| # | Test | Expected |
|---|------|----------|
| 62 | Click Export | Downloads `<filename>-filtered.log` |
| 63 | Export with search active | Only filtered lines in download |
| 64 | Export with level filter | Only matching lines in download |

## Drop zone / file loading

| # | Test | Expected |
|---|------|----------|
| 65 | Click drop zone | File dialog opens |
| 66 | Drag file over drop zone | Blue highlight border |
| 67 | Drop file | File loads, UI updates |
| 68 | Load second file | All state resets, new file loads |

## UI / Visual

| # | Test | Expected |
|---|------|----------|
| 69 | Dark theme consistent | No white flashes, all panels dark |
| 70 | Flatpickr calendar | Dark theme, rounded, styled |
| 71 | Level chips | Colored dots, counts, toggle on/off |
| 72 | Custom scrollbar | Thin, dark, matches theme |
| 73 | Responsive layout | Works on narrow viewport |
