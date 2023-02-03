## Npcap

execute this in your commandline(cmd) after installing npcap + the sdk

``set LIBPCAP_LIBDIR=C:\path\to\npcapsdk\Lib\x64``

eg.

``set LIBPCAP_LIBDIR=C:\Users\admin\Documents\npcapsdk\Lib\x64``

and 

``$env:LIBPCAP_LIBDIR='C:\Users\admin\Documents\npcapsdk\Lib\x64'``

with powershell

openssl = {version = "0.10.0", features = ["vendored"]}
openssl = "0.10.0"


```rust
let mut contents = String::new();
for byte in &mut *data{
    contents.push_str(&format!("{:02x}", byte))
}

```