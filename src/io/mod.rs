//! Readers and writers for common bioinformatics file formats.


pub mod fastq;
pub mod fasta;
pub mod bed;
pub mod gff;

use std::io as std_io;
use std::io::{ Seek, Read, SeekFrom };
use std::fs::File;
use flate2::read::GzDecoder;

use self::fasta::{ Reader as FAReader, Sequences as FASequences };
use self::fastq::{ Reader as FQReader, Sequences as FQSequences };

/// Strand information.
#[derive(Debug, PartialEq)]
pub enum Strand {
    Forward,
    Reverse,
    Unknown,
}

/// wrapper for Sequences interface to Fasta & Fastq readers
pub enum SeqReader<R: Read> {
    Nil,
    Fasta(FASequences<R>),
    Fastq(FQSequences<R>),
    GzFasta(FASequences<GzDecoder<R>>),
    GzFastq(FQSequences<GzDecoder<R>>),
}

impl<R: Read> SeqReader<R> {

    /// open specified file, automatically testing for gzip
    /// and determining whether file is Fasta or Fastq
    pub fn open(fname: &str) -> SeqReader<File> {

        // first test for gzip
        let mut magic_num = [0u8; 2];
        let mut f = File::open(fname).unwrap();
        let _ = f.read_exact(&mut magic_num).unwrap();
        let _ = f.seek(SeekFrom::Start(0));  // rewind

        if magic_num == [0x1fu8, 0x8bu8] {
            // 1f 8b is the magic number of a gzip file

            // now grab the first byte to choose between Fasta and Fastq
            let mut gunz = GzDecoder::new(f).unwrap();
            let _ = gunz.read_exact(&mut magic_num).unwrap();

            // re-open the file, since GzDecoder doesn't have a seek()
            let f = File::open(fname).unwrap();
            let gunz = GzDecoder::new(f).unwrap();

            if magic_num[0] == '>' as u8 {
                SeqReader::GzFasta(FAReader::new(gunz).sequences())
            } else if magic_num[0] == '@' as u8 {
                SeqReader::GzFastq(FQReader::new(gunz).sequences())
            } else {
                panic!("");
            }
        } else {
            if magic_num[0] == '>' as u8 {
                SeqReader::Fasta( FAReader::new(f).sequences() )
            } else if magic_num[0] == '@' as u8 {
                SeqReader::Fastq(FQReader::new(f).sequences())
            } else {
                panic!("");
            }
        }
    }
}

impl<R: std_io::Read> Iterator for SeqReader<R> {
    type Item = std_io::Result<(String,String)>;

    fn next(&mut self) -> Option<std_io::Result<(String,String)>> {
        match self {
            &mut SeqReader::Fasta(ref mut rdr) => rdr.next(),
            &mut SeqReader::Fastq(ref mut rdr) => rdr.next(),
            &mut SeqReader::GzFasta(ref mut rdr) => rdr.next(),
            &mut SeqReader::GzFastq(ref mut rdr) => rdr.next(),
            _ => unimplemented!(),
        }
    }
}
