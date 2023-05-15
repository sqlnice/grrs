use std::io;
pub fn find_matches(content: &str, pattern: &str, mut writer: impl io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}
