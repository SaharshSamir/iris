use crate::prisma::PrismaClient;
use anyhow::Result;
use prisma_client_rust::NewClientError;

pub async fn new_client() -> Result<PrismaClient, NewClientError> {
    let client = PrismaClient::_builder().build().await;

    return client;
}
