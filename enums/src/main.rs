
#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64
}

#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEKeys(KeyPress),
    WEClick(MouseClick),
}

fn main() {
    let keys = KeyPress("AAAA".to_string(), 'A');
    let click = MouseClick { x: 20, y: 30 };

    let we_keys = WebEvent::WEKeys(keys);
    let click = WebEvent::WEClick(click);

    println!("{:?}", we_keys);
    println!("{:#?}", we_keys);
    println!("{:?}", click);
    println!("{:#?}", click);

    let mut test = WebEvent::WELoad(true);

    println!("{:#?}", test);

    let test_key_press = KeyPress(String::from("AAAA"), 'S');

    test = WebEvent::WEKeys(test_key_press);

    println!("{:#?}", test);
}
