pub mod model;
pub mod enums;
pub mod file_meta;
pub mod container_meta;

pub fn hello() {
    println!("Hello from domain models!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }
}
