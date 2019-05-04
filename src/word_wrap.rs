fn word_wrap(str: String, at: usize) -> String {
    let mut output = str;
    let (first, last) = output.split_at(at);
    first.to_string() + "\n" + last
}

#[test]
fn test_word_wrap(){
    assert_word_wrap("xxx".to_string(), 2, "xx\nx".to_string());
    assert_word_wrap("xxx yyyy".to_string(), 3, "xxx\n yyyy".to_string());
}

fn assert_word_wrap(str: String, at: usize, output: String){
    assert_eq!(output, word_wrap(str, at));
}
