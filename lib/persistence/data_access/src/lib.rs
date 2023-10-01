pub fn hello() {
    println!("Hello from persistence data_access!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }
}
