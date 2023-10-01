pub fn hello() {
    println!("Hello from infrastructure settings!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }
}
