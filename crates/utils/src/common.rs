use regex::Regex;
use reqwest::Error;
use std::sync::OnceLock;

static ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

pub async fn get_url_content(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    response.text().await
}

/// Checks if the given string is a valid Ethereum address and it is not the zero address.
///
/// # Arguments
///
/// * `address` - A string slice that holds the Ethereum address.
///
/// # Returns
///
/// * `true` if the address is a valid Ethereum address and it is not the zero address.
/// * `false` otherwise.
pub fn is_valid_ethereum_address(address: &str) -> bool {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX
        .get_or_init(|| Regex::new(r"(?i)^0x[0-9a-f]{40}$").expect("Fail to compile regex"))
        .is_match(address)
        && address != ZERO_ADDRESS
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_is_valid_ethereum_address() {
        let tests = vec![
            (
                "valid address",
                "0x1234567890abcdef1234567890abcdef12345678",
                true,
            ),
            (
                "uppercase",
                "0x1234567890ABCDEF1234567890ABCDEF12345678",
                true,
            ),
            (
                "too short",
                "0x1234567890abcdef1234567890abcdef123456",
                false,
            ),
            (
                "missing 0x prefix",
                "001234567890abcdef1234567890abcdef12345678",
                false,
            ),
            (
                "non-hex characters",
                "0x1234567890abcdef1234567890abcdef123ÅÅÅÅÅ",
                false,
            ),
        ];

        for (name, address, expected) in tests {
            let result = super::is_valid_ethereum_address(address);
            assert_eq!(expected, result, "{}", name);
        }
    }
}
