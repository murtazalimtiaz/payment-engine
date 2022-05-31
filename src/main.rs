use payments_engine::{get_args, print_client_account_balances, process_transactions};

fn main() {
    if let Err(err) = get_args()
        .and_then(process_transactions)
        .and_then(print_client_account_balances)
    {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
