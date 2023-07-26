mod leetcode;

fn main() {
    //14. Longest Common Prefix
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let answer = leetcode::longest_common_prefix::solution(strings);
    //13. Roman to Integer
    // let string = String::from("MCMXCIV");
    // let answer = leetcode::roman_to_integer::solution(string);

    //9.
    // let answer = leetcode::palindrome_number::is_palindrome(32);

    //2. Add Two Numbers
    // let answer = leetcode::add_two_numbers::solution([9, 9, 9, 9, 9, 9, 9], [9, 9, 9, 9]);
    //1.
    // let answer = leetcode::two_sum::two_sum([2, 7, 11, 15].to_vec(), 9);

    println!("Answer is: {:?}", answer);
}
