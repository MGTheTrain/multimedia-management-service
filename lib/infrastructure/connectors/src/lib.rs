// pub mod blob_storage_connector;
pub mod azure_blob_storage_account_connector;

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
