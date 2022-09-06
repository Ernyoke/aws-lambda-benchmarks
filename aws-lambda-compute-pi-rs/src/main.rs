use lambda_runtime::{Error, LambdaEvent, service_fn};
use num_bigint::{BigInt, ToBigInt};
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    digits: i32,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    digits: Vec<i32>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();

    let handler = service_fn(handler);
    Ok(lambda_runtime::run(handler).await?)
}

pub(crate) async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let resp = Response {
        req_id: event.context.request_id,
        digits: generate_pi(event.payload.digits),
    };

    Ok(resp)
}

fn generate_pi(limit: i32) -> Vec<i32> {
    let mut q = 1.to_bigint().unwrap();
    let mut r = 180.to_bigint().unwrap();
    let mut t = 60.to_bigint().unwrap();
    let mut i = 2.to_bigint().unwrap();
    let mut res: Vec<i32> = Vec::new();
    for _ in 0..limit {
        let digit: BigInt = ((&i * 27 - 12) * &q + &r * 5) / (&t * 5);
        res.push(digit.to_i32().unwrap());
        let mut u: BigInt = &i * 3;
        u = (&u + 1) * 3 * (&u + 2);
        r = &u * 10 * (&q * (&i * 5 - 2) + r - &t * digit);
        q *= 10 * &i * (&i * 2 - 1);
        i = i + 1;
        t *= u;
    }
    res
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_handler() {
        match handler(LambdaEvent::new(Request {
            digits: 10_000
        }, Default::default())).await {
            Ok(r) => {
                print!("{:?}", r.digits);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
