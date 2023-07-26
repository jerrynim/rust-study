#[allow(dead_code)]
pub fn solution(strs: Vec<String>) -> String {
    if let Some(first_str) = strs.first() {
        for (i, ch) in first_str.char_indices() {
            for str_ in &strs {
                //?unwrap_or_default 값이 없는 경우, i32의 기본값인 0을 반환 d
                if str_.chars().nth(i).unwrap_or_default() != ch {
                    return first_str[..i].to_string();
                }
            }
        }
        return first_str.clone();
    }
    String::from("")
}
