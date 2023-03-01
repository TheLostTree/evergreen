# Evergreen

a dntk successor!

also works as a backend for iridium!



## Npcap

execute this in your commandline(cmd) after installing npcap + the sdk

``set LIBPCAP_LIBDIR=C:\path\to\npcapsdk\Lib\x64``

eg.

``set LIBPCAP_LIBDIR=C:\Users\admin\Documents\npcapsdk\Lib\x64``

and 

``$env:LIBPCAP_LIBDIR='C:\Users\admin\Documents\npcapsdk\Lib\x64'``

with powershell



# Building
drop in all the protos you want into ./all_protos 

run index.js to move a few protos into ./protos (the ones that ares statically compiled for use in the actual code)




todo:

1. properly do unknown parsing
2. some kind of tauri gui window (various reasons)