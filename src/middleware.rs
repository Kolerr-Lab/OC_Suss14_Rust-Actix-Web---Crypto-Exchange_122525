use actix_service::Service;
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, HttpMessage};
use std::time::Instant;

pub async fn logging_middleware<S>(req: ServiceRequest, srv: &S) -> Result<ServiceResponse, Error>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    let start_time = Instant::now();
    let res = srv.call(req).await?;
    let duration = start_time.elapsed();
    println!(\