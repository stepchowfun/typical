// This index records where each line of some source contents starts, so finding the line that
// contains a byte offset doesn't require scanning the source from its start.
#[derive(Clone, Debug)]
pub struct LineIndex {
    line_starts: Vec<usize>, // The first line starts at 0, and every other line follows a `\n`.
    source_len: usize,       // The length of the source contents in bytes
}

impl LineIndex {
    // Find the start of each line in the source contents.
    pub fn new(source_contents: &str) -> Self {
        Self {
            line_starts: std::iter::once(0)
                .chain(
                    source_contents
                        .match_indices('\n')
                        .map(|(index, _)| index + '\n'.len_utf8()),
                )
                .collect(),
            source_len: source_contents.len(),
        }
    }

    // Find the zero-based line containing a byte offset, if the offset is within the source or at
    // its end.
    pub fn line(&self, byte_offset: usize) -> Option<usize> {
        (byte_offset <= self.source_len).then(|| {
            self.line_starts
                .partition_point(|line_start| *line_start <= byte_offset)
                - 1
        })
    }

    // Find where a line starts, if the source has that line.
    pub fn line_start(&self, line: usize) -> Option<usize> {
        self.line_starts.get(line).copied()
    }

    // Find where a line ends, not including its `\n`, if the source has that line.
    pub fn line_end(&self, line: usize) -> Option<usize> {
        self.line_start(line)?;
        Some(
            self.line_start(line + 1)
                .map_or(self.source_len, |next_line_start| {
                    next_line_start - '\n'.len_utf8()
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::LineIndex;

    // Treat an empty source as a single empty line.
    #[test]
    fn empty_source() {
        let line_index = LineIndex::new("");

        assert_eq!(line_index.line(0), Some(0));
        assert_eq!(line_index.line(1), None);
        assert_eq!(line_index.line_start(0), Some(0));
        assert_eq!(line_index.line_end(0), Some(0));
        assert_eq!(line_index.line_start(1), None);
        assert_eq!(line_index.line_end(1), None);
    }

    // Locate offsets on the lines that contain them, including the empty line after a final line
    // break, and reject offsets beyond the source.
    #[test]
    fn lines_contain_offsets() {
        let source = "zero\none\n";
        let line_index = LineIndex::new(source);

        assert_eq!(line_index.line(0), Some(0));
        assert_eq!(line_index.line(4), Some(0));
        assert_eq!(line_index.line(5), Some(1));
        assert_eq!(line_index.line(8), Some(1));
        assert_eq!(line_index.line(9), Some(2));
        assert_eq!(line_index.line(10), None);
    }

    // Report each line's extent without its line break.
    #[test]
    fn line_extents() {
        let source = "zero\r\none\n";
        let line_index = LineIndex::new(source);

        assert_eq!(line_index.line_start(0), Some(0));
        assert_eq!(line_index.line_end(0), Some(5));
        assert_eq!(line_index.line_start(1), Some(6));
        assert_eq!(line_index.line_end(1), Some(9));
        assert_eq!(line_index.line_start(2), Some(10));
        assert_eq!(line_index.line_end(2), Some(10));
        assert_eq!(line_index.line_start(3), None);
        assert_eq!(line_index.line_end(3), None);
    }
}
