use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountTypeEnum {
    Saver,
    Transactional,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoneyObject {
    /// The ISO 4217 currency code.
    pub currency_code : String,
    /// The amount of money, formatted as a string in the relevant currency. For example, for an Australian dollar value of $10.56, this field will be `"10.56"`. The currency symbol is not included in the string
    pub value : String,
    /// The amount of money in the smallest denomination for the currency, as a 64-bit integer. For example, for an Australian dollar value of $10.56, this field will be `1056`.
    pub value_in_base_units : i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OwnershipTypeEnum {
    Individual,
    Joint,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatusEnum {
    Held,
    Settled,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HoldInfoObject {
    /// The amount of this transaction while in the `HELD` status, in Australian dollars.
    pub amount : MoneyObject,
    /// The foreign currency amount of this transaction while in the `HELD` status. This field will be `null` for domestic transactions. The amount was converted to the AUD amount reflected in the `amount` field.
    pub foreign_amount : Option<MoneyObject>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoundUpObject {
    /// The total amount of this Round Up, including any boosts, represented as a negative value.
    pub amount : MoneyObject,
    /// The portion of the Round Up `amount` owing to boosted Round Ups, represented as a negative value. If no boost was added to the Round Up this field will be `null`.
    pub boost_portion : Option<MoneyObject>,
}

#[derive(Deserialize, Debug)]
pub struct CashBackObject {
    /// A brief description of why this cashback was paid.
    pub description : String,
    /// The total amount of cashback paid, represented as a positive value.
    pub amount : MoneyObject,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPurchaseMethodEnum {
    BarCode,
    OCR,
    CardPin,
    CardDetails,
    CardOnFile,
    #[serde(rename = "ECOMMERCE")]
    ECommerce,
    MagneticStripe,
    Contactless,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardPurchaseMethodObject {
    /// The type of card purchase.
    pub method : CardPurchaseMethodEnum,
    /// The last four digits of the card used for the purchase, if applicable.
    pub card_number_suffix : Option<String>,
}
