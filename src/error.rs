quick_error! {
    /// App's error enum
    #[derive(Debug)]
    pub enum Error {
        Custom(err: String) {
            from()
            from(err: &str) -> (err.into())
            display("✘ Program Error!\n✘ {err}")
        }
        Io(err: std::io::Error) {
            from()
            display("✘ I/O Error!\n✘ {err}")
        }
        Csv(err: csv::Error) {
            from()
            display("✘ CSV Error!\n✘ {err}")
        }
    }
}
