# Blockchains-with-Proof-of-work
The implementation and the "mining" of a simple blockchain system with proof-of-work.    
  
A simple blockchain system with proof-of-work (cryptographically hashed values ending in a certain number of 0's). The proof-of-work are computed in parallel with concurrency tools that include OS-level threads, mpsc/spmc 
message queues, and atomic reference counting.  
  
The implementation includes the "mining" of constructed blocks by using worker threads to check possible proof-of-work values and returning the first valid proof found. Tests for its functions are also included.
