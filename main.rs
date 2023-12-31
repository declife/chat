mod client;

fn main() {
    println!("Hello, World");
    let str = "fdgllf";
    let ans = smallest_palindrome(str);
    print!("result = {}", call());
    
}

fn call() -> i32 {
    let a = 1;
    let mut b = 2;

    let c = a + b;
    return c;
}

pub fn smallest_palindrome(s: &str) -> String {
    let mut ans = String::new();

    for (x, y) in s.chars().zip(s.chars().rev()) {
        ans.push(x.min(y))
    }

    ans
}
