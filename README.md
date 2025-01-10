# AV-evasion-payloads
This is an evaluation of payloads to run shellcode while evading anti-virus solutions. It is primarily focused around Windows Defender evasion, but I will try to include other scan results later (At the moment, sites like antiscan.me do not work, making it difficult to test against other vendors).

The focus is to see how effective each of these techniques remain in the current landscape, given that that nothing used in this repo is a particularly new technique. Most of the code in this repo was taken from PoCs written by other people, and modified until it was undetectable to windows Defender. For the tests , cloud protection was kept on and sample submission turned off. At the moment , since this is limited to AV solutions, I will not be doing much code obfuscation or EDR un hooking, and so these payloads are not OPSEC-safe and probably wont bypass EDR or manual inspection/debugging by an experienced security analyst. 

This is primarily a learning experience for me, so I am starting this off with simple shellcode runners , with a few modificaations like encryption or encoding, and moving onto more advanced techniques like packing,  and Donut shellcode for loading .NET assemblies. 
The payloads are categorized by language, and separated into the respective language folder (Rust, Nim, and C# as of now). 

The shellcode used, unless specified otherwise , is always an msfvenom reverse shell payload pointing to localhost, at port 4322. I  used a nc.exe listener to catch the reverse shells , to prove that it worked. 

# C-sharp payloads
1. The basic shellcode runner in C-sharp is ridiculously simple, and is somehow still undetectable by Defender. 
