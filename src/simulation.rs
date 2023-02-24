use std::collections::HashMap;

use chrono::{Days, NaiveDate};
use currency::Currency;

use crate::{
    account::{AccountID, AccountSpec},
    transaction::Transaction,
    util::DateInterval,
};

// TODO: Can we do something equivalent to a 'mutable singleton' in Rust?
/// The simulation which produces predicted values for accounts over time, based on loaded
/// specifications for accounts and transactions.
#[derive(Clone, Default, Debug)]
pub struct Simulation {
    /// All accounts which may be used as a sink or source for transactions.
    pub accounts: HashMap<AccountID, AccountSpec>,

    /// All specified transactions which may take place.
    pub transactions: Vec<Transaction>,

    /// The start date for the simulation.
    pub start: NaiveDate,
}

impl Simulation {
    /// Load values for accounts and transactions to be used when running the simulation.
    pub fn load(&mut self) {
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

        self.transactions = vec![
            Transaction::single(
                &self,
                Currency::from_str("£500.00").unwrap(),
                AccountID { id_val: 0 },
                AccountID { id_val: 1 },
                NaiveDate::from_ymd_opt(2023, 02, 25).unwrap(),
            )
            .expect(""),
            Transaction::repeating(
                &self,
                Currency::from_str("£1500").unwrap(),
                AccountID { id_val: 2 },
                AccountID { id_val: 0 },
                NaiveDate::from_ymd_opt(2023, 02, 24).unwrap(),
                DateInterval::Monthly,
            )
            .expect(""),
        ];

        self.start = NaiveDate::from_ymd_opt(2023, 02, 23).unwrap();
    }

    pub fn iter(self) -> SimulationIterator {
        // TODO: Make `sim` a reference so we don't have to do so much cloning.
        let clone = self.clone();
        SimulationIterator {
            sim: clone,
            values: self
                .accounts
                .iter()
                .map(|kv| (kv.0.clone(), kv.1.initial_value.clone()))
                .collect(),
            date: self.start,
        }
    }
}

/// An iterator type which provides values of accounts over a forward progression of time.
#[derive(Clone, Debug)]
pub struct SimulationIterator {
    /// Data relating to the original state of the simulation.
    pub sim: Simulation,

    /// The current values of all the accounts on this iteration of the simulation.
    pub values: HashMap<AccountID, Currency>,

    /// The current date on this iteration of the simulation.
    pub date: NaiveDate,
}

impl IntoIterator for Simulation {
    type Item = (HashMap<AccountID, Currency>, NaiveDate);
    type IntoIter = SimulationIterator;

    fn into_iter(self) -> Self::IntoIter {
        // TODO: Make `sim` a reference so we don't have to do so much cloning.
        let accounts = self.accounts.clone();
        let date = self.start;
        SimulationIterator {
            sim: self,
            values: accounts
                .into_iter()
                .map(|kv| (kv.0, kv.1.initial_value))
                .collect(),
            date,
        }
    }
}

impl Iterator for SimulationIterator {
    type Item = (HashMap<AccountID, Currency>, NaiveDate);

    fn next(&mut self) -> Option<Self::Item> {
        // Iterate through the relevant transactions, IE those which occur on the current date.
        for t in self.sim.transactions.iter().filter(|t| t.occurs(self.date)) {
            // TODO: Error handling if account ID doesn't exist.

            // Update source and sink account values according to their specification on how to
            // handle money being transferred out and in respectively.
            let source_spec = self.sim.accounts.get(&t.source).unwrap();
            let source_val = self.values.get_mut(&t.source).unwrap();
            *source_val = source_spec.source(source_val, &t.value);

            let sink_spec = self.sim.accounts.get(&t.sink).unwrap();
            let sink_val = self.values.get_mut(&t.sink).unwrap();
            *sink_val = sink_spec.sink(sink_val, &t.value);
        }

        // Iterate through all accounts and allow them to apply whatever update is necessary to
        // their values over the course of a single day.
        for (a, val) in self.values.iter_mut() {
            let spec = self.sim.accounts.get(&a).unwrap();
            *val = spec.update(val);
        }

        // Advance the date by one day.
        self.date = self.date + Days::new(1);

        Some((self.values.clone(), self.date))
    }
}
