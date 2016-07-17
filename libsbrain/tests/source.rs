extern crate libsbrain;
use libsbrain::{source, machine};

fn compare_output(source: &str, expected_output: &str) {
    let (p, d) = source::source_to_tapes(&source);
    let mut machine = machine::SBrainVM::new(vec![]);
    machine.load_program(&p).unwrap();
    machine.load_data(&d).unwrap();
    machine.run(Some(1000));
    let mut expected = Vec::with_capacity(expected_output.len());
    for c in expected_output.chars() {
        expected.push(c as u32);
    }
    let actual = machine.get_output();
    print!("Output: ");
    for c in &actual {
        print!("{}", *c as u8 as char);
    }
    println!("");
    assert_eq!(expected, actual);
}

fn compare_vec_output(source: &str,
                      data_tape: Vec<machine::MData>,
                      expected_output: Vec<machine::MData>) {
    let (p, _) = source::source_to_tapes(&source);
    let mut machine = machine::SBrainVM::new(vec![]);
    machine.load_program(&p).unwrap();
    machine.load_data(&data_tape).unwrap();
    machine.run(Some(1000));
    let actual = machine.get_output();
    println!("Output: {:?}", actual);
    assert_eq!(expected_output, actual);
}

#[test]
fn test_transliteration() {
    let source = String::from("[.>]
                              #comment#
                              #these two should not trigger a transition to data mode @@#
                              @@Hello, World!");
    let tapes = source::source_to_tapes(&source);
    assert_eq!(tapes,
               (vec![4, 6, 1, 5, 31],
                vec![72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]));
}

#[test]
fn test_hello_world() {
    compare_output("[.>]@@Hello, World!", "Hello, World!");
}

#[test]
fn test_cell_mod() {
    compare_vec_output("+. >-.", vec![1, 1], vec![2, 0]);
}
