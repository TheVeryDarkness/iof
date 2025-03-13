set -eux

export CARGO_PROFILE_BENCH_DEBUG=true

# cargo flamegraph --root --bench read -o short_read_n.svg -- --bench

cargo flamegraph --root --bench read -o short_read_n.svg -- --bench 'cursor - short - read_n'
cargo flamegraph --root --bench read -o short_read_to_end.svg -- --bench 'cursor - short - read_to_end split'

cargo flamegraph --root --bench read -o long_read_n.svg -- --bench 'cursor - long - read_n'
cargo flamegraph --root --bench read -o long_read_to_end.svg -- --bench 'cursor - long - read_to_end split'

CARGO_PROFILE_BENCH_DEBUG=true cargo run --example read --profile bench -- --bench --profile-time 5
