use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2J\x1B[1:1H";

struct Progress<Iter> {
    iter: Iter,
    i: usize,
    bound: Option<usize>,
    delims: (char, char)
}


// This impl blocks associates a method with a type, this function new is associated with the type Iter
// new is a static method because it have no self
// For all types Iter, implements Progress of iter, just like prototype in JS
impl<Iter> Progress<Iter> {
    // This self means wherever it is implemented
    pub fn new(iter: Iter) -> Self {
        Progress { iter, i: 0, bound: None, delims: ('[', ']') }
    }
}

//Add this method to the Progress data structure only the when the type of Iter is ExactSizeIterator
impl<Iter> Progress<Iter> 
where Iter: ExactSizeIterator {
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
} 

impl<Iter> Progress<Iter> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.delims = delims;
        self
    }
}

// This block implements a trait for a type.
// The compiler understands that the Progress data type is an Iterator and have a for loop,
// satisfying the traits requirements
impl<Iter> Iterator for Progress<Iter>
where Iter: Iterator {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        match self.bound {
            Some(bound) => println!(
                "{}{}{}{}",
                 self.delims.0,
                 "#".repeat(self.i),
                 " ".repeat(bound-self.i),
                 self.delims.1),
            None => println!("{}", "#".repeat(self.i))
        };
        self.i += 1;
        self.iter.next()
    }
}


trait ProgressIteratorExt: Sized{
    fn progress(self) -> Progress<Self>;
}

// For all types Iter, implements the trait for the quantified Item.
// Cool thing number one
impl<Iter> ProgressIteratorExt for Iter
where Iter: Iterator {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let brkts = ('<', '>');
    //for n in (0 .. ).progress() {
    //    expensive_calculation(&n);
   //}

    let v = vec![1, 2, 3];
    for n in Progress::new(v.iter().progress().with_bound().with_delims(brkts)) {
        expensive_calculation(n);
    }
}
