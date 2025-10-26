use regex::Regex;

pub fn extract_unit(address: &str) -> Result<String, String> {
    let re = Regex::new(r#"(#[A-Za-z0-9]+[-[A-Za-z0-9/]+]+)"#).unwrap();
    match re.captures(address) {
        Some(caps) => Ok(caps.get(1).unwrap().as_str().to_string()),
        None => Err("No valid unit found".into()),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::extract_unit;
    use rstest::rstest;

    #[rstest]
    #[case("1234 Main St #123-01", "#123-01")]
    #[case("23 SERANGOON CENTRAL #B2-49 NEX 556083", "#B2-49")]
    #[case(
        "11 BEDOK NORTH STREET 1 #01-28/29 HEARTBEAT @ BEDOK, Stall 5 469662",
        "#01-28/29"
    )]
    // #[case("23 UPPER DICKSON ROAD 01-01 207482", "eturn err )]
    // #[case("101 THOMSON ROAD #01-14 / 15 / 16 UNITED SQUARE 307591", "eturn err )]
    #[case("421C NORTHSHORE DRIVE #01-01, Stall 9 823421", "#01-01")]
    #[case(
        "11 JALAN TAN TOCK SENG #01-13/14/15 TAN TOCK SENG HOSPITAL 308433",
        "#01-13/14/15"
    )]
    #[case(
        "80 AIRPORT BOULEVARD #M021-53 CHANGI AIRPORT T1 Departure/Transit Lounge West 819642",
        "#M021-53"
    )]
    #[case(
        "33 SENGKANG WEST AVENUE #01-09-14 THE SELETAR MALL 797653",
        "#01-09-14"
    )]
    fn test_extract_unit(#[case] address: &str, #[case] expected: &str) {
        assert_eq!(extract_unit(address).unwrap(), expected.to_string());
    }
}
