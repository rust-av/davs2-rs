#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/davs2.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn version() {
        assert_eq!(2 + 2, 4);
    }
}
