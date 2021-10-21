use crate::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::sync;

type Hash = GenericArray<u8, U32>;

#[derive(Debug, Clone)]
pub struct Block {
    prev_hash: Hash,
    generation: u64,
    difficulty: u8,
    data: String,
    proof: Option<u64>,
}

impl Block {
    pub fn initial(difficulty: u8) -> Block {
        // TODO: create and return a new initial block
        Block {
            prev_hash: Hash::default(),
            generation: 0,
            difficulty: difficulty,
            data: String::new(),
            proof: None,
        }
    }

    pub fn next(previous: &Block, data: String) -> Block {
        // TODO: create and return a block that could follow `previous` in the chain
        Block{
            prev_hash: previous.hash(),
            generation: previous.generation + 1,
            difficulty: previous.difficulty,
            data: data,
            proof: None,
        }
    }

    pub fn hash_string_for_proof(&self, proof: u64) -> String {
        // TODO: return the hash string this block would have if we set the proof to `proof`.
        let mut v = Vec::new();
        v.push(format!("{:02x}", self.prev_hash));
        v.push(self.generation.to_string());
        v.push(self.difficulty.to_string());
        v.push(self.data.clone());
        v.push(proof.to_string());
        v.join(":")
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    pub fn hash_for_proof(&self, proof: u64) -> Hash {
        // TODO: return the block's hash as it would be if we set the proof to `proof`.
        let mut hasher = Sha256::new();
        hasher.update(self.hash_string_for_proof(proof));
        hasher.finalize()
    }

    pub fn hash(&self) -> Hash {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_for_proof(p)
    }

    pub fn set_proof(self: &mut Block, proof: u64) {
        self.proof = Some(proof);
    }

    pub fn is_valid_for_proof(&self, proof: u64) -> bool {
        // TODO: would this block be valid if we set the proof to `proof`?
        let arr = self.hash_for_proof(proof);
        let n_bytes = usize::from(self.difficulty / 8);
        let n_bits = usize::from(self.difficulty % 8);

        // Check each of the last n bytes are 0
        for i in arr.len()-n_bytes..arr.len() {
            if arr[i] != 0u8 {
                return false;
            }
        }
        // Check next byte is divisible by 1<<n_bits
        if arr[arr.len()-n_bytes-1] % (1<<n_bits) != 0 {
            return false;
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        if self.proof.is_none() {
            return false;
        }
        self.is_valid_for_proof(self.proof.unwrap())
    }

    // Mine in a very simple way: check sequentially until a valid hash is found.
    // This doesn't *need* to be used in any way, but could be used to do some mining
    // before your .mine is complete. Results should be the same as .mine (but slower).
    pub fn mine_serial(self: &mut Block) {
        let mut p = 0u64;
        while !self.is_valid_for_proof(p) {
            p += 1;
        }
        self.proof = Some(p);
    }

    pub fn mine_range(self: &Block, workers: usize, start: u64, end: u64, chunks: u64) -> u64 {
        // TODO: with `workers` threads, check proof values in the given range, breaking up
	// into `chunks` tasks in a work queue. Return the first valid proof found.
        // HINTS:
        // - Create and use a queue::WorkQueue.
        // - Use sync::Arc to wrap a clone of self for sharing.
        let mut queue = WorkQueue::new(workers);
        let block = sync::Arc::new(self.clone());

        let step = (end-start+1) / chunks;
        if step == 0 {
            // If step==0, grab the whole thing
            let task = MiningTask::new(block.clone(), start, end);
            queue.enqueue(task).unwrap();
            let proof = queue.recv();
            queue.shutdown();
            return proof;
        }

        for i in (start..=end).step_by(step as usize) {
            let task = MiningTask::new(block.clone(), i, i+step-1);
            queue.enqueue(task).unwrap();

            // Try to receive optimistically in every enqueue operation
            let possible_proof = queue.try_recv();
            if possible_proof.is_ok() {
                queue.shutdown();
                return possible_proof.unwrap();
            }
        }
        // Enqueue the remaining piece
        let remaining = (end-start+1) % chunks;
        if remaining != 0 {
            let task = MiningTask::new(block.clone(), end-remaining+1, end);
            queue.enqueue(task).unwrap();
        }

        // Wait to receive
        let proof = queue.recv();
        queue.shutdown();
        proof
    }

    pub fn mine_for_proof(self: &Block, workers: usize) -> u64 {
        let range_start: u64 = 0;
        let range_end: u64 = 8 * (1 << self.difficulty); // 8 * 2^(bits that must be zero)
        let chunks: u64 = 2345;
        self.mine_range(workers, range_start, range_end, chunks)
    }

    pub fn mine(self: &mut Block, workers: usize) {
        self.proof = Some(self.mine_for_proof(workers));
    }
}

struct MiningTask {
    block: sync::Arc<Block>,
    // TODO: more fields as needed
    start: u64,
    end: u64,
}

impl MiningTask {
    // TODO: implement MiningTask::new(???) -> MiningTask
    fn new(block: sync::Arc<Block>, start: u64, end: u64) -> MiningTask {
        MiningTask {
            block, start, end
        }
    }
}

impl Task for MiningTask {
    type Output = u64;

    fn run(&self) -> Option<u64> {
        // TODO: what does it mean to .run?
        for p in self.start..=self.end {
            if self.block.is_valid_for_proof(p) {
                return Some(p);
            }
        }
        None
    }
}
