fn main() {
    let input: &str = include_str!("./input1.txt");
    let output: String = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: String = process("");
        assert_eq!(result, "");
    }
}
