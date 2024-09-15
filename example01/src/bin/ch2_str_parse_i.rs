fn main() {
    let s = "365";
    let i: i32 = match s.parse() {
        Ok(v) => v,
        Err(_) => 0,
    };
    println!("{}", i);
}