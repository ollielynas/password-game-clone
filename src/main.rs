use rand::prelude::*;
use sycamore::prelude::*;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Condition {
    LargerThan(i32),
    SmallerThan(i32),
    ContainsDigit(i32),
    SumOfDigits(i32),
    HasFactor(i32),
    HasMultiple(i32),
    IsSquare(bool),
    IsCube(bool),
    IsPalindrome(bool),

}

fn digits(num: i32) -> impl Iterator<Item = i32> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>()
        .into_iter()
}
impl Condition {
    fn check(&self, num: i32) -> bool {
        match self {
            Condition::LargerThan(n) => num > *n,
            Condition::SmallerThan(n) => num < *n,
            Condition::ContainsDigit(n) => digits(num).any(|d| d == *n),
            Condition::SumOfDigits(n) => digits(num).sum::<i32>() == *n,
            Condition::HasFactor(n) => num % n == 0,
            Condition::HasMultiple(n) => n % num == 0,
            Condition::IsSquare(n) => {
                let root = (num as f64).sqrt() as i32;
                (root * root == num) == *n
            }
            Condition::IsCube(n) => {
                let root = (num as f64).cbrt() as i32;
                (root * root * root == num) == *n
            }
            Condition::IsPalindrome(n) => {
                let digits = digits(num).collect::<Vec<_>>();
                digits.iter().eq(digits.iter().rev())==*n
            }
        }
    }

    fn as_valid(&self, num: i32) -> Self {
        let mut rng = rand::thread_rng();
        match self {
            Condition::LargerThan(_) => Condition::LargerThan(num - rng.gen_range(1..num)),
            Condition::SmallerThan(_) => Condition::SmallerThan(num + rng.gen_range(1..200)),
            Condition::ContainsDigit(_) => Condition::ContainsDigit(
                digits(num).collect::<Vec<_>>().choose(&mut rng).unwrap().clone(),
            ),
            Condition::SumOfDigits(_) => Condition::SumOfDigits(digits(num).sum::<i32>()),
            Condition::HasFactor(_) => Condition::HasFactor(
                (1..=num)
                    .filter(|i| num % i == 0)
                    .collect::<Vec<_>>()
                    .choose(&mut rng)
                    .unwrap()
                    .clone(),
            ),
            Condition::HasMultiple(_) => Condition::HasMultiple(
                num*rng.gen_range(1..10)
            ),
            Condition::IsSquare(_) => {
                let root = (num as f64).sqrt() as i32;
                Condition::IsSquare(root * root == num)
            },
            Condition::IsCube(_) => {
                let root = (num as f64).cbrt() as i32;
                Condition::IsCube(root * root * root == num)
            },
            Condition::IsPalindrome(_) => {
                let digits = digits(num).collect::<Vec<_>>();
                Condition::IsPalindrome(digits.iter().eq(digits.iter().rev()))
            },
        }
    }

    fn as_string(&self) -> String {
        match self {
            Condition::LargerThan(n) => format!("must be larger than {}", n),
            Condition::SmallerThan(n) => format!("must be smaller than {}", n),
            Condition::ContainsDigit(n) => format!("the number mus contain the get digit {}", n),
            Condition::SumOfDigits(n) => format!("the digits of the number must sum to {}", n),
            Condition::HasFactor(n) => format!("the number must have the factor {}", n),
            Condition::HasMultiple(n) => format!("the number must be a factor of {}", n),
            Condition::IsSquare(n) => format!("the number must {} be a square", if *n {""} else {"not"}),
            Condition::IsCube(n) => format!("the number must {} be a cube", if *n {""} else {"not"}),
            Condition::IsPalindrome(n) => format!("the number must {} be a palindrome", if *n {""} else {"not"}),

        }
    }
}

fn main() {
    sycamore::render(|cx| {

        let mut rng = rand::thread_rng();
        let number = rng.gen_range(100..10000);
        
        let mut unseen: Vec<Condition> = vec![
            Condition::LargerThan(0),
            Condition::SmallerThan(0),
            Condition::ContainsDigit(0),
            Condition::SumOfDigits(0),
            Condition::HasFactor(0),
            Condition::HasMultiple(0),
            Condition::IsSquare(false),
            Condition::IsCube(false),
            Condition::IsPalindrome(false),

        ];
        unseen.iter_mut().for_each(|i| *i = i.as_valid(number));
        unseen.shuffle(&mut rng);
        let mut seen: Vec<Condition> = vec![];

        let num = create_signal(cx, None::<i32>);
        let value = create_signal(cx, "".to_string());
        let text = create_signal(cx, "".to_string());

        create_effect(cx, move || {
            let a = value.get();
            let b = a.replace(" ", "").parse::<i32>();
            let mut valid_num =num.get_untracked().unwrap_or(0);
            match b {
                Ok(b) => {num.set(Some(b));
                    valid_num=b;
                },
                Err(_) => {}
            }

            while unseen.len() > 0 && seen.iter().all(|i| i.check(valid_num)) {
                seen.push(unseen.pop().unwrap());
            }
            seen.sort_by(|c, d| c.check(valid_num).cmp(&d.check(valid_num)));

            let mut new_text = String::new();
            for i in seen.iter() {
                new_text.push_str(&format!("<div class='{}'>{}</div>", i.check(valid_num), i.as_string()));
            }
            text.set(new_text);

            
        });
        view! { cx,
                    style {(include_str!("style.css"))}
                    div {
                        h1 {"Guess the Number"}
                    input(inputmode="numeric", pattern="[0-9]*", type="text", bind:value=value)
                    p{(format!("{}",num.get().unwrap_or(0)))}
                }
                div(class="valid", dangerously_set_inner_html=&text.get())
        }
    });
}
