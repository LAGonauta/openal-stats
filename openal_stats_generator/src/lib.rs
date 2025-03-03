#![allow(non_snake_case)]

mod definitions;
mod al;
mod alc;
mod stats_generator;
mod macros;
mod al_api;

pub fn add(left: u64, right: u64) -> u64 {
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
