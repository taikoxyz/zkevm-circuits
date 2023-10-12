#![no_main]

use halo2_proofs::halo2curves::bn256::Fr;
use libfuzzer_sys::fuzz_target;
use rand::SeedableRng;

use eth_types::geth_types::Transaction;
use mock::MockTransaction;
use zkevm_circuits::tx_circuit::test::run;

use crate::lib::TransactionMember;

mod lib;

// use mock::AddrOrWallet;



use rand_chacha::ChaCha20Rng;


#[derive(Clone, Debug, libfuzzer_sys::arbitrary::Arbitrary)]
pub struct TxRandomInput {
    pub transactions_random_input: Vec<[u8; 128]>,
    pub transactions_random_to: [u8; 20],
    pub transactions_value: [u8; 32],
}

const NTX: usize = 100;

fuzz_target!(|tx_random_input: TxRandomInput| {

    println!("tx random input count: {:?}", tx_random_input.transactions_random_input.len());
    if tx_random_input.transactions_random_input.len() != NTX {
        return;
    }
    if tx_random_input.transactions_random_to == [0; 20] {
        return;
    }
    if tx_random_input.transactions_value == [0; 32] {
        return;
    }

    const MAX_CALLDATA: usize = 32;
    let chain_id: u64 = mock::MOCK_CHAIN_ID.as_u64();

    let mut transactions = vec![MockTransaction::default(); NTX];
    transactions = TransactionMember::<NTX>::randomize_transactions_vec_one_random_member(
        transactions,
        tx_random_input
    );
    let  transactions: Vec<Transaction>=
            transactions.iter_mut().map(|tx| {
                let tx_build = tx.build();
                println!("Tx after build: {:?}", tx_build);
                tx_build
            }).map(|tx| {
                let tx_transformed: Transaction = tx.clone().into();
                println!("Tx after tranformation: {:?}", tx_transformed);
                tx_transformed
            }).collect();

    println!("Input: {:?}", transactions);
    assert_eq!(run::<Fr>(transactions, chain_id, NTX, MAX_CALLDATA), Ok(()));

    // let mut rng = ChaCha20Rng::seed_from_u64(2u64);

    // let tx: Transaction = mock::CORRECT_MOCK_TXS[0].clone().into();
    // let tx: Transaction = MockTransaction::default()
    //     .from(mock::AddrOrWallet::random(&mut rng))
    //     .to(mock::AddrOrWallet::random(&mut rng))
    //     // .nonce(0x103u64)
    //     // .value(word!("0x3e8"))
    //     // .gas_price(word!("0x4d2"))
    //     // .input(Bytes::from(b"hello"))
    //     .build()
    //     .into();
    // println!("Transaction: {:?}", tx);
    // assert_eq!(run::<Fr>(vec![tx], chain_id, 1, 30), Ok(()));

});