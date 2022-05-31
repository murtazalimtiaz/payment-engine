use hashbrown::HashMap;
use tinyset::SetU32;

use crate::{
    balance::Balance,
    transaction::{TransactionData, TransactionKind, Transactions},
    types::Result,
};

///  Unique client
#[derive(Debug, Default, PartialEq)]
pub struct Client {
    /// Balance of the client
    balance: Balance,

    /// Hash map of client's transactions
    transactions: Transactions,

    /// List of disputed transactions IDs
    disputed_transactions: SetU32,

    /// Previous transaction ID
    previous_tx_id: u32,

    /// A flag indicating whether the account is locked (chargeback occurred)
    locked: bool,
}

impl Client {
    /// Returns new initialized client
    pub fn new(tx_id: u32, tx_data: TransactionData) -> Self {
        let balance = if tx_data.kind == TransactionKind::Deposit {
            Balance::new(tx_data.amount)
        } else {
            Balance::default()
        };

        let mut client = Client {
            balance,
            transactions: Transactions::new(),
            disputed_transactions: SetU32::new(),
            previous_tx_id: tx_id,
            locked: false,
        };

        client.record_tx(tx_id, tx_data);
        client
    }

    /// Records transaction for the client
    #[inline]
    fn record_tx(&mut self, tx_id: u32, tx_data: TransactionData) {
        self.transactions.insert(tx_id, tx_data);
        self.previous_tx_id = tx_id;
    }

    /// Retrieves transaction by its ID
    ///
    /// This doesn't work for `Dispute`, `Resolve` & `Chargeback` transactions
    /// as we deliberately don't store them (they don't have _own_ tx IDs)
    #[inline]
    fn get_tx(&self, tx_id: u32) -> Result<&TransactionData> {
        match self.transactions.get(&tx_id) {
            Some(tx) => Ok(tx),
            None => Err(format!(
                "Can't retrieve transaction with ID {tx_id}! \
                Either it was for a different user or we haven't seen it at all."
            )
            .into()),
        }
    }

    /// Checks that transaction have a higher ID than previously recorded
    ///
    /// Transaction is suspicious if its ID is lower than the ones we previously seen,
    /// surprisingly enough `Dispute`, `Resolve` & `Chargeback` don't have their _own_ tx IDs
    #[inline]
    fn check_tx_id(&self, tx_id: u32) -> Result<()> {
        if self.previous_tx_id < tx_id {
            Ok(())
        } else {
            Err(format!(
                "Newly seen transactions should have a higher ID! \
                Previously seen transaction ID: {}. \
                Current one: {tx_id}.",
                self.previous_tx_id
            )
            .into())
        }
    }

    /// Checks if clients available balance is sufficient to execute transaction
    #[inline]
    fn amount_is_available(&self, tx_amount: f32) -> Result<()> {
        if self.balance.available >= tx_amount {
            Ok(())
        } else {
            return Err(format!(
                "The client doesn't have sufficient funds! \
                Funds available: {}. \
                Transaction amount: {tx_amount}.",
                self.balance.available
            )
            .into());
        }
    }

    /// Checks if transaction is disputed
    ///
    /// Returns error if the result is not the same as `expected`
    #[inline]
    fn transaction_is_disputed(&self, tx_id: u32, expected: bool) -> Result<()> {
        if self.disputed_transactions.contains(tx_id) != expected {
            Err(format!(
                "The transaction with ID {tx_id} is{}disputed! The opposite was expected.",
                if expected { " not " } else { " " }
            )
            .into())
        } else {
            Ok(())
        }
    }

    /// Checks if account is locked
    #[inline]
    fn account_is_locked(&self, tx_id: u32) -> Result<()> {
        if self.locked {
            Err(format!("Transaction to the locked account! Transaction with ID {tx_id} was ignored.").into())
        } else {
            Ok(())
        }
    }

    /// Processes transaction for the client
    pub fn process_tx(&mut self, tx_id: u32, tx_data: TransactionData) -> Result<()> {
        self.account_is_locked(tx_id)?;

        match tx_data.kind {
            TransactionKind::Deposit => {
                self.check_tx_id(tx_id)?;
                self.balance.available += tx_data.amount;
                self.record_tx(tx_id, tx_data);
            },
            TransactionKind::Withdrawal => {
                self.check_tx_id(tx_id)?;
                self.amount_is_available(tx_data.amount)?;
                self.balance.available -= tx_data.amount;
                self.record_tx(tx_id, tx_data);
            },
            TransactionKind::Dispute => {
                self.transaction_is_disputed(tx_id, false)?;
                let disputed = self.get_tx(tx_id)?.amount;

                // this is not strictly necessary, but nice to have
                self.amount_is_available(disputed)?;

                self.balance.available -= disputed;
                self.balance.held += disputed;
                self.disputed_transactions.insert(tx_id);
            },
            TransactionKind::Resolve => {
                self.transaction_is_disputed(tx_id, true)?;
                let disputed = self.get_tx(tx_id)?.amount;
                if self.balance.held >= disputed {
                    self.balance.held -= disputed;
                    self.balance.available += disputed;
                    self.disputed_transactions.remove(tx_id);
                }
            },
            TransactionKind::Chargeback => {
                self.transaction_is_disputed(tx_id, true)?;
                let disputed = self.get_tx(tx_id)?.amount;

                // this is not strictly necessary, but nice to have
                if self.balance.held >= disputed {
                    self.balance.held -= disputed;
                    self.locked = true;
                    self.disputed_transactions.remove(tx_id);
                }
            },
        }

        Ok(())
    }

    /// Returns properly formatted user record
    // TODO: Optimize slow formatting routines
    pub fn get_record(&self, client_id: u16) -> Vec<String> {
        vec![
            client_id.to_string(),
            format!("{:.4}", self.balance.available),
            format!("{:.4}", self.balance.held),
            format!("{:.4}", self.balance.available + self.balance.held),
            self.locked.to_string(),
        ]
    }
}

/// App's map of clients with client ID as a key
pub type Clients = HashMap<u16, Client>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_client_from_deposit() {
        let tx_data = TransactionData {
            kind: TransactionKind::Deposit,
            amount: 56.8953,
        };
        let client = Client::new(78, tx_data.clone());
        let mut expected_client = Client {
            balance: Balance {
                available: 56.8953,
                held: 0.0,
            },
            transactions: Transactions::new(),
            disputed_transactions: SetU32::new(),
            previous_tx_id: 78,
            locked: false,
        };
        expected_client.record_tx(78, tx_data);
        assert_eq!(client, expected_client);
    }

    #[test]
    fn should_create_new_client_from_arbitrary_tx() {
        let tx_data = TransactionData {
            kind: TransactionKind::Dispute,
            amount: 657.5675,
        };
        let client = Client::new(3438, tx_data.clone());
        let mut expected_client = Client {
            balance: Balance {
                available: 0.0,
                held: 0.0,
            },
            transactions: Transactions::new(),
            disputed_transactions: SetU32::new(),
            previous_tx_id: 0,
            locked: false,
        };
        expected_client.record_tx(3438, tx_data);
        assert_eq!(client, expected_client);
    }
}
