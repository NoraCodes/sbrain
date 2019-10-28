extern crate sbrain;
use sbrain::*;
use std::io::Cursor;

fn compare_output(source: &str, expected: &[u8]) {
    let program = source_to_tape(&source);
    let mut output = sbrain::make_output_vec();
    {
        let mut machine = SBrainVM::new(None, Some(&mut output), &program)
            .expect("Could not build machine");

        machine.load_program(&program).unwrap();
        machine.run(Some(1000)).expect("I/O failed");
    }

    let actual = output.into_inner();
    assert_eq!(
        expected,
        &actual[0..],
        "Expected {:?}, but the machine output {:?}.",
        expected,
        actual
    );
}

fn compare_output_ext(source: &str, input: Vec<u8>, expected: &[u8]) {
    let program = source_to_tape(&source);
    let mut output = sbrain::make_output_vec();
    let mut input = Box::new(Cursor::new(input));
    {
        let mut machine = SBrainVM::new(Some(&mut input), Some(&mut output), &program)
            .expect("Could not build machine");

        machine.load_program(&program).unwrap();
        machine.run(Some(1000)).expect("I/O failed");
    }

    let actual = output.into_inner();
    assert_eq!(
        expected,
        &actual[0..],
        "Expected {:?}, but the machine output {:?}.",
        expected,
        actual
    );
}

#[test]
fn test_transliteration() {
    let source = String::from(
        "[.>]@
                              #comment#",
    );
    let tape = source_to_tape(&source);
    assert_eq!(tape, vec![4, 6, 1, 5, 15]);
}

#[test]
fn test_cat() {
    compare_output_ext(",[.>,]", b"Hello, World!".to_vec(), b"Hello, World!");
}

#[test]
fn test_badloop() {
    compare_output("+[.", &[1]);
    compare_output("+].[.", &[1, 1]);
}

#[test]
fn test_cell_mod() {
    compare_output_ext(",+. >-.", vec![1], &[2, 255]);
}

#[test]
fn test_manual_division() {
    compare_output_ext(
        ",>,>,<<[->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<] >>>>.",
        vec![20, 0, 2],
        &[10],
    );
}

#[test]
fn test_loop() {
    // Count down from 5
    compare_output_ext(",[.-]", vec![5], &[5, 4, 3, 2, 1]);
}

#[test]
fn test_stack() {
    // print, push, forward, pop, print
    compare_output_ext(",.{>}.", vec![1], &[1, 1]);
}

#[test]
fn test_aux() {
    // Put a value in the aux regiser and modify the tape, then pop and check
    // that it's the same
    compare_output_ext(",.(+.).", vec![0], &[0, 1, 0]);
}

#[test]
fn test_auxi_zero() {
    // put a 1 on the tape. It gets turned into a 0.
    compare_output_ext(",(^).", vec![1], &[0]);
}

#[test]
fn test_auxi_bitwise_unary() {
    // (!). tests bitwise NOT
    compare_output_ext(",(!).", vec![0], &[255]);
}

#[test]
fn test_auxi_bitwise_binary() {
    // read, load register, read, operate, write to tape, write out
    compare_output_ext(",(,&).", vec![2, 128], &[0]);
}
