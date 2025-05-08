use regex::Regex;

/// Formats a decimal price string with appropriate decimal places
///
/// # Arguments
///
/// * `price` - The price as a string or f64
/// * `fixed_points` - Number of fixed decimal points (default: 4)
///
/// # Returns
///
/// A formatted string representation of the price
pub fn format_decimal_price<T: ToString>(price: T, fixed_points: Option<usize>) -> String {
    let price_str = price.to_string();
    let fixed = fixed_points.unwrap_or(4);

    // // Find the first non-zero digit after decimal
    // let re = Regex::new(r"^0\.([0]*[1-9])").unwrap();

    // if let Some(captures) = re.captures(&price_str) {
    //     if fixed == 4 {
    //         if let Some(significant_match) = captures.get(1) {
    //             let significant_index = significant_match.as_str().len();
    //             let decimals = significant_index + 3;

    //             // Parse and format with the calculated number of decimal places
    //             if let Ok(num) = price_str.parse::<f64>() {
    //                 return format!("{:.*}", decimals, num);
    //             }
    //         }
    //     }

    //     // For custom fixed points
    //     if let Ok(num) = price_str.parse::<f64>() {
    return format!("{:.*}", fixed, price_str.parse::<f64>().unwrap());
    // }
    // }

    // Return original if no match found or conversion fails
    // price_str
}

/// Formats a long number with K, M, B suffixes
///
/// # Arguments
///
/// * `num` - The number to format
///
/// # Returns
///
/// A formatted string representation of the number
pub fn format_long_number(num: f64) -> String {
    let lookup = [
        (1e9, "B"), // Billion
        (1e6, "M"), // Million
        (1e3, "K"), // Thousand
    ];

    for (value, symbol) in lookup.iter() {
        if num.abs() >= *value {
            // Calculate the value first without rounding
            let divided = num / value;
            // Round to 1 decimal place
            let rounded = (divided * 10.0).floor() / 10.0;

            // Convert to string and remove trailing .0 if present
            let result = format!("{}", rounded);
            if result.ends_with(".0") {
                return format!("{}{}", &result[..result.len() - 2], symbol);
            } else {
                return format!("{}{}", result, symbol);
            }
        }
    }

    // Return the number as string if it's small
    num.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_decimal_price() {
        assert_eq!(format_decimal_price("0.0123", None), "0.0123");
        assert_eq!(format_decimal_price("0.00123", None), "0.00123");
        assert_eq!(format_decimal_price(0.000123, None), "0.000123");
        assert_eq!(format_decimal_price("123.456", None), "123.456");
        assert_eq!(format_decimal_price("0.1", Some(2)), "0.10");
    }

    #[test]
    fn test_format_long_number() {
        assert_eq!(format_long_number(1234.0), "1.2K");
        assert_eq!(format_long_number(1200.0), "1.2K");
        assert_eq!(format_long_number(1000000.0), "1M");
        assert_eq!(format_long_number(1500000.0), "1.5M");
        assert_eq!(format_long_number(1000000000.0), "1B");
        assert_eq!(format_long_number(123.0), "123");
    }
}
