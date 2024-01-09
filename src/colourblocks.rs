const COLOUR_RESET: &str = "\x1b[0m";
//use std::io::Write;
use std::fmt::Write;

pub fn colourblocks(indent: usize, colours: usize, line_length: usize) -> String {
    let mut blocks = String::new();
    append_spaces(&mut blocks, indent);
    for i in 0..colours {
        if (i % line_length == 0) && (i != 0) {
            blocks.push_str(COLOUR_RESET);
            blocks.push('\n');
            append_spaces(&mut blocks, indent);
        }
        write!(&mut blocks, "\x1b[38;5;{i}m\x1b[48;5;{i}m   ")
            .expect("Could not write colourblocks for some reason");
    }
    blocks.push_str(COLOUR_RESET);
    blocks
}

fn append_spaces(s: &mut String, count: usize) {
    s.push_str(&format!("{:indent$}", "", indent = count));
}
