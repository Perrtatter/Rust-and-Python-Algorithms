struct Tower {
    n: i8,
}

impl Tower {
    fn build(n: i8) -> Tower {
        Tower { n: n }
    }

    fn hanoi(&self, n: i8, a: &str, c: &str, b: &str) {
        if n == 1 {
            println!("{}->{}", a, b);
        } else {
            self.hanoi(n - 1, a, b, c);
            self.hanoi(1, a, c, b);
            self.hanoi(n - 1, b, c, a);
        }
    }
}

fn main() {
    let tower = Tower::build(3);
    tower.hanoi(3, "a", "c", "b");
}