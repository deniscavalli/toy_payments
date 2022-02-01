use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
extern crate csv as ECSV;

use crate::csv::{reader, writer};
use processors::txprocessor;
use structs::clients::ClientAccount;
use structs::transaction::{Transaction, TransactionRecord};

use std::thread;

mod csv;
mod processors;
mod structs;

fn main() {
    let clients: HashMap<u16, ClientAccount> = HashMap::new();
    let clients_ledger = Arc::new(Mutex::new(clients));

    let transactions: HashMap<u32, TransactionRecord> = HashMap::new();
    let transactions_ledger = Arc::new(Mutex::new(transactions));

    let start_write = Arc::new(AtomicBool::new(false));
    let start_writer = Arc::clone(&start_write);

    let (tx_transactions, rx_transactions): (Sender<Transaction>, Receiver<Transaction>) =
        mpsc::channel();
    let (tx_transactions2, rx_transactions2): (Sender<Transaction>, Receiver<Transaction>) =
        mpsc::channel();

    let tx_clone1 = tx_transactions.clone();
    let handle1 = thread::spawn(|| reader::read(tx_clone1).unwrap());

    let tl_clone1 = Arc::clone(&transactions_ledger);
    let tx_clone2 = tx_transactions2.clone();
    let handle2 = thread::spawn(|| {
        txprocessor::store_transactions(rx_transactions, tx_clone2, tl_clone1).unwrap()
    });

    let tl_clone2 = Arc::clone(&transactions_ledger);
    let cl_clone = Arc::clone(&clients_ledger);
    let handle3 = thread::spawn(|| {
        txprocessor::process_transactions(rx_transactions2, tl_clone2, cl_clone, start_write)
            .unwrap()
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    let handle4 = thread::spawn(|| writer::write(clients_ledger, start_writer).unwrap());
    handle4.join().unwrap();
}
