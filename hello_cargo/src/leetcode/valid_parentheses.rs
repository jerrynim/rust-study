#[allow(dead_code)]
pub fn solution(s: String) -> bool {
    let mut stack = Vec::new();

    for i in s.chars() {
        match i {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}' | ')' | ']' if Some(i) != stack.pop() => return false,
            _ => (),
        }
    }
    return stack.is_empty();
}
