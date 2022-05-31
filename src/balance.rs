/// Balance of the client
#[derive(Debug, Default, PartialEq)]
pub struct Balance {
    /// The total funds that are available (`total - held`)
    pub available: f32,

    /// The total funds that are held for dispute (`total - available`)
    pub held: f32,
}

impl Balance {
    /// Returns new balance from transaction amount
    pub fn new(available: f32) -> Self {
        Balance { available, held: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_balance() {
        let balance = Balance::new(7.908);
        assert_eq!(balance, Balance {
            available: 7.908,
            held: 0.0
        });
    }
}
