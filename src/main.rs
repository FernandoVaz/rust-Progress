use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2J\x1B[1:1H";

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
}


trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}


impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "#".repeat(progress.i))
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
            println!("{}{}{}{}",
                self.delims.0, 
                "#".repeat(progress.i),
                " ".repeat(self.bound - progress.i), 
                self.delims.1);
    }
}


// This impl blocks associates a method with a type, this function new is associated with the type Iter
// new is a static method because it have no self
// For all types Iter, implements Progress of iter, just like prototype in JS
impl<Iter> Progress<Iter, Unbounded> {
    // This self means wherever it is implemented
    pub fn new(iter: Iter) -> Self {
        Progress { iter, i: 0, bound: Unbounded }
    }
}

//Add this method to the Progress data structure only the when the type of Iter is ExactSizeIterator
impl<Iter> Progress<Iter, Unbounded> 
where Iter: ExactSizeIterator {
    pub fn with_bound(mut self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']')
        };
        Progress { i: self.i, iter: self.iter, bound}
    }
} 

impl<Iter> Progress<Iter, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

// This block implements a trait for a type.
// The compiler understands that the Progress data type is an Iterator and have a for loop,
// satisfying the traits requirements
impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where Iter: Iterator, Bound: ProgressDisplay {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        self.bound.display(&self);
        self.i += 1;

        self.iter.next()
    }
}


trait ProgressIteratorExt: Sized{
    fn progress(self) -> Progress<Self, Unbounded>;
}

// For all types Iter, implements the trait for the quantified Item.
// Cool thing number one
impl<Iter> ProgressIteratorExt for Iter
where Iter: Iterator {
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let brkts = ('<', '>');
    for n in (0 .. ).progress() {
        expensive_calculation(&n);
    }

    let v = vec![1, 2, 3];
    for n in Progress::new(v.iter().progress().with_bound().with_delims(brkts)) {
        expensive_calculation(n);
    }
}
