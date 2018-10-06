MHC Server mockup
=================

Compiling:

1. Get Rust: https://rustup.rs
2. `cd [this directory]`
3. `cargo build --release`
4. Get binary: `target/release/mhc` or `target/release/mhc.exe` on Windows

Usage:

```
$ cargo run 0.0.0.0:5000

Binding to: 0.0.0.0:5000
```

Now throw some UDP packet at it:

```
echo -en "\x15\x06\x01\x00\x10\x66" | nc -4u -w1 -p 50001 127.0.0.1 5000
```

You should see on mhc console:

```
Valid MHC packet from: 127.0.0.1:50001 data: MHCPacket { protocol_id: 21, size: 6, command_type: 1, head: 0, preset: 16, signature: 102 }
```
