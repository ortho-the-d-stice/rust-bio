// Copyright 2014-2016 Johannes Köster.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # Rust-bio, a bioinformatics library for Rust.
//! This library provides implementations of many algorithms and data structures
//! that are useful for bioinformatics.
//! All provided implementations are rigorously tested via continuous
//! integration.
//! For installation instructions and a general overview, visit
//! https://rust-bio.github.io.
//!
//! Currently, rust-bio provides
//!
//! * most major pattern matching algorithms,
//! * a convenient alphabet implementation,
//! * pairwise alignment,
//! * suffix arrays,
//! * BWT and FM-Index,
//! * FMD-Index for finding supermaximal exact matches,
//! * a q-gram index,
//! * an orf research algorithm,
//! * a rank/select data structure,
//! * FASTQ and FASTA and BED readers and writers,
//! * helper functions for combinatorics and dealing with log probabilities.
//!
//! # Example
//!
//! ```rust
//! use bio::alphabets;
//! use bio::data_structures::suffix_array::suffix_array;
//! use bio::data_structures::bwt::{bwt, less, Occ};
//! use bio::data_structures::fmindex::{FMIndex, FMIndexable};
//!
//! let text = b"ACGGATGCTGGATCGGATCGCGCTAGCTA$";
//! let pattern = b"ACCG";
//!
//! // Create an FM-Index for a given text.
//! let alphabet = alphabets::dna::iupac_alphabet();
//! let sa = suffix_array(text);
//! let bwt = bwt(text, &sa);
//! let less = less(&bwt, &alphabet);
//! let occ = Occ::new(&bwt, 3, &alphabet);
//! let fmindex = FMIndex::new(&bwt, &less, &occ);
//!
//! let interval = fmindex.backward_search(pattern.iter());
//! let positions = interval.occ(&sa);
//! ```
//!
//! # Multithreaded Example
//!
//! ```rust
//! use bio::alphabets;
//! use bio::data_structures::suffix_array::suffix_array;
//! use bio::data_structures::bwt::{bwt, less, Occ};
//! use bio::data_structures::fmindex::{FMIndex, FMIndexable};
//! use std::sync::Arc;
//! use std::thread;
//!
//! let text = b"ACGGATGCTGGATCGGATCGCGCTAGCTA$";
//! let patterns = vec![b"ACCG", b"TGCT"];
//!
//! // Create an FM-Index for a given text.
//! let alphabet = alphabets::dna::iupac_alphabet();
//! let sa = suffix_array(text);
//! let bwt = Arc::new(bwt(text, &sa));
//! let less = Arc::new(less(bwt.as_ref(), &alphabet));
//! let occ = Arc::new(Occ::new(bwt.as_ref(), 3, &alphabet));
//! let fmindex = Arc::new(FMIndex::new(bwt, less, occ));
//!
//! // Spawn threads to perform backward searches for each interval
//! let interval_calculators = patterns.into_iter().map(|pattern| {
//!     let fmindex = fmindex.clone();
//!     thread::spawn(move ||
//!         fmindex.backward_search(pattern.iter())
//!     )
//! }).collect::<Vec<_>>();
//!
//! // Loop through the results, extracting the positions array for each pattern
//! for interval_calculator in interval_calculators {
//!     let positions = interval_calculator.join().unwrap().occ(&sa);
//! }
//! ```
//!
//! Documentation and further examples for each module can be found in the module descriptions below.

#![cfg_attr(feature = "serde_macros", feature(const_fn, custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

#[cfg(feature = "serde_macros")]
extern crate serde;

extern crate rustc_serialize;
extern crate csv;
extern crate num_traits;
extern crate num_integer;
extern crate itertools;
extern crate itertools_num;
extern crate bit_vec;
extern crate vec_map;
extern crate bit_set;
#[macro_use]
extern crate lazy_static;
extern crate nalgebra;
#[macro_use]
extern crate approx;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate newtype_derive;
extern crate ordered_float;
#[macro_use]
extern crate quick_error;
extern crate flate2;

pub mod utils;
pub mod alphabets;
pub mod pattern_matching;
pub mod data_structures;
pub mod alignment;
pub mod io;
pub mod seq_analysis;
pub mod stats;
pub mod scores;
