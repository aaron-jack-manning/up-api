# Up API

A convenient and easy to use wrapper for the [UP Bank API](https://developer.up.com.au).

## Example

The following example shows the calculation of the sum of all transactions after a given date.

```
use up_api::Client;
use up_api::transactions::ListTransactionsOptions;

#[tokio::main]
async fn main() {
    let token = std::env::var("UP_ACCESS_TOKEN").unwrap();
    let client = Client::new(token.to_string());

    let mut options = ListTransactionsOptions::default();
    options.filter_since("2020-01-01T01:02:03+10:00".to_string());

    let transactions = client.list_transactions().unwrap();

    let total : f32 =
        transactions
        .data
        .into_iter()
        .map(|t| t.attributes.amount.value)
        .map(|v| v.parse::<f32>())
        .sum();

    println!("{}", total);
}
```
