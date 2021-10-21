#[cfg(test)]
mod block_tests {
    use crate::queue::{Task, WorkQueue};
    use crate::block::{Block};

    #[test]
    fn hashing_blocks() {
        let mut b0 = Block::initial(16);
        b0.set_proof(56231);
        assert_eq!("0000000000000000000000000000000000000000000000000000000000000000:0:16::56231", b0.hash_string());
        let mut b1 = Block::next(&b0, String::from("message"));
        b1.set_proof(2159);
        assert_eq!("6c71ff02a08a22309b7dbbcee45d291d4ce955caa32031c50d941e3e9dbd0000:1:16:message:2159", b1.hash_string());
        let b0_hash = Block::hash(&b0);
        let b1_hash = Block::hash(&b1);
        let b0_hash_f = format!("{:02x}", b0_hash);
        let b1_hash_f = format!("{:02x}", b1_hash);
        assert_eq!("6c71ff02a08a22309b7dbbcee45d291d4ce955caa32031c50d941e3e9dbd0000", b0_hash_f);
        assert_eq!("9b4417b36afa6d31c728eed7abc14dd84468fdb055d8f3cbe308b0179df40000", b1_hash_f);
    }

    #[test]
    fn valid_hashes() {
        let mut b0 = Block::initial(19);
        b0.set_proof(87745);
        let mut b1 = Block::next(&b0, String::from("hash example 1234"));
        b1.set_proof(1407891);
        assert_eq!(b0.is_valid(), true);
        assert_eq!(b1.is_valid(), true);
        b1.set_proof(346082);
        assert_eq!(b1.is_valid(), false);
    }

    #[test]
    // A difficulty-20 mining test took 327s.
    // So this is only going to include the lower difficulty ones
    fn mining_tasks() {
        let mut b0 = Block::initial(7);
        b0.mine(1);
        assert_eq!(b0.hash_string(), "0000000000000000000000000000000000000000000000000000000000000000:0:7::385");

        let mut b1 = Block::initial(8);
        b1.mine(2);
        let mut b2 = Block::initial(8);
        b2.mine_serial();
        assert_eq!(b1.hash_string(), b2.hash_string());

        // Larger difficulty, varied number of workers, a chain created with "next".
        let mut b3 = Block::initial(12);
        b3.mine(10);
        let mut b4 = Block::next(&b3, "the future is better than b4".to_string());
        b4.mine(7);
        let mut b5 = Block::next(&b4, "because we have--".to_string());
        b5.mine(100);
        let mut b6 = Block::next(&b5, "rust blockchains".to_string());
        b6.mine(1000);
        let mut b7 = Block::initial(12);
        b7.mine_serial();
        let mut b8 = Block::next(&b7, "the future is better than b4".to_string());
        b8.mine_serial();
        let mut b9 = Block::next(&b8, "because we have--".to_string());
        b9.mine_serial();
        let mut b10 = Block::next(&b9, "rust blockchains".to_string());
        b10.mine_serial();
        assert_eq!(b6.hash_string(), b10.hash_string());

        // Check if prev_hash matches previous hash
        let b5_hash = format!("{:02x}", Block::hash(&b5));
        let b6_hash_string = b6.hash_string();
        let b6_prev_hash_vec:Vec<&str> = b6_hash_string.split(":").collect();
        let b6_prev_hash = b6_prev_hash_vec[0];
        assert_eq!(b5_hash, b6_prev_hash);
    }
}
