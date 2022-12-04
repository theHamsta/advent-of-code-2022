# Just file: https://github.com/casey/just

bench-go:
	go build
	hyperfine -P day 1 4 -N --warmup 10 "./advent-of-code-2022 --day {day}"

bench-rust:
	cd rust/day01/ && cargo build --release && hyperfine -N --warmup 10 "target/release/day01"
	cd rust/day03/ && cargo build --release && hyperfine -N --warmup 10 "target/release/day03"
	cd rust/day04/ && cargo build --release && hyperfine -N --warmup 10 "target/release/day04"

clean-bench:
	rm -f profile/*.prof
	rm -f profile/*.svg
