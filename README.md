# AV-evasion-payloads
This is an evaluation of payloads to run shellcode while evading anti-virus solutions. It is primarily focused around Windows Defender evasion, but I will try to include other scan results later. I will be using kleenscan.com for most scans, as antiscan.me seems to be having issues with payments. 
The main targets to be achieved with every single payload are as follows: 
1. It should be undetectable by Windows Defender.
2. It should have an overall detection rate of 5/40 or less (as judged by Kleenscan.com ) .

The steps that I took to make a payload  undetectable are listed in the various payload sections below. 

The focus is to see how effective each of these techniques remain in the current security  landscape, given that nothing used in this repo is a particularly new technique. Most of the code in this repo was taken from PoCs written by other people, and modified until it was undetectable to windows Defender. Multiple languages were also used , to see how effective their binaries are at evading detection with the same techniques. For these tests , cloud protection was kept on and sample submission turned off. At the moment , since this is limited to AV solutions, I will not be doing much code obfuscation or EDR evasion, and so these payloads are not OPSEC-safe and probably wont bypass EDR or manual inspection/debugging by an experienced malware analyst. 

This is basically a learning experience for me, so I am starting this with simple shellcode runners , with a few modifications like encryption or encoding, and moving onto more advanced techniques like packing,  and Donut shellcode for loading .NET assemblies. 
The payloads are categorized by language, and separated into the respective language folder (Rust, Nim, and C# as of now). 

The shellcode used, unless specified otherwise , is always an msfvenom reverse shell payload pointing to localhost, at port 4322. I  used a nc.exe listener to catch the reverse shells , to prove that it worked. 

## Rust payloads
1. Shellcode runner with AES256 encryption and base64 encoding of shellcode: 
A basic shellcode loader in rust, with encryption and encoding used. The shellcode is injected into a chosen application after creating a new process and suspending it.  The shellcode is directly inserted into the code, after being separately encrypted by an encryptor program (also included). 
The original program was modified to remove extra string output , which was triggering defender. The process to be used was changed to the Edge browser , since that is common on windows systems and is  less suspicious than notepad, calc or explorer (these three are the most commonly used in PoCs. Also,  heuristic based detection should catch the fact that these three are making weird connections over the network, when they don't need to).
Another important thing to do is to compile it as a  release instead of using the debug setting. The --release flag should be added to cargo while running.
Detection reults :

![image](https://github.com/user-attachments/assets/73c4685d-8b5a-446c-ae55-f999959abb18)


2. 



## C-sharp payloads
1. Basic Shellcode runner :  
The basic shellcode runner in C-sharp is ridiculously simple, and is somehow still undetectable by Defender. A more advanced version of this is shown in the next example. The code is taken from [this repo](https://gist.github.com/matterpreter/03e2bd3cf8b26d57044f3b494e73bbea). I have included it just to show that even this will work against Defender currently, as of early 2025. Maybe if given a few more months , this signature will be picked up by vendors.
The code simply allocates memory for the shellcode array, loads the shellcode into the newly created memory and then creates a new thread for it. 

2. AES 256 Shellcode stager that downloads remote shellcode and runs it :
This is an advanced and heavily modified version of the previous loader, that will first obtain the base64 encoded  and encrypted shellcode from a remote server and then run it. AES 256 encryption was also added. Some code snippets for the encryption were taken from [this repo](https://github.com/Tw1sm/SharpInjector/blob/master/ScEncryptor/Program.cs). The AES- 256 encryptor program is also included for this particular stager's shellcode.
One more key change that I made in this version is to make sure the command prompt window does not pop up on running the payload, which would be quite suspicious. This was done by changing the output type  to be a windows application, in the properties in Visual Studio 2022. I hosted the shellcode file called sc.txt on a server on localhost and and ran the nc.exe listener as usual. 

## Nim payloads 
1. Nim encrypted shellcode loader: 
Original code was taken from [this amazing repo on offensive Nim code](https://github.com/S3cur3Th1sSh1t/Creds/blob/master/nim/encrypted_shellcode_loader.nim) .
 As with the basic rust payload, the encryptor program is included , and will output the AES256 encrypted shellcode in base64 encoding. This is directly put into the loader's code. Again compiling as release and using msedge.exe as the process to be injected into is essential, as notepad is too suspicious. Unnecessary string output was removed.

### References 
This repo was built using the code from many places, and the knowledge and techniques used came from many more  blogs, repos, tutorials written by other people, some of which have been lost in my browser history. I have tried to include all the major ones here for anyone to see the source material. I thank all the contributors of these links for their amazing efforts to spread this knowledge, even those that I missed out in this list here. 

1. [https://github.com/trickster0/OffensiveRust/tree/master](https://github.com/trickster0/OffensiveRust/tree/master)
2. [https://github.com/trickster0/OffensiveRust/tree/master](https://github.com/trickster0/OffensiveRust/tree/master)
3. [https://github.com/brosck/RustSCLoader/blob/main/src/main.rs](https://github.com/brosck/RustSCLoader/blob/main/src/main.rs)
4. [https://tryhackme.com/r/room/avevasionshellcode](https://tryhackme.com/r/room/avevasionshellcode)
5. [https://github.com/byt3bl33d3r/OffensiveNim/blob/master/src/shellcode_bin.nim](https://github.com/byt3bl33d3r/OffensiveNim/blob/master/src/shellcode_bin.nim)
6. [https://github.com/safedv/Rustic64](https://github.com/safedv/Rustic64)
7. [https://github.com/mvelazc0/defcon27_csharp_workshop/blob/master/Labs/lab3/1.cs](https://github.com/mvelazc0/defcon27_csharp_workshop/blob/master/Labs/lab3/1.cs)
8. [https://www.ired.team/offensive-security/code-injection-process-injection/injecting-and-executing-.net-assemblies-to-unmanaged-process](https://www.ired.team/offensive-security/code-injection-process-injection/injecting-and-executing-.net-assemblies-to-unmanaged-process)
9. [https://fgsec.net/posts/Injecting-dotNet-Assemblies-To-Unmanaged-Processes/](https://fgsec.net/posts/Injecting-dotNet-Assemblies-To-Unmanaged-Processes/)

