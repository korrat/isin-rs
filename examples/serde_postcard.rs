use isin::ISIN;

fn main() {
    let mut buffer = [0; 30];
    let isin: ISIN = "US0378331005".parse().unwrap();

    let serialized = postcard::to_slice(&isin, &mut buffer).unwrap();
    println!(
        "Serialized ISIN: {} ({:?})",
        hex::encode(&serialized)
       , bstr::BStr::new(serialized
    ));

    let deserialized: ISIN = postcard::from_bytes(&serialized).unwrap();

    println!("Deserialized ISIN: {}", deserialized.as_ref()); // "US0378331005"
    println!("  Prefix: {}", deserialized.prefix()); // "US"
    println!("  Basic code: {}", deserialized.basic_code()); // "037833100"
    println!("  Check digit: {}", deserialized.check_digit()); // '5'
    assert_eq!(isin, deserialized);
}
