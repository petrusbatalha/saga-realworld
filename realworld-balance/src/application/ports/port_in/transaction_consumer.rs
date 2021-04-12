pub trait TransactionReceiver {
    fn consume_transaction() -> Transaction;
}