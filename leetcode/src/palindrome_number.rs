pub fn is_palindrome(x: u32) -> bool {
    println!("{}", x);
    let string = x.to_string();
    let chars = string.chars();
    let reverse = chars.rev();

    reverse.eq(x.to_string().chars())
}
