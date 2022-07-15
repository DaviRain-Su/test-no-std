#![no_std]
extern crate eyre;
extern crate ibc;
extern crate anyhow;
use anyhow::*;
use ibc::*;
use eyre::*;

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
