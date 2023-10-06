use isin::ISIN;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Deserialize, Serialize)]
struct Stock {
    isin: ISIN,
    name: String,
}

fn main() {
    let isin: ISIN = "US0378331005".parse().unwrap();
    let order = Stock {
        isin,
        name: "Apple Inc".into(),
    };

    let serialized = serde_json::to_string(&order).unwrap();
    println!("Serialized object: {:?}", serialized);

    let deserialized: Stock = serde_json::from_str(&serialized).unwrap();

    println!("Deserialized object: {:?}", deserialized);
}
