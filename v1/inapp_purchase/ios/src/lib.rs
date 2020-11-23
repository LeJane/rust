pub mod models;
pub mod validator;

#[cfg(test)]
mod tests {
    use crate::models::IAPRequest;
    use crate::validator::ReqClient;

    #[tokio::test]
    async fn it_works() {
        let data = IAPRequest {
            receipt_data: "asdfadsf".into(),
            password: "adsf".into(),
            exclude_old_transactions: false,
        };

        let res = match ReqClient::new().verify(data).await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("verify error:{:?}", e);
                return;
            }
        };

        println!("res:{:?}", std::str::from_utf8(&res).unwrap());
    }
}
