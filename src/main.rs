use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn pretty(t: &str) -> String {
    let mut text = String::new();
    let sentence: Vec<&str> = t.split("\r\n\r\n").collect();
    for s in sentence {
        let s = String::from(s);
        let s = s.replace("-\r\n", "");
        let s = s.replace("\r\n", " ");
        text.push_str(&s);
        text.push_str("\r\n\r\n");
    }
    text
}

fn main() {
    let mut clip: ClipboardContext =
        ClipboardProvider::new().expect("found not clipboard contents");
    let pretty_text = pretty(&clip.get_contents().unwrap());
    clip.set_contents(pretty_text).unwrap();
}

#[test]
fn test_pretty() {
    let x = pretty(
        "gue gue pi-\r\nyo pipipi\r\nhogehoge"
    );
    let ok = String::from("gue gue piyo pipipi hogehoge\r\n\r\n");
    assert_eq!(x, ok);
}
