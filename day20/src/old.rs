use std::fs;

trait Dirty<T> {
    fn get(&self) -> T;
    fn set_dirty(&mut self, dirty: bool);
    fn is_dirty(&self) -> bool;
}

#[derive(Copy, Clone)]
struct Value {
    x: i64,
    dirty: bool,
}

impl Value {
    fn new(x: i64) -> Self {
        Self { x, dirty: false }
    }
}

impl Dirty<i64> for Value {
    fn get(&self) -> i64 {
        self.x
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}

fn next_index(i: i64, val: i64, n: i64) -> i64 {
    if i + val > 0 {
        (i + val) % (n - 1)
    } else {
        (n - 1) + (i + val) % (n - 1)
    }
}

fn main() {
    let content = fs::read_to_string("example").unwrap();
    let values = content
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let n = values.len();

    let key = 811589153;
    let mut xs = values
        .iter()
        .copied()
        .map(|x| x * key)
        .map(Value::new)
        .collect::<Vec<_>>();

    let mut i = 0;
    while i < xs.len() {
        if xs[i].is_dirty() {
            i += 1;
        } else if xs[i].get() == 0 {
            xs[i].set_dirty(true);
        } else {
            xs[i].set_dirty(true);

            let val = xs[i];
            let j = next_index(i as i64, xs[i].get(), n as i64) as usize;
            if j > i {
                for k in i..j {
                    xs[k] = xs[k + 1];
                }
                xs[j] = val;
            } else {
                for k in (j..i).rev() {
                    xs[k + 1] = xs[k];
                }
                xs[j] = val;
            }
        }
    }

    let offset = xs.iter().enumerate().find(|(_, v)| v.get() == 0).unwrap().0;
    let res = [1000, 2000, 3000]
        .iter()
        .map(|i| xs[(i + offset) % n].get())
        .sum::<i64>();

    println!("1: {}", res);
}
