// Problem: https://leetcode.com/problems/backspace-string-compare/submissions/

impl Solution {
   pub fn backspace_compare(s: String, t: String) -> bool {      
       pub fn logic(str: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        for c in str.chars() { 
            if(c != '#'){
                stack.push(c);
            } else if(!stack.is_empty()){
                stack.pop();
            }
        }
        stack.into_iter().collect()
    }
       
        let x = logic(s) == logic(t);
        print!("{}", x);
        x
    }
}
