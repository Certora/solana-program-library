# Requirements # 

Solana version: solana-cli 1.14.16 (src:0fb2ffda; feat:3488713414)

# Download # 

`git clone --recurse-submodules https://github.com/Certora/solana-program-library.git`

# Generation of the SBF file #


`cargo build-sbf --arch=sbfv2`

You should see the file `target/sbf-solana-solana/release/spl_token_2022.so`


# Run Rules #

```
cd token/program-2022/src
mkdir certora_files && cd certora_files
../certora/verify proper_use_of_encryption_key_process_empty_account
../certora/verity proper_use_of_encryption_key_process_withdraw_account
../certora/verify integrity_of_process_mint_to
../certora/verify integrity_of_process_mint_to_false
```
