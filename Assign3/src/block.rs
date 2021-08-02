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
        let prev_hash = GenericArray::default();
        for mut entry in prev_hash {
            entry = 0u8;
        }

        Block {
            generation: 0,
            difficulty: difficulty,
            data: String::from(""),
            prev_hash: prev_hash,
            proof: None,
        }
    }

    pub fn next(previous: &Block, data: String) -> Block {
        // TODO: create and return a block that could follow `previous` in the chain
        Block {
            generation: previous.generation + 1,
            difficulty: previous.difficulty,
            data: data,
            prev_hash: previous.hash(),
            proof: None,
        }
    }

    pub fn hash_string_for_proof(&self, proof: u64) -> String {
        // TODO: return the hash string this block would have if we set the proof to `proof`.
        let mut hash_hex = String::new();
        write!(&mut hash_hex, "{:02x}", self.prev_hash).unwrap();
        let data = format!("{}:{}:{}:{}:{}",hash_hex,self.generation,self.difficulty,self.data,proof);
        return data;
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    pub fn hash_for_proof(&self, proof: u64) -> Hash {
        // TODO: return the block's hash as it would be if we set the proof to `proof`.
        let test_block = Block {
            generation: self.generation,
            difficulty: self.difficulty,
            data: self.data.clone(),
            prev_hash: self.prev_hash,
            proof: Some(proof),
        };
        let mut hash_hex = String::new();
        write!(&mut hash_hex, "{:02x}", test_block.prev_hash).unwrap();
        let data = format!("{}:{}:{}:{}:{}",hash_hex,test_block.generation,test_block.difficulty,test_block.data,proof);
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        return hash;
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
        let n_bytes = self.difficulty/8;
        let n_bits = self.difficulty%8;
        let hash = self.hash_for_proof(proof);
        for i in 32-n_bytes..32 {
            if hash[i as usize] != 0u8 {
                return false;
            }
        }
        if hash[31-n_bytes as usize]%(1<<n_bits) != 0 {
            return false;
        }

        return true;
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
        let arc = sync::Arc::new(self.clone());
        let mut q = WorkQueue::new(workers);
        let chunk_size = if end-start < chunks {1} else {(end-start)/chunks};
        let max_i = if end-start < chunks {end-start} else {chunks};

        for i in 0..max_i-1 {
            q.enqueue(MiningTask::new(sync::Arc::clone(&arc),i*chunk_size, ((i+1)*chunk_size)-1)).unwrap();
        }
        q.enqueue(MiningTask::new(sync::Arc::clone(&arc),chunk_size*(max_i-1), end)).unwrap();

        loop {
            let rec = q.recv();
            if self.is_valid_for_proof(rec) {
                q.shutdown();
                return rec;
            }
        }

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
    pub fn new(block: sync::Arc<Block>, start: u64, end: u64) -> MiningTask {
        MiningTask {
            block: block,
            start: start,
            end: end,
        }
    }
}

impl Task for MiningTask {
    type Output = u64;

    fn run(&self) -> Option<u64> {
        // TODO: what does it mean to .run?
        for i in self.start..=self.end {
            if self.block.is_valid_for_proof(i as u64) {
                return Some(i as u64);
            }
        }
        return None;
    }
}

// general idea

/*
    -we are given a general range to search for a proof in
    -create new worker queue of n workers
    -make sure that we are properly cloning self, so that each task has its own block to access
    -break up the ranges into n chunks, each of size range/nchunks
        - the last one being a special case
        - the rest of them are i*nchunks to (i+1)*nchunks
            - not inclusive for the end, for i = 0 to nchunks-1 (because of special case)
    - create a new mining task for each chunk

*/

// TODO: stop my hashing function from being circular, get mining working, tests