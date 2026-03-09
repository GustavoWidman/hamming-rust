# hamming-rust

![rust](https://img.shields.io/badge/rust-2024-orange?logo=rust)
![license](https://img.shields.io/badge/license-MIT-blue)

teaching rocks to fix themselves. a university assignment that i overengineered because i wanted to use rust.

> **want the full story on how i built this + an explanation on how hamming codes work?** 
> check out my blog post: [blog.r3dlust.com/hamming-rust](https://blog.r3dlust.com/hamming-rust)

## what is this?

this is a rust implementation of hamming code error correction, wrapped in a completely made-up data link layer frame format called the "GUS protocol" (Great Unused Standard).

it's a cli pipeline. you give it some data, it encodes it with redundant parity bits, maliciously flips a bit 50% of the time to simulate a noisy wire, and spits out the raw bytes. pipe that into the receiver, and it will read the frame, mathematically figure out exactly which bit flipped, fix it, and give you your original data back.

it just works.

## features

- **custom bitvector:** rust doesn't have a great way to handle individual bits natively, so i wrote my own `BitVec` implementation that packs bits into bytes. it was a nightmare of big-endian bit ordering and off-by-one index errors.
- **hamming codes:** single error correction (sec). triangulates broken bits using overlapping parity groups based on powers of 2.
- **gus protocol:** a 20-byte frame header that wraps the encoded payload just to satisfy the assignment requirements.
- **split logging:** logs go to stderr, raw binary data goes to stdout. this means you can actually pipe the output between processes without the logs corrupting the binary payload.

## running it

you'll need the rust toolchain (edition 2024).

```bash
cargo build --release
```

the cli supports two modes (`sender` and `receiver`) and two data types (`binary` and `text`).

**send and receive some binary:**
```bash
./target/release/hamming_rust -t binary sender -d "01101001" | ./target/release/hamming_rust -t binary receiver
```

**send and receive some text:**
```bash
./target/release/hamming_rust -t text sender -d "hello world" | ./target/release/hamming_rust -t text receiver
```

if you watch the logs, about 50% of the time you'll see the receiver log `[WRN] [RECEIVER] Corrected an error at position X`. this is the hamming code doing its job.

## the frame format

because this was a computer networks assignment, the data couldn't just be raw bits. it had to be framed.

| Offset | Size | Field |
|--------|------|-------|
| 0 | 3 bytes | Magic string: "GUS" |
| 3 | 1 byte | Version: `0x01` |
| 4 | 8 bytes* | Byte length of payload |
| 12 | 8 bytes* | Exact bit length of payload |
| 20 | Variable | Hamming-encoded payload bytes |

*\* Note: The length fields use `usize`, so the frame size is architecture-dependent. A frame built on a 64-bit machine cannot be decoded on a 32-bit machine. I could fix this by using `u64`, but it's a university assignment and it already works.*

## benchmarks

i also threw in some criterion benchmarks for encoding and decoding payloads up to 64kb, because why not.

```bash
cargo bench
```

## license

mit. do whatever you want with it.