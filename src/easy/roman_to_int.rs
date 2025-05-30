// https://leetcode.com/problems/roman-to-integer/description/

fn get_value(c: char) -> i32 {
    let value = match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    };

    return value;
}

fn roman_to_int(s: String) -> i32 {
    let mut result = 0;

    let mut curr: i32;
    let mut next: i32;
    let mut i=0;

    while i < s.len(){

        curr = get_value(s.chars().nth(i).unwrap());

        if i == s.len() - 1{
            result+=curr;
            i+=1;
            continue;
        }

        next = get_value(s.chars().nth(i+1).unwrap());

        if next > curr{
            result+=next - curr;
            i+=2;
        }
        else{
            result+=curr;
            i+=1;
        }
    }
    return result

}

fn main(){
    let s: String = "III".to_string();

    println!("{}",roman_to_int(s))

}

