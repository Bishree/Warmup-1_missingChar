
fn missingChar (mut s: String, n:usize) -> String {
    s.remove(n);
    s
}

fn main() {
    let mut new_string = "ahmed".parse().unwrap();
    println!("{}", missingChar(new_string, 3));
    println!("{}", missingChar(new_string, 2));
    println!("{}", missingChar(new_string, 4));

}
