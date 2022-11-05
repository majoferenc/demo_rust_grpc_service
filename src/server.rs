use tonic::{transport::Server, Request, Response, Status};

use accounts::account_server::{Account, AccountServer};
use accounts::{PaymentResponse, PaymentRequest};

pub mod accounts {
    tonic::include_proto!("accounts");
}

#[derive(Debug, Default)]
pub struct AccountService {}

#[tonic::async_trait]
impl Account for AccountService {
    async fn send_payment(&self, request: Request<PaymentRequest>,) -> Result<Response<PaymentResponse>, Status> {
        println!("Enter: {:?}", request);
        let req: PaymentRequest = request.into_inner();

        let reply: PaymentResponse = PaymentResponse{
            successful: true,
            message: format!("Request of {} processed successfully from account {} to account {}.", req.amount, req.from, req.to).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let account_service = AccountService::default();

    Server::builder()
        .add_service(AccountServer::new(account_service))
        .serve(addr)
        .await?;

    Ok(())
}
