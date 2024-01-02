fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn is_nice(input: &str) -> bool {
    false
}

fn process(input: &str) -> usize {
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

    let mut nice_counter = 0;
    for line in lines {
        println!("{}", line);
        if is_nice(line) {
            nice_counter += 1;
        }
    }

    nice_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = process("ugknbfddgicrmopn");
        assert_eq!(result, 1);
    }
    #[test]
    fn part1_test2() {
        let result = process("jchzalrnumimnmhp");
        assert_eq!(result, 1);
    }
    #[test]
    fn part1_test3() {
        let result = process("haegwjzuvuyypxyu");
        assert_eq!(result, 1);
    }
    #[test]
    fn part1_test4() {
        let result = process("dvszwmarrgswjxmb");
        assert_eq!(result, 1);
    }
}
