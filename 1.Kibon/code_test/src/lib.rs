/// A savings account
pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    /// Creates a `SavingsAccount` with a balance of 0
    ///
    /// # Examples
    ///
    /// ```
    /// use code_test::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    pub fn new() -> Self {
        Self { balance: 0 }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Deposit amount cannot be negative");
        }
        self.balance += amount;
    }

    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transfered ${amount} to {acc_number}"))
    }
}

fn some_function() {
    println!("private function");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 잔액은_0으로_시작한다() {
        some_function();
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn 계좌에_입금한다() {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100, "잔액은 100!");
        assert_ne!(account.get_balance(), 0);
        assert!(account.get_balance() == 100);
    }

    #[test]
    #[should_panic]
    fn 잔액이_음수인_경우_패닉된다() {
        let mut account = SavingsAccount::new();
        account.deposit(-1);
    }

    #[test]
    fn 송금한다() -> Result<(), String> {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        account.transfer(123, 100)?;
        Ok(())
    }
}
