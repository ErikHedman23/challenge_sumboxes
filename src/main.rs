use std::ops::Add;

fn sum_boxes<T: Add<Output = T>>(value1: Box<T>, value2: Box<T>) -> Box<T> {
    return Box::new(*value1 + *value2);
}

fn main() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);

    println!("Tests passed!");
}
