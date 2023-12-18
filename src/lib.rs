pub mod client;
pub mod res_struct;

pub fn new(url_str: &str) -> Result<client::SupervisorXmlrpcBuilder, url::ParseError> {
    match dxr_client::Url::parse(url_str) {
        Ok(url) => Ok(client::SupervisorXmlrpcBuilder::new(url)),
        Err(e) => Err(e),
    }
}

pub fn url(url_str: &str) -> client::SupervisorXmlrpc {
    new(url_str).unwrap().build()
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
