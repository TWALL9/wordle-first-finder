use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Default, Debug)]
struct Wordle {
    letter: char,
    counts: [i32; 5],
}

impl PartialEq for Wordle {
    fn eq(&self, other: &Self) -> bool {
        let sum: i32 = self.counts.iter().sum();
        let other_sum = other.counts.iter().sum();
        sum == other_sum
    }
}

impl Eq for Wordle {}

impl PartialOrd for Wordle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Wordle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sum: i32 = self.counts.iter().sum();
        let other_sum = other.counts.iter().sum();

        if sum > other_sum {
            std::cmp::Ordering::Greater
        } else if sum < other_sum {
            std::cmp::Ordering::Less
        } else {
            self.letter.cmp(&other.letter)
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut words = HashMap::new();
    let mut sorted = Vec::new();

    for c in 'A'..='Z' {
        words.insert(
            c,
            Wordle {
                letter: c,
                counts: [0; 5],
            },
        );
    }

    for line in read_to_string("wordle_answers.txt")?.lines() {
        let mut i = 0;
        for c in line.chars() {
            if let Some(w) = words.get_mut(&c) {
                w.counts[i] += 1;
            }
            i += 1;
        }
    }

    for w in words.iter() {
        sorted.push(w.1);
    }

    for i in 0..5 {
        print_sort(&mut sorted, i);
    }

    Ok(())
}

fn print_sort(v: &mut Vec<&Wordle>, index: usize) {
    v.sort_by_key(|c| c.counts[index]);
    v.reverse();

    for w in v[0..10].iter() {
        print!("{:?}: {:?}, ", w.letter, w.counts[index]);
    }
    println!("");
}
