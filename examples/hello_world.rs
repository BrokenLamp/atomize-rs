use atomize::{a, Atom};

fn main() {
    // a!(apple) will always make the same value
    let apple = a!(apple);

    // They can be compared in O(1) time
    assert_eq!(apple, a!(apple));

    // In fact, they compile to simple u64 and so
    // are compared in a single x64 operation

    // Here are some values
    println!("apple          : {}", a!(apple));
    println!("orange         : {}", a!(orange));
    println!("watermelon     : {}", a!(watermelon));

    // Values can also be mixed
    println!("apple + orange : {}", a!(apple) + a!(orange));
}
