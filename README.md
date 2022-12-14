# Chingari Assignment

### Problem Statement

Fetch all the gari token `(CKaKtYvz6dKPyMvYq9Rh3UBrnNqYZAyd7iF4hJtjUvks)` related transactions, filter the transactions according to the start date and end date i.e the transaction date should be between start date and end date. From the transactions parse the *new user* whose pre balance < 0.0 and post balance > 0.0.

### When new user is created?

Whenever someone downloads the Chingari android application and signs up. A new Keypair is generated which is basically an account whose owner is system program which is a wallet. And a new ATA for gari token mint associated with the wallet. Then 0.000000001 gari token is airdropped to the users gari token ATA associated with their wallet.

That is we can check if new user has signed up on chingari.

Steps - 
* Create a wellet - (1) for user (Keypair).
* Create a gari token ATA (2) for wallet (1).
* Airdrop 0.000000001 gari token to their gari token ATA.

### How do we check new users from solana transactions.

1. We need to get the start date and end date filtered transactions.
2. Then we need to check the transactions meta data (1).
3. We need to fetch the pre_token_balances and post_token_balances from the transactions meta data (1).
4. pre_toke_balances and post_token_balances is basically a `Vec` of `UiTransactionTokenBalance`.
5. We need to check if pre_token_balances `Vec` is length zero, i.e before the transaction there is not gari token ATA for the user.
6. And after the transaction (post_token_balance) there is a gari token ATA for user and has a balance of `amount: "0"` because of the fact amount 0.000000001 gari token was airdropped to the users gari ATA, the amount is close to 0, thats why it shows `amount: "0"` on the transaction's meta data.
7. If the above is the case for the transcation that means this is the new user, get the token owner from the post_token_balances `Vec`.
8. Print all the data.

### How to run the program

1. Clone the repo, build the program using `$ cargo build --release`.
2. Run the program - `$ ./target/release/chingari_assignment -s 2022-11-13 -e 2022-12-14`.
3. Argument `-s` is the start date and `-e` is end date.
4. Or download the build file from relases tab form github repo and do the step 2.

```
$ ./target/debug/chingari_assignment -s 2022-11-13 -e 2022-12-14
...
...
...

real	6m18.197s
user	0m6.585s
sys	0m0.404s
```

Execution time of the program for the debug build ^
Release build will be much faster.

I have used the nonblocking `RpcClient` and async/await paradigm to make the execution as much faster as it can be. 

# Note 

This program fetch the last 1000 signatures related to the gari token, gari token has 1000 and 1000's of signatures each day. But the RpcClient can only give us around 1000 recent signatures (1) at a time. If want to get older signature you have to pass (1)'s last signature (2) into the Rpc call (`get_signatures_for_address_with_config()`) in this call you need to pass `before: last signature (2)` to get older signatures than (1).

If you repeat this process two times then you will get 2000 signatures, if repeated 3 times then 3000 signatures so on and so forth.

If suppose we entered a very historical date, such as few months back start_date and end_date then, the program needs to get a lots of signatures, so it will take a lots of time, also a lots of io operations as well. So I have not implemented that.