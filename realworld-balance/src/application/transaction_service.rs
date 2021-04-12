use anyhow::{bail, Result};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TransactionService<U, T> {
    pub receiver: U,
    pub persist: T,
}

fn receiver() {
    let (tx, rx) = flume::unbounded();
}
