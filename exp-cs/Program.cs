using System.Runtime.InteropServices;
using IC;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;

var keyStore1 = Agent.CreateKeyStore("Allen Pocket", "123456");
Console.WriteLine($"[x] Create first keystore: {keyStore1}\n");

var keyStore2 = Agent.CreateKeyStore("Allen Pocket", "123456");
Console.WriteLine($"[x] Create second keystore: {keyStore2}\n");

var receipt1 = Agent.LoginByHost(keyStore1, "123456");
Console.WriteLine($"[x] Login by first keystore: {receipt1}\n");

var receipt2 = Agent.LoginByHost(keyStore2, "123456");
Console.WriteLine($"[x] Login by second keystore: {receipt2}\n");

receipt1 = Agent.GetLoggedReceipt(receipt1.Principal);
Console.WriteLine($"[x] Get first login info: {receipt1}\n");

receipt2 = Agent.GetLoggedReceipt(receipt2.Principal);
Console.WriteLine($"[x] Get second login info: {receipt2}\n");

var receipts = Agent.ListLoggedReceipt();
Console.WriteLine($"[x] List login infos:");
for (int i = 0; i < receipts.Length; i++)
{
    Console.WriteLine($"    {i}. {receipts[i]}");
}
Console.WriteLine();

Agent.Logout(receipt1.Principal);
Console.WriteLine($"[x] Logout first keystore: {receipt1}\n");

Agent.Logout(receipt2.Principal);
Console.WriteLine($"[x] Logout second keystore: {receipt2}\n");

namespace IC
{
    public static class Agent
    {
        private readonly struct Response
        {
            public IntPtr Ptr { get; init; }
            public bool IsErr { get; init; }
        }

        [DllImport("agent.dylib")]
        private static extern Response create_keystore([MarshalAs(UnmanagedType.LPStr)] string req);

        [DllImport("agent.dylib")]
        private static extern void free_rsp(Response rsp);

        [DllImport("agent.dylib")]
        private static extern Response login_by_host([MarshalAs(UnmanagedType.LPStr)] string req);

        [DllImport("agent.dylib")]
        private static extern Response get_logged_receipt([MarshalAs(UnmanagedType.LPStr)] string req);

        [DllImport("agent.dylib")]
        private static extern Response list_logged_receipt();

        [DllImport("agent.dylib")]
        private static extern Response logout([MarshalAs(UnmanagedType.LPStr)] string req);

        public static HostKeyStore CreateKeyStore(string name, string password)
        {
            var req = $@"{{""name"": ""{name}"", ""password"": ""{password}""}}";
            var rsp = create_keystore(req);

            var data = Marshal.PtrToStringAnsi(rsp.Ptr);
            free_rsp(rsp);

            if (data == null) throw new Exception("inner error: data from rust-lib is null");
            if (rsp.IsErr) throw new Exception(data);

            var keyStore = JsonConvert.DeserializeObject<HostKeyStore>(data);

            return keyStore;
        }

        public static LoggedReceipt LoginByHost(HostKeyStore keyStore, string password)
        {
            var keyStoreStr = JsonConvert.SerializeObject(keyStore, new JsonSerializerSettings
            {
                ContractResolver = new DefaultContractResolver
                {
                    NamingStrategy = new CamelCaseNamingStrategy()
                }
            });
            
            var req = $@"{{""keyStore"": {keyStoreStr}, ""password"": ""{password}""}}";
            var rsp = login_by_host(req);

            var data = Marshal.PtrToStringAnsi(rsp.Ptr);
            free_rsp(rsp);

            if (data == null) throw new Exception("inner error: data from rust-lib is null");
            if (rsp.IsErr) throw new Exception(data);

            var receipt = JsonConvert.DeserializeObject<LoggedReceipt>(data);

            return receipt;
        }

        public static void Logout(string principal)
        {
            var req = $@"{{""principal"": ""{principal}""}}";
            var rsp = logout(req);

            var data = Marshal.PtrToStringAnsi(rsp.Ptr);
            free_rsp(rsp);

            if (data == null) throw new Exception("inner error: data from rust-lib is null");
            if (rsp.IsErr) throw new Exception(data);
        }

        public static LoggedReceipt GetLoggedReceipt(string principal)
        {
            var req = $@"{{""principal"": ""{principal}""}}";
            var rsp = get_logged_receipt(req);

            var data = Marshal.PtrToStringAnsi(rsp.Ptr);
            free_rsp(rsp);

            if (data == null) throw new Exception("inner error: data from rust-lib is null");
            if (rsp.IsErr) throw new Exception(data);

            var receipt = JsonConvert.DeserializeObject<LoggedReceipt>(data);

            return receipt;
        }

        public static LoggedReceipt[] ListLoggedReceipt()
        {
            var rsp = list_logged_receipt();

            var data = Marshal.PtrToStringAnsi(rsp.Ptr);
            free_rsp(rsp);

            if (data == null) throw new Exception("inner error: data from rust-lib is null");
            if (rsp.IsErr) throw new Exception(data);

            var receipts = JsonConvert.DeserializeObject<LoggedReceipt[]>(data);

            return receipts;
        }
    }

    public struct HostKeyStore
    {
        public string Encoded { get; init; }
        public string Principal { get; init; }
        public HostKeyStoreMeta Meta { get; init; }

        public override string ToString() => $"{{encoded: {Encoded}, principal: {Principal}, meta: {Meta}}}";
    }

    public struct HostKeyStoreMeta
    {
        public string Name { get; set; }
        public DateTime WhenCreated { get; init; }
        public string StoreSyntax { get; init; }
        public string SigScheme { get; init; }
        public string[] EncryptScheme { get; init; }

        public override string ToString() =>
            $"{{name: {Name}, whenCreated: {WhenCreated}, storeSyntax: {StoreSyntax}, sigScheme: {SigScheme}, encryptScheme: ({EncryptScheme[0]}, {EncryptScheme[1]})}}";
    }

    public readonly struct LoggedReceipt
    {
        public string Principal { get; init; }
        public DateTime Deadline { get; init; }
        public LoggedType LoggedType { get; init; }

        public override string ToString() =>
            $"{{principal: {Principal}, deadline: {Deadline}, loggedType: {LoggedType}}}";
    }

    public enum LoggedType
    {
        II = 0,
        Host = 1,
        Ext = 2,
    }

    public class KeyStore {
        

        public KeyStore(string name, string password) {
            todo!()
        }
    }
}