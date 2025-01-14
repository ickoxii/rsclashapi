// cargo run --example format_tag
fn format_tag(tag: &str) -> String {
    if tag.starts_with('#') {
        format!("%23{}", &tag[1..])
    } else {
        format!("%23{}", tag)
    }
}

fn main() {
    let tag1 = "#abcdef";
    let tag2 = "abcdef";

    println!("{}", format_tag(tag1));
    println!("{}", format_tag(tag2));
}
