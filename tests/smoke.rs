use atomize::{a, Atom};

#[test]
fn it_works() {
    assert_eq!(a!(apple), a!(apple));
    assert_ne!(a!(apple), a!(orange));
}

#[test]
fn atom_struct() {
    assert_eq!(a!(apple), Atom::from("apple"));
}

#[test]
fn mixing() {
    assert_eq!(a!(apple) + a!(orange), a!(orange) + a!(apple));
    assert_ne!(a!(apple) + a!(orange), a!(orange));
    assert_ne!(a!(apple) + a!(orange), a!(watermelon));
}
