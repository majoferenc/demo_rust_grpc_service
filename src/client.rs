use accounts::account_client::AccountClient;
use accounts::PaymentRequest;

pub mod accounts {
    tonic::include_proto!("accounts");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AccountClient::connect("http://[::1]:50051").await?;
    let req = tonic::Request::new(
        PaymentRequest {
            from: "from_addr".to_owned(),
            to: "to_addr".to_owned(),
            amount: 100,
        }
    );

    let response = client.send_payment(req).await?;

    println!("Response={:?}", response);

    Ok(())
}
