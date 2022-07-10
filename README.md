# Up API

A convenient and easy to use wrapper for the [Up Bank API](https://developer.up.com.au).

## Example

The following example shows the calculation of the sum of all transactions after a given date (up to the page limit).

```
use up_api::v1::Client;
use up_api::v1::transactions::ListTransactionsOptions;

#[tokio::main]
async fn main() {
    let token = std::env::var("UP_ACCESS_TOKEN").unwrap();

    let client = Client::new(token.to_string());

    let mut options = ListTransactionsOptions::default();
    options.filter_since("2020-01-01T01:02:03Z".to_string());
    options.page_size(100);

    let transactions = client.list_transactions(&options).await.unwrap();

    let total : f32 =
        transactions
        .data
        .into_iter()
        .map(|t| t.attributes.amount.value)
        .map(|v| v.parse::<f32>().unwrap())
        .filter(|a| a > &0.0)
        .sum();

    println!("{}", total);
}
```

## Planned Features

Currently this API wrapper supports all of the `v1` Up API endpoints except [webhooks](https://developer.up.com.au/#webhooks). This is planned for a (hopefully soon) future release.
