#![feature(use_extern_macros, specialization)]

extern crate pyo3;
extern crate strsim;
use pyo3::prelude::*;


#[pymodinit]
fn strsimrs(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "jaro")]
    fn jaro(a: &str, b: &str) -> f64 {
        strsim::jaro(a, b)
    }

    #[pyfn(m, "jaro_winkler")]
    fn jaro_winkler(a: &str, b: &str) -> f64 {
        strsim::jaro_winkler(a, b)
    }

    #[pyfn(m, "levenshtein")]
    fn levenshtein(a: &str, b: &str) -> usize {
        strsim::levenshtein(a, b)
    }

    #[pyfn(m, "osa_distance")]
    fn osa_distance(a: &str, b: &str) -> usize {
        strsim::osa_distance(a, b)
    }

    #[pyfn(m, "damerau_levenshtein")]
    fn damerau_levenshtein(a: &str, b: &str) -> usize {
        strsim::damerau_levenshtein(a, b)
    }

    #[pyfn(m, "hamming")]
    fn hamming(a: &str, b: &str) -> PyResult<usize> {
        match strsim::hamming(a, b) {
            Ok(count) => Ok(count),
            Err(e) => match e {
                strsim::StrSimError::DifferentLengthArgs => exc::ValueError::new("Arguments have different lenths.").into(),
                _ => exc::ValueError.into()
            }
        }
    }

    Ok(())
}
