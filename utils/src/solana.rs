use bs58;

pub fn base58_to_wallet(base58: &str) -> Vec<u8> {
    // Convert base58 string to wallet byte array
    bs58::decode(base58).into_vec().unwrap()
}

pub fn wallet_to_base58(wallet: &[u8]) -> String {
    // Convert wallet byte array to base58 string
    bs58::encode(wallet).into_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base58_to_wallet() {
        // Test with a known value
        let base58 = "61kmUkBqx9vjDBzqpaBEU8FHDmkNNBj97S3d5GxaCGYb";
        let expected = vec![
            127, 163, 169, 93, 54, 236, 17, 94, 31, 93, 192, 83, 206, 151, 60, 192, 59, 22, 183,
            254, 55, 124, 142, 242, 149, 54, 106, 62, 44, 39, 239, 127, 74, 123, 236, 7, 93, 179,
            89, 3, 97, 82, 86, 192, 78, 54, 14, 91, 219, 112, 62, 197, 197, 191, 57, 242, 109, 138,
            51, 217, 223, 27, 148, 44,
        ];
        assert_eq!(base58_to_wallet(base58), expected);
    }

    #[test]
    fn test_wallet_to_base58() {
        // Test with a known value
        let wallet = vec![
            127, 163, 169, 93, 54, 236, 17, 94, 31, 93, 192, 83, 206, 151, 60, 192, 59, 22, 183,
            254, 55, 124, 142, 242, 149, 54, 106, 62, 44, 39, 239, 127, 74, 123, 236, 7, 93, 179,
            89, 3, 97, 82, 86, 192, 78, 54, 14, 91, 219, 112, 62, 197, 197, 191, 57, 242, 109, 138,
            51, 217, 223, 27, 148, 44,
        ];
        let expected = "61kmUkBqx9vjDBzqpaBEU8FHDmkNNBj97S3d5GxaCGYb";
        assert_eq!(wallet_to_base58(&wallet), expected);
    }

    #[test]
    fn test_roundtrip_base58_wallet() {
        // Test that encoding and then decoding returns the original value
        let original_base58 = "61kmUkBqx9vjDBzqpaBEU8FHDmkNNBj97S3d5GxaCGYb";
        let wallet = base58_to_wallet(original_base58);
        let roundtrip_base58 = wallet_to_base58(&wallet);
        assert_eq!(original_base58, roundtrip_base58);
    }

    #[test]
    fn test_roundtrip_wallet_base58() {
        // Test that decoding and then encoding returns the original value
        let original_wallet = vec![
            127, 163, 169, 93, 54, 236, 17, 94, 31, 93, 192, 83, 206, 151, 60, 192, 59, 22, 183,
            254, 55, 124, 142, 242, 149, 54, 106, 62, 44, 39, 239, 127, 74, 123, 236, 7, 93, 179,
            89, 3, 97, 82, 86, 192, 78, 54, 14, 91, 219, 112, 62, 197, 197, 191, 57, 242, 109, 138,
            51, 217, 223, 27, 148, 44,
        ];
        let base58 = wallet_to_base58(&original_wallet);
        println!("base58: {}", base58);
        let roundtrip_wallet = base58_to_wallet(&base58);
        assert_eq!(original_wallet, roundtrip_wallet);
    }

    #[test]
    fn test_empty_input() {
        // Test with empty inputs
        let empty_base58 = "";
        let empty_wallet: Vec<u8> = Vec::new();

        // An empty base58 string should decode to an empty wallet
        assert_eq!(base58_to_wallet(empty_base58), empty_wallet);

        // An empty wallet should encode to an empty base58 string
        assert_eq!(wallet_to_base58(&empty_wallet), empty_base58);
    }
}
