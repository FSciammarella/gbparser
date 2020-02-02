pub mod gbreader;
pub mod range;
pub mod gbrecord;
//pub mod gbfilter;

pub(crate) mod gbparser;
pub(crate) mod alphabet;
pub(crate) mod conformation;
pub(crate) mod references;
pub(crate) mod features;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
