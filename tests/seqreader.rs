extern crate bio;

use std::fs::File;
use bio::io::SeqReader;

#[test]
fn test() {
    let mut reader = SeqReader::<File>::new("tests/test.fa.gz");
    assert_eq!( reader.next().unwrap().unwrap(), (String::from(">some_gene\n"), String::from("ATGCATGCAATCA")) );
}
