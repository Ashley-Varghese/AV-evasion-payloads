# AV-evasion-payloads
This is an evaluation of payloads to run shellcode while evading anti-virus solutions. It is primarily focused around Windows Defender evasion, but I will try to include other scan results later (At the moment, sites like antiscan.me do not work, making it difficult to test against other vendors).

The focus is to see how effective each of these techniques remain in the current security  landscape, given that nothing used in this repo is a particularly new technique. Most of the code in this repo was taken from PoCs written by other people, and modified until it was undetectable to windows Defender. Multiple languages were also used , to see how effective their binaries are at evading detection with the same techniques. For these tests , cloud protection was kept on and sample submission turned off. At the moment , since this is limited to AV solutions, I will not be doing much code obfuscation or EDR evasion, and so these payloads are not OPSEC-safe and probably wont bypass EDR or manual inspection/debugging by an experienced security analyst. 

This is basically a learning experience for me, so I am starting this off with simple shellcode runners , with a few modifications like encryption or encoding, and moving onto more advanced techniques like packing,  and Donut shellcode for loading .NET assemblies. 
The payloads are categorized by language, and separated into the respective language folder (Rust, Nim, and C# as of now). 

The shellcode used, unless specified otherwise , is always an msfvenom reverse shell payload pointing to localhost, at port 4322. I  used a nc.exe listener to catch the reverse shells , to prove that it worked. 

## Rust payloads
1. Shellcode runner with AES256 encryption and base64 encoding of shellcode: 
A basic shellcode loader in rust, with encryption mad encoding used. The shellcode is directly inserted into the code, after being separately encrypted by an encryptor program (also included). 

## C-sharp payloads
1. Basic Shellcode runner :  
The basic shellcode runner in C-sharp is ridiculously simple, and is somehow still undetectable by Defender. A more advanced version of this is shown in the next example. The code is taken from [this repo](https://gist.github.com/matterpreter/03e2bd3cf8b26d57044f3b494e73bbea). I have included it just to show that even this will work against Defender currently, as of early 2025. maybe given a few more months , this signature will be picked up by vendors.
The code simply allocates memory for the shellcode array, loads the shellcode into the newly created memory and then creates a new thread for it. 

2. Shellcode stager that obtains remote shellcode and runs it :
This is an advanced version of the previous loader, that will first obtain the base64 encoded shellcode from a remote server and then run it. 

