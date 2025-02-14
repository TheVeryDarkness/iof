set -eux

sudo cargo flamegraph --bench read -o short_read_n.svg -- --exact 'cursor - short - read_n'
sudo cargo flamegraph --bench read -o short_read_to_end.svg -- --exact 'cursor - short - read_to_end split'

sudo cargo flamegraph --bench read -o long_read_n.svg -- --exact 'cursor - long - read_n'
sudo cargo flamegraph --bench read -o long_read_to_end.svg -- --exact 'cursor - long - read_to_end split'
