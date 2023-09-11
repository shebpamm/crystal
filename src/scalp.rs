use chrono::Utc;

use crate::request::Client;
use crate::strategy::Count;
use fang::FangError;
use std::time::{Duration,Instant};

pub async fn scalp(
    event_id: String,
    account_token: String,
) -> Result<(), FangError> {
    let client = Client::new(account_token);
    let mut sale_client = client.product(event_id.clone()).await.unwrap();

    // Block until the sale starts.
    // If there's over 2 seconds left until the sale starts, sleep for 1 second and
    // recheck.
    // If there's less than 2 seconds left until the sale starts, sleep for only 0.1 seconds.
    if sale_client.sale.variants.len() == 0 {
        log::debug!("Waiting for sale to start...");
        loop {
            let now = Utc::now();
            let diff = sale_client.sale.product.date_sales_from - now;
            log::debug!("{} seconds until sale starts", diff.num_seconds());
            if diff.num_seconds() > 2 {
                tokio::time::sleep(Duration::from_millis(1000)).await;
            } else {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }

            sale_client = client.product(event_id.clone()).await.unwrap();
            if sale_client.sale.variants.len() > 0 {
                break;
            }
        }
    }


    // Begin reserving tickets
    log::info!("Reserving all variants...");
    let measurement_begin = Instant::now();
    for i in 1..21 {
        let _ = sale_client.reserve_all(&Count { count: i }).await;
    }
    let execution_time = measurement_begin.elapsed().as_millis();
    log::debug!("Execution took {}ms", execution_time);
    log::info!("Done");

    Ok(())
}
