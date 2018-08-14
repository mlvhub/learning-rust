#[derive(Debug)]
struct Collatz {
    current: u64,
    end: u64,
}

impl Collatz {
    fn new(start: u64) -> Collatz {
        Collatz {
            current: start,
            end: 1,
        }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.current % 2 == 0 {
            self.current = self.current / 2;
        } else {
            self.current = 3 * self.current + 1;
        }

        if self.current == self.end {
            None
        } else {
            Some(self.current)
        }
    }
}

fn main() {
    let input = 10;

    for n in Collatz::new(input).take(2) {
        println!("{}", n);
    }

    for n in Collatz::new(input).skip(2) {
        println!("{}", n);
    }

    let mut collatz = Collatz::new(input);

    println!("While");
    while let Some(n) = collatz.next() {
        println!("{}", n);
    }
    println!("{}", collatz.end);
}
