use std::collections::HashMap;

use currency::Currency;

use crate::{
    account::{AccountID, AccountSpec},
    transaction::Transaction,
};

// TODO: Can we do something equivalent to a 'mutable singleton' in Rust?
#[derive(Default, Debug)]
pub struct BudgetData {
    /// All accounts which may be used as a sink or source for transactions.
    pub accounts: HashMap<AccountID, AccountSpec>,

    /// All specified transactions which may take place.
    pub transactions: Vec<Transaction>,
}

impl BudgetData {
    pub fn load(self: &mut BudgetData) {
        // TODO: Read accounts and transactions from a file. For now just specify some defaults.
        self.accounts.insert(
            AccountID { id_val: 0 },
            AccountSpec {
                name: "Bank".to_string(),
                initial_value: Currency::from_str("£1000.00").unwrap(),
                interest: 0.0,
                out_charge: 0.0,
                in_charge: 0.0,
            },
        );
        self.accounts.insert(
            AccountID { id_val: 1 },
            AccountSpec {
                name: "Savings".to_string(),
                initial_value: Currency::from_str("£500.00").unwrap(),
                interest: 0.03,
                out_charge: 0.0,
                in_charge: 0.0,
            },
        );
        self.accounts.insert(
            AccountID { id_val: 2 },
            AccountSpec {
                name: "Employer".to_string(),
                initial_value: Currency::from_str("£0.00").unwrap(),
                interest: 0.0,
                out_charge: -1.0,
                in_charge: 0.0,
            },
        );
        self.accounts.insert(
            AccountID { id_val: 3 },
            AccountSpec {
                name: "Costs".to_string(),
                initial_value: Currency::from_str("£0.00").unwrap(),
                interest: 0.0,
                out_charge: 0.0,
                in_charge: 1.0,
            },
        );
    }
}
