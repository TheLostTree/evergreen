## Npcap

execute this in your commandline(cmd) after installing npcap + the sdk

``set LIBPCAP_LIBDIR=C:\path\to\npcapsdk\Lib\x64``

eg.

``set LIBPCAP_LIBDIR=C:\Users\admin\Documents\npcapsdk\Lib\x64``

and 

``$env:LIBPCAP_LIBDIR='C:\Users\admin\Documents\npcapsdk\Lib\x64'``

with powershell

also install openssl, compiling it from source takes forever.

and set OPENSSL_DIR in your environment variables


~~~~~

drop in all the protos you want into ./protos and then just wait a long time to compile lol

yes, it will take 5 minutes with all 3k protobuf fies

no, i have no clue how to fix it



~~~~~~


todo:
1. some kind of ui (dps meter a la chrome)
2. ws connection to iridium
3. properly do unknown parsing
