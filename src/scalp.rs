use chrono::Utc;
use fang::FangError;
use futures::future::join_all;
use std::time::{Duration, Instant};

use crate::account::{KideAccount,fetch_kide_accounts,AccountIDList};
use crate::request::Client;
use crate::sale::SaleClient;
use crate::strategy::Count;

async fn reserve_in_succession(
    sale_client: SaleClient,
    account: KideAccount,
    count: i64,
) -> Result<(), FangError> {
    for i in 1..count + 1 {
        let _ = sale_client
            .reserve_all(account.token.clone(), &Count { count: i })
            .await;
        log::debug!(
            "Reserved ticket {} of {} for account {}",
            i,
            count,
            account.name
        );
    }

    Ok(())
}

pub async fn scalp(event_id: String, account_ids: AccountIDList) -> Result<(), FangError> {
    // Fetch the accounts from the database
    log::debug!("Fetching accounts...");
    let accounts = fetch_kide_accounts(account_ids).await?;

    // Initialize the connection to the kide api
    log::debug!("Initializing client...");
    let client = Client::new();
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
    log::trace!("Using following info: {:?}", sale_client.sale);
    let measurement_begin = Instant::now();

    let reserve_jobs = accounts
        .into_iter()
        .map(|account| reserve_in_succession(sale_client.clone(), account, 20));
    join_all(reserve_jobs).await;

    let execution_time = measurement_begin.elapsed().as_millis();
    log::debug!("Execution took {}ms", execution_time);
    log::info!("Done");

    Ok(())
}
