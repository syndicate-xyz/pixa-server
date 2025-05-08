pub fn calculate_price_change(current_price: f64, compared_price: f64) -> f64 {
    if current_price == 0.0 || compared_price == 0.0 {
        return 0.0;
    }
    ((current_price - compared_price) / compared_price) * 100.0
}
