use passwords::analyzer;
use passwords::scorer;

pub fn analyse_password(password: &str) -> u8 {
    let score = scorer::score(&analyzer::analyze(password));
    println!("Password score: {score}");
    score as u8
}
