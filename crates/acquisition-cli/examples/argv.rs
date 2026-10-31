//! The arguments this program received, as one JSON array of strings on
//! stdout, every character past ASCII escaped (`\uXXXX`), so no shell's
//! output encoding stands between the words and the reader. The C156
//! table test (`tests/answer.rs`,
//! `every_printed_word_reaches_the_program_whole_through_each_shell`)
//! runs it through each shell to see what a printed command delivers:
//! it splits its command line as `acq` does, by Rust's standard library.

fn main() {
    let mut out = String::from("[");
    for (i, arg) in std::env::args().skip(1).enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push('"');
        for unit in arg.encode_utf16() {
            match char::from_u32(u32::from(unit)) {
                Some(c) if (c.is_ascii_graphic() && c != '"' && c != '\\') || c == ' ' => {
                    out.push(c)
                }
                _ => out.push_str(&format!("\\u{unit:04x}")),
            }
        }
        out.push('"');
    }
    out.push(']');
    println!("{out}");
}
