# Requirements # 

solana-cli 1.17.2 (src:dfa5829b; feat:3079302975, client:SolanaLabs)

# Download # 

`git clone --recurse-submodules https://github.com/Certora/solana-program-library.git`

# Generation of the SBF file for Token 2022 #


```
cd token/program-2022
cargo build-sbf --arch=sbfv1 --features no-entrypoint certora

```

You should see the file `target/sbf-solana-solana/release/spl_token_2022.so`

