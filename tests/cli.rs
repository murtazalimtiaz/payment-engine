// TODO: Refactor tests
use assert_cmd::Command;

#[test]
fn should_not_run_without_an_input_file_specified() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd
        .assert()
        .failure()
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::contains("error: The following required arguments were not provided:\n    <file>\n\nUSAGE:\n    payments-engine <file>\n\nFor more information try --help"));
}

#[test]
fn should_not_run_with_unaccessible_file() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/inputs/unaccessible.csv"))
        .assert()
        .failure()
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::contains(
            "✘ CSV Error!\n✘ No such file or directory (os error 2)\n",
        ));
}

#[test]
fn should_be_able_to_process_all_types_of_tx_including_poorly_formatted() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/inputs/transactions_1.csv"))
        .assert()
        .success()
        .stdout(predicates::str::contains("client,available,held,total,locked\n"))
        .stdout(predicates::str::contains("87,4666.3398,0.0000,4666.3398,false\n"))
        .stdout(predicates::str::contains("326,5523.9937,0.0000,5523.9937,true\n"))
        .stderr(predicates::str::is_empty());
}

#[test]
fn should_handle_common_tx_errors() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/inputs/transactions_2.csv"))
        .assert()
        .success()
        .stdout(predicates::str::contains("client,available,held,total,locked\n"))
        .stdout(predicates::str::contains("77,698.1449,0.0000,698.1449,true"))
        .stdout(predicates::str::contains("1,56.0000,0.0000,56.0000,false"))
        .stdout(predicates::str::contains("3,3780.5808,349.4377,4130.0186,false"))
        .stderr(predicates::str::contains("✘ Error processing transaction! Transaction { client_id: 1, id: 2, transaction_data: TransactionData { kind: Chargeback, amount: 0.0 } }\n✘ Program Error!\n✘ The transaction with ID 2 is not disputed! The opposite was expected.\n"))
        .stderr(predicates::str::contains("✘ Error processing transaction! Transaction { client_id: 1, id: 1, transaction_data: TransactionData { kind: Withdrawal, amount: 12.0 } }\n✘ Program Error!\n✘ Newly seen transactions should have a higher ID! Previously seen transaction ID: 1. Current one: 1.\n"))
        .stderr(predicates::str::contains("✘ Error processing transaction! Transaction { client_id: 3, id: 96, transaction_data: TransactionData { kind: Withdrawal, amount: 344.0 } }\n✘ Program Error!\n✘ The client doesn\'t have sufficient funds! Funds available: 34. Transaction amount: 344.\n"))
        .stderr(predicates::str::contains("✘ Error processing transaction! Transaction { client_id: 3, id: 96, transaction_data: TransactionData { kind: Chargeback, amount: 0.0 } }\n✘ Program Error!\n✘ The transaction with ID 96 is not disputed! The opposite was expected.\n"))
        .stderr(predicates::str::contains("✘ Error processing transaction! Transaction { client_id: 3, id: 96, transaction_data: TransactionData { kind: Dispute, amount: 0.0 } }\n✘ Program Error!\n✘ Can\'t retrieve transaction with ID 96! Either it was for a different user or we haven\'t seen it at all.\n"))
        .stderr(predicates::str::contains("✘ Error processing transaction! Transaction { client_id: 77, id: 39, transaction_data: TransactionData { kind: Dispute, amount: 0.0 } }\n✘ Program Error!\n✘ The transaction with ID 39 is disputed! The opposite was expected.\n"))
        .stderr(predicates::str::contains("✘ Error processing transaction! Transaction { client_id: 77, id: 900, transaction_data: TransactionData { kind: Withdrawal, amount: 800.765 } }\n✘ Program Error!\n✘ Transaction to the locked account! Transaction with ID 900 was ignored.\n"));
}

#[test]
fn should_process_normal_txs_swiftly() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/inputs/transactions_3.csv"))
        .assert()
        .success()
        .stdout(predicates::str::contains("client,available,held,total,locked\n"))
        .stdout(predicates::str::contains("2,6690.4302,0.0000,6690.4302,false\n"))
        .stdout(predicates::str::contains("1,60702.5195,752.5600,61455.0781,false\n"))
        .stdout(predicates::str::contains("3,8328.4473,0.0000,8328.4473,true\n"))
        .stderr(predicates::str::is_empty());
}
