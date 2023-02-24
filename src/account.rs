use chrono::naive::NaiveDate as Date;
use currency::Currency;

use crate::util::{to_currency, to_f64};

/// A simple unique ID for a specific account, simply used to identify which account we are looking
/// at.
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default, Hash)]
pub struct AccountID {
    pub id_val: u64,
}

/// A data structure representing the specified detals of an individual account. These details can
/// be used to determine how transactions on specific accounts affect the account's value and how it
/// changes over time.
#[derive(PartialEq, Clone, Debug)]
pub struct AccountSpec {
    /// The name of the account.
    pub name: String,

    /// The initial total account value.
    pub initial_value: Currency,

    // TODO: More flexible ways of expressing how account value changes from day to day.
    /// The effective interest rate per day on this account as a fraction of total account value.
    pub interest: f64,

    // TODO: Express exactly how transactions affect the account's value.
    /// The effective charge on transactions made from this account as a fraction of the
    /// transaction value.
    pub out_charge: f64,
    /// The effective charge on transactions made to this account as a fraction of the transaction
    /// value.
    pub in_charge: f64,
}

impl AccountSpec {
    /// Calculate the total value of this account after using it as a source for a transaction.
    pub fn source(&self, value: &Currency, out: &Currency) -> Currency {
        value - to_currency(to_f64(out) * (1.0 + self.out_charge))
    }

    /// Calculate the total value of this account after using it as a sink for a transaction.
    pub fn sink(&self, value: &Currency, in_: &Currency) -> Currency {
        value + to_currency(to_f64(in_) * (1.0 - self.in_charge))
    }

    /// Calculate the total value of this account after a single day has passed.
    pub fn update(&self, value: &Currency) -> Currency {
        to_currency(to_f64(value) * (1.0 + (self.interest / 365.0)))
    }
}

/// A simple data structure representing the current state of an account's value on a given date.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct AccountState {
    /// The total account value.
    pub value: Currency,

    /// The date for which the account has this state.
    pub date: Date,
}
