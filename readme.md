MHC Server mockup
=================

Compiling:

1. Get Rust: https://rustup.rs
2. `cd [this directory]`
3. `cargo build --release`
4. Get binary: `target/release/mhc` or `target/release/mhc.exe` on Windows

Usage:

```
cargo run 0.0.0.0:5000

Binding to: 0.0.0.0:5000
```

Now throw some UDP packet at it:

Unix (using netcat):

```sh
$ echo -en "\x15\x6\x1\x0\x63\x66" | nc -4u -w1 -p 50001 127.0.0.1 5000
```

On Windows using (included) PowerShell script:

```ps
.\mockup_client.ps1 -preset 99 -ip 127.0.0.1 -port 5000 -srcport 50002
```

You should see on mhc console:

```
Valid MHC packet from: 127.0.0.1:50001 data: MHCPacket { protocol_id: 21, size: 6, command_type: 1, head: 0, preset: 99, signature: 102 }
```
