use std::collections::VecDeque;

fn main() {
    palindrome(&mut String::new());
}

fn palindrome(word: &mut String) -> bool {
    println!("Please input a word and we will check its palindromity: ");
    std::io::stdin()
        .read_line(word)
        .expect("Failed to read the input provided");

    let mut word_vec_stack: Vec<char> = vec![' '];
    word_vec_stack.pop();
    for i in word.trim().chars().into_iter() {
        word_vec_stack.push(i);
    }

    // println!("{:?}", word_vec_stack);

    let mut buffer_word_vec_stack: VecDeque<char> = VecDeque::new();

    for i in &word_vec_stack {
        let char = *i;
        buffer_word_vec_stack.push_front(char);
    }

    println!("{:?}", buffer_word_vec_stack);

    //println!("buffer stack: {:?}", buffer_word_vec_stack);

    let mut palindrome_vec_stack_checker = vec![' '];
    palindrome_vec_stack_checker.pop();
    for i in &word_vec_stack {
        palindrome_vec_stack_checker.push(*i);
    }

    println!("{:?}", palindrome_vec_stack_checker);

    let palindrome_vec_stack_checker_vecdeque: VecDeque<char> = VecDeque::from(palindrome_vec_stack_checker);

    let is_palindrome = palindrome_vec_stack_checker_vecdeque == buffer_word_vec_stack;

    
    println!("Palindromity final status check: {}", is_palindrome);
    is_palindrome


}
