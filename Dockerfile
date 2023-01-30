FROM rust:slim
COPY ./target/release/wallets-rebalance-watcher ./target/release/wallets-rebalance-watcher
ENTRYPOINT ["./target/release/wallets-rebalance-watcher"]