pub fn hello() {
    println!("Hello from persistence migrations!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }
}
