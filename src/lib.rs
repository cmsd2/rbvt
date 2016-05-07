#[macro_use] extern crate quick_error;

pub mod validation;
pub mod params;
pub mod result;
pub mod field;
pub mod state;
pub mod builder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
