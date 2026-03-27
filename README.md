# Bash Run Command
Extremely unsafe, minimalistic backend to execute predefined commands in bash environment.  

The size of this shit is ~1Mb, it could become even less without `dotenvy` but I was too lazy to remove `dotnevy:var()` calls after I realized I don't really need a `.env` parsing. X)  
Anyway, 1Mb is more than enough for my use case.

## Why?
I had to provide some possibility to run commands on nginx request.  
Of course it required some minimal HTTP server to serve requests and a minimal API handling.  
That's how this shit was born.  

## Possibly no support
This shit is a micro-util to supply my needs so, possibly, it will **NEVER** be updated in future.  
It was pushed to git so I could easily clone and run it anywhere I need in future.

## Be careful!
Using this MUST NEVER EVER call commands provided from any other source than `.env` or program environment which is controlled by YOU!  
The reason for that is commands are being lazily executed from `bash`, this means that anyone with ability to change `COMMAND` env variable could exploit your host/docker container.
