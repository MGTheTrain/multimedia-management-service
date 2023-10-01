pub mod video_management_service;

pub fn hello() {
    println!("Hello from domain interfaces!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }
}
