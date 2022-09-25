extern crate intervals;

fn main() {
    use intervals::{Interval, bounds};

    let i1 = Interval::new(
        bounds::Closed(0),
        bounds::Open(100usize)
    ).unwrap();
    let i2 = Interval::new(
        bounds::Open(10),
        bounds::Closed(100usize)
    ).unwrap();

    println!("{}", i1.intersect(i2));
}
