pub fn main() {
    let msgs = ["Hello, world!", "Grüß Gott!", "ハロー・ワールド"];

    for msg in msgs.iter() {
        println!("{}", msg);
    }

    for msg in msgs.iter() {
        println!("{:?}", msg);
    }
}
