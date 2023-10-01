pub fn hello() {
    println!("Hello from infrastructure connectors!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }
}
