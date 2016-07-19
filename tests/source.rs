extern crate sbrain;
use sbrain::*;

fn compare_output(source: &str, expected_output: &str) {
    let (p, d) = source_to_tapes(&source);
    let mut machine = SBrainVM::new(Some(vec![]));
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

fn compare_vec_output(source: &str, data_tape: Vec<MData>, expected_output: Vec<MData>) {
    let (p, _) = source_to_tapes(&source);
    let mut machine = SBrainVM::new(Some(vec![]));
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
    let tapes = source_to_tapes(&source);
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

#[test]
fn test_loop() {
    // Count down from 5
    compare_vec_output("[.-]", vec![5], vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_stack() {
    // print, push, forward, pop, print
    compare_vec_output(".{>}.", vec![1], vec![1, 1]);
}

#[test]
fn test_aux() {
    // Put a value in the aux regiser and modify the tape, then pop and check
    // that it's the same
    compare_vec_output(".(+.).", vec![0], vec![0, 1, 0]);
}

#[test]
fn test_auxi_zero() {
    // put a 1 on the tape. It gets turned into a 0.
    compare_vec_output("(z).", vec![1], vec![0]);
}

#[test]
fn test_auxi_arithmetic() {
    compare_vec_output("(a. > (d. > (m. > (q. > (p.",
                       vec![1, 1, 2, 4, 5],
                       vec![2, 0, 0, 1, 25]);
}

#[test]
fn test_auxi_bitwise_unary() {
    // (!). tests bitwise NOT
    // (s). tests bit shift left
    // (S). tests bit shift right
    compare_vec_output("(!). > (s). > (S).",
                       vec![1024, 2, 2],
                       vec![4294966271, 4, 1]);
}

#[test]
fn test_auxi_bitwise_binary() {
    // Each of the four tests operates on 2 and 1024:
    // load 2, advance to the cell containing 1024, operate, print, then move on to the next 2.
    compare_vec_output("(>|. > (>&. > (>*. > (>^.",
                       vec![2, 1024, 2, 1024, 2, 1024, 2, 1024],
                       vec![1026, 0, 1026, 4294966269]);
}
