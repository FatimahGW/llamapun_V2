//! # The LLaMaPUn library in Rust
//! This is an attempt to reimplement the LLaMaPUn library in Rust.
//! The original library can be found at https://github.com/KWARC/LLaMaPUn

#![feature(slice_patterns)]
#![feature(collections)]

extern crate rustlibxml;
extern crate libc;
extern crate regex;
extern crate unidecode;
extern crate gnuplot;
extern crate rustmorpha;

pub mod dnmlib;
pub mod stopwords;
pub mod tokenizer;
