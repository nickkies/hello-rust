use code_test::SavingsAccount;

mod utils;

#[test]
fn 잔액은_0으로_시작한다() {
    utils::common_setup();
    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0);
}
