extern crate libsbrain;
use libsbrain::source;
#[test]
fn test_transliteration(){
    let source = String::from("[.>]
                              #comment#
                              #these two should not trigger a transition to data mode @@#
                              @@Hello, World!");
    let tapes = source::source_to_tapes(&source);
    assert_eq!(tapes, (vec![4, 6, 1, 5, 31], vec![72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]));
}
