use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2J\x1B[1:1H";

struct Progress<Iter> {
    iter: Iter,
    i: usize
}


// This impl blocks associates a method with a type, this function new is associated with the type Iter
// new is a static method because it have no self
// For all types Iter, implements Progress of iter, just like prototype in JS
impl<Iter> Progress<Iter> {
    // This self means wherever it is implemented
    pub fn new(iter: Iter) -> Self {
        Progress { iter, i: 0 }
    }
}

// This block implements a trait for a type.
// The compiler understands that the Progress data type is an Iterator and have a for loop,
// satisfying the traits requirements
impl<Iter> Iterator for Progress<Iter>
where Iter: Iterator {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "#".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}


//The where clause is located as an heritance from the function?
fn progress<Iter>(iter: Iter, f: fn(Iter::Item) -> ()) 
where Iter: Iterator {
    let mut i = 1;
    for n in iter {
        println!("{}{}", CLEAR, "#".repeat(i));
        i += 1;
        f(n);
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];
    for n in Progress::new(v.iter()) {
        expensive_calculation(n);
    }
}
