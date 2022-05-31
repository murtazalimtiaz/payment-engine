lazy_static! {
    /// Definition of `Deposit` & `Withdrawal` input header
    pub static ref STANDARD_INPUT_HEADER: csv::ByteRecord = csv::ByteRecord::from(
        vec!["type", "client", "tx", "amount"]
    );

    /// Definition of `Dispute`, `Resolve` and `Chargeback` header
    pub static ref MINIMAL_INPUT_HEADER: csv::ByteRecord = csv::ByteRecord::from(
        vec!["type", "client", "tx"]
    );
}

/// Definition of clients output header
pub const OUTPUT_HEADER: &[&str; 5] = &["client", "available", "held", "total", "locked"];
