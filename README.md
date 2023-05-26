# Evergreen

a dntk successor!

also works as a backend for iridium!


# Undergoing a rewrite probably


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



# Updating to a new game version
1. update protos by dropping new protos into ./all_protos
2. update cmdids by either 
    a) updating packetIds.json then running index.js + the generateCmdIdsCsv function
    b) update cmdids.csv directly
3. hopefully no name changes are in the protos, but fix if they appear

todo:

1. properly do unknown parsing
2. some kind of tauri gui window (various reasons)