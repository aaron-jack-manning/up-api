# Up API

A convenient and easy to use wrapper for the [Up Bank API](https://developer.up.com.au).

## Example

The following example shows the calculation of the sum of all earnings (transactions with positive value) since a given date:

```
use up_api::v1::Client;
use up_api::v1::transactions::{ListTransactionsOptions, TransactionResource};

fn sum_earnings(transactions : &Vec<TransactionResource>) -> f32 {
    transactions
    .iter()
    .map(|t| &t.attributes.amount.value)
    .map(|v| v.parse::<f32>().unwrap())
    .filter(|a| a > &0.0)
    .sum()
}

#[tokio::main]
async fn main() {
    let token = std::env::var("UP_ACCESS_TOKEN").unwrap();

    let client = Client::new(token.to_string());

    let mut options = ListTransactionsOptions::default();
    options.filter_since("2022-01-01T00:00:00Z".to_string());
    options.page_size(100);

    let mut transactions = client.list_transactions(&options).await.unwrap();

    let mut total = sum_earnings(&transactions.data);

    while let Some(next_page) = transactions.next(&client).await {
        let next_page = next_page.unwrap();
        
        total = total + sum_earnings(&next_page.data);

        transactions = next_page;
    }

    println!("{}", total);
}
```

## Planned Features

- Currently this API wrapper supports all of the `v1` Up API endpoints except [webhooks](https://developer.up.com.au/#webhooks). This is planned for a (hopefully soon) future release.
