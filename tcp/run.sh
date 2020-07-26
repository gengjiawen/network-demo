#!/bin/bash
cargo build
ext=$?
if [[ $ext -ne 0 ]]; then
	exit $ext
fi
sudo setcap cap_net_admin=eip ./target/debug/tcp
./target/debug/tcp &
sleep 0.2
pid=$!
echo 'running'
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid