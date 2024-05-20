# SNARKey
CPA-secure zk-friendly encryption

## Encryption algorithm
We base the algorithm on the assumption that the zk-friendly hash function Poseidon is pseudorandom. 
However, the use of the hash function is black-box and any other zk-friendly pseudorandom function could be used in its place.

To compute $\mathsf{Enc}(k, m)$, where $k \in \mathbb{F}, m \in \mathbb{F}$:
1. Sample $\mathsf{nonce} \leftarrow_\\$ \mathbb{F}$
2. Compute $\mathsf{pad} := H(k, \mathsf{nonce})$
3. Output the ciphertext $\mathsf{pad} + m$

In the general case of encrypting a vector $m \in \mathbb{F}^n, k \in \mathbb{F}$, we define $\mathsf{Enc}(k,m)$ in the following way:
1. Sample $\mathsf{nonce} \leftarrow_\\$ \mathbb{F}$
2. Compute $\mathsf{seed} := H(k, \mathsf{nonce})$
3. For $i := 1, \dots, n$, compute $\mathsf{pad}_i := H(\mathsf{seed}, i)$
4. Output the ciphertext $\left(\mathsf{pad}_1 + m_1, \dots, \mathsf{pad}_n + m_n\right)$

This algorithm is very similar to the one published by [AlephZero](https://docs.alephzero.org/aleph-zero/protocol-details/shielder/snark-friendly-symmetric-encryption), 
however, we arrived at it independently.
