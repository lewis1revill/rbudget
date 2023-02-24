use crate::account::AccountID;
use crate::simulation::Simulation;
use crate::util::DateInterval;
use chrono::naive::NaiveDate as Date;
use chrono::Datelike;
use currency::Currency;

/// Representation of a transaction of some value from a source account to a sink account. Occurs
/// on at least one date.
#[derive(Eq, PartialEq, Clone, Debug, Default, Hash)]
pub struct Transaction {
    /// The value transferred by the transaction.
    pub value: Currency,

    /// The ID of an account which is the source of the transaction, from which the value is taken.
    pub source: AccountID,

    /// The ID of an account which is the sink of the transaction, to which the value is
    /// transferred.
    pub sink: AccountID,

    /// The date of the first occurrence of this transaction. If no repetition is specified it will
    /// be the only occurrence of the transaction.
    start: Date,

    /// How the transaction repeats, if at all. Will be used in conjunction with `start` (and
    /// optionally `end` to determine dates on which the transaction occurs.
    rpt: Option<DateInterval>,

    /// The end date of the transaction. Does not necessarily have to be one of the potential dates
    /// of occurrence but must not be before `start`. No transactions occur after this date.
    end: Option<Date>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum TransactionError {
    /// Transaction was not constructed because an error occurred obtaining an `Account` from an
    /// `AccountID`.
    InvalidAccountID { id: AccountID },

    /// Transaction was not constructed because the source and sink account IDs were equivalent.
    DuplicateAccountID { duplicate_id: AccountID },

    /// Transaction was not constructed because the end date was before the start date.
    InvalidStartEndDateCombination { start: Date, end: Date },
}

impl Transaction {
    /// Creates a transaction which occurs on a single date.
    pub fn single(
        sim: &Simulation,
        value: Currency,
        source: AccountID,
        sink: AccountID,
        date: Date,
    ) -> Result<Transaction, TransactionError> {
        // Ensure we have valid accounts.
        if !sim.accounts.contains_key(&source) {
            return Err(TransactionError::InvalidAccountID { id: source });
        }
        if !sim.accounts.contains_key(&sink) {
            return Err(TransactionError::InvalidAccountID { id: sink });
        }

        // Ensure accounts don't match.
        if source == sink {
            return Err(TransactionError::DuplicateAccountID {
                duplicate_id: source,
            });
        }

        // Ok to create the transaction now with no repetition or end date.
        Ok(Transaction {
            value,
            source,
            sink,
            start: date,
            rpt: None,
            end: None,
        })
    }

    /// Create a transaction which occurs first on a given start date and repeats endlessly.
    pub fn repeating(
        sim: &Simulation,
        value: Currency,
        source: AccountID,
        sink: AccountID,
        start: Date,
        rpt: DateInterval,
    ) -> Result<Transaction, TransactionError> {
        // Construct a transaction with a single occurrence to build from.
        let mut t = Transaction::single(sim, value, source, sink, start)?;

        // Set the repetition.
        t.rpt = Some(rpt);

        Ok(t)
    }

    /// Create a transaction which occurs first on a given start date and repeats until an end
    /// date.
    pub fn repeating_until(
        sim: &Simulation,
        value: Currency,
        source: AccountID,
        sink: AccountID,
        start: Date,
        rpt: DateInterval,
        end: Date,
    ) -> Result<Transaction, TransactionError> {
        // Construct an endlessly repeating transaction to build from.
        let mut t = Transaction::repeating(sim, value, source, sink, start, rpt)?;

        // Ensure we don't have an invalid start and end date combination.
        if end < start {
            return Err(TransactionError::InvalidStartEndDateCombination { start, end });
        }

        // Ok to set the end date.
        t.end = Some(end);

        Ok(t)
    }

    /// Determine whether this transaction occurs on a specific date. Returns true if the
    /// transaction takes place.
    pub fn occurs(self: &Transaction, date: Date) -> bool {
        // Transactions cannot occur before their start date.
        //
        // If the transaction repeats, then we need to determine if the date is one of
        // those dates on which it repeats according to the date interval. If the transaction does
        // not repeat, then it only occurs on the start date.
        //
        // If we have an end date, the transaction cannot occur after the end date.
        date >= self.start
            && match self.rpt {
                Some(DateInterval::Daily) => true,
                Some(DateInterval::Weekly) => date.weekday() == self.start.weekday(),
                Some(DateInterval::Monthly) => date.day() == self.start.day(),
                Some(DateInterval::Yearly) => date.ordinal() == self.start.ordinal(),
                None => date == self.start,
            }
            && self.end.map_or(true, |e| date < e)
    }
}
