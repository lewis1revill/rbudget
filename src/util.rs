use currency::Currency;
use num::ToPrimitive;

/// An enum defining different intervals between dates for use when defining repeating events.
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub enum DateInterval {
    /// Event occurs once every day.
    Daily,
    /// Event occurs once every week.
    Weekly,
    /// Event occurs once every month.
    Monthly,
    /// Event occurs once every year.
    Yearly,

    // TODO: Custom interval.
}

/// Create a floating point value representing a currency value so that we can do higher precision
/// mathematical operations before converting back to a currency value.
pub fn to_f64(val: &Currency) -> f64 {
    val.value().to_f64().unwrap_or(0.0) / 100.0
}

/// Convert a floating point value to a currency value, rounding off to the precision of two
/// decimal places.
pub fn to_currency(val: f64) -> Currency {
    match Currency::from_str(&format!("£{:.2}", val).to_string()) {
        Ok(v) => v,
        Err(_) => Currency::new(),
    }
}
