# Winner of the [AlephZero](https://zk-hack-krakow.devfolio.co/prizes?partner=Aleph+Zero) [hacker track](https://zk-hack-krakow.devfolio.co/projects) at [ZK Hack Krakow 2024](https://www.zkkrakow.com/)
In-circuit data encryption: In the context of privacy systems with anonymity revokers (see for instance this [example](https://docs.alephzero.org/aleph-zero/protocol-details/shielder/anonymity-revokers)) one must provide encryptions of in-circuit witnesses, thus really implement in-circuit encryption. In this bounty we would like you to implement and benchmark a symmetric encryption scheme as a Halo2 circuit. An example basic design is provided here in [this doc](https://docs.alephzero.org/aleph-zero/protocol-details/shielder/snark-friendly-symmetric-encryption), and other useful resources include this [paper](https://eprint.iacr.org/2023/520.pdf), and this [blog article](https://blog.taceo.io/how-to-choose-your-zk-friendly-hash-function/).

# [SNARKey](https://devfolio.co/projects/snarkey-77dd)
CPA-secure zk-friendly encryption, developed during ZK Hack Kraków 2024.

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

This algorithm is very similar to the one published by [AlephZero](https://docs.alephzero.org/aleph-zero/protocol-details/shielder/snark-friendly-symmetric-encryption), however, we arrived at it independently.
