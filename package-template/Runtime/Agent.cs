using System;
using System.Runtime.InteropServices;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using UnityEngine;

namespace IC {
    public static class Agent
    {
        [DllImport("ic-agent")]
        private static extern Response create_keystore([MarshalAs(UnmanagedType.LPStr)] string req);

        [DllImport("ic-agent")]
        private static extern void free_rsp(Response rsp);

        [DllImport("ic-agent")]
        private static extern Response login_by_host([MarshalAs(UnmanagedType.LPStr)] string req);

        [DllImport("ic-agent")]
        private static extern Response get_logged_receipt([MarshalAs(UnmanagedType.LPStr)] string req);

        [DllImport("ic-agent")]
        private static extern Response list_logged_receipt();

        [DllImport("ic-agent")]
        private static extern Response logout([MarshalAs(UnmanagedType.LPStr)] string req);

        [DllImport("ic-agent")]
        private static extern Response ic_register_idl([MarshalAs(UnmanagedType.LPStr)] string canisterId, [MarshalAs(UnmanagedType.LPStr)] string candidFile);

        [DllImport("ic-agent")]
        private static extern Response ic_remove_idl([MarshalAs(UnmanagedType.LPStr)] string canisterId);
        
        [DllImport("ic-agent")]
        private static extern Response ic_get_idl([MarshalAs(UnmanagedType.LPStr)] string canisterId);

        [DllImport("ic-agent")]
        private static extern Response ic_list_idl();
        
        [DllImport("ic-agent")]
        private static extern Response ic_query_sync(
            [MarshalAs(UnmanagedType.LPStr)] string caller,
            [MarshalAs(UnmanagedType.LPStr)] string canisterId,
            [MarshalAs(UnmanagedType.LPStr)] string methodName,
            [MarshalAs(UnmanagedType.LPStr)] string argsRaw
        );
        
        
        [StructLayout(LayoutKind.Sequential)]
        private readonly struct Response
        {
            public readonly IntPtr Ptr;
            [MarshalAs(UnmanagedType.I1)]
            public readonly bool IsErr;
        }
        
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

        public static void RegisterIdl(string canisterId, string candidFile)
        {
            var rsp = ic_register_idl(canisterId, candidFile);
            
            var data = Marshal.PtrToStringAnsi(rsp.Ptr);
            free_rsp(rsp);
            
            if (data == null) throw new Exception("inner error: data from rust-lib is null");
            if (rsp.IsErr) throw new Exception(data);
        }

        public static string RemoveIdl(string canisterId)
        {
            var rsp = ic_remove_idl(canisterId);
            
            var data = Marshal.PtrToStringAnsi(rsp.Ptr);
            free_rsp(rsp);
            
            if (data == null) throw new Exception("inner error: data from rust-lib is null");
            if (rsp.IsErr) throw new Exception(data);

            if (data == "null") return null;

            return data;
        }
        
        public static string GetIdl(string canisterId)
        {
            var rsp = ic_get_idl(canisterId);
            
            var data = Marshal.PtrToStringAnsi(rsp.Ptr);
            free_rsp(rsp);
            
            if (data == null) throw new Exception("inner error: data from rust-lib is null");
            if (rsp.IsErr) throw new Exception(data);

            if (data == "null") return null;

            return data;
        }

        public static string[] ListIdl()
        {
            var rsp = ic_list_idl();
            
            var data = Marshal.PtrToStringAnsi(rsp.Ptr);
            free_rsp(rsp);
            
            if (data == null) throw new Exception("inner error: data from rust-lib is null");
            if (rsp.IsErr) throw new Exception(data);

            var principals = JsonConvert.DeserializeObject<string[]>(data);

            return principals;
        }
        
        public static string QuerySync(string caller, string canisterId, string methodName, string argsRaw)
        {
            var rsp = ic_query_sync(caller, canisterId, methodName, argsRaw);
            
            var data = Marshal.PtrToStringAnsi(rsp.Ptr);
            free_rsp(rsp);
            
            if (data == null) throw new Exception("inner error: data from rust-lib is null");
            if (rsp.IsErr) throw new Exception(data);

            return data;
        }
    }
    
    public struct HostKeyStore
    {
        [JsonProperty]
        public readonly string Encoded;
        [JsonProperty]
        public readonly string Principal;
        [JsonProperty]
        public HostKeyStoreMeta Meta { get; set; }

        public HostKeyStore(string encoded, string principal, HostKeyStoreMeta meta)
        {
            Encoded = encoded;
            Principal = principal;
            Meta = meta;
        }
        
        public override string ToString() => $"{{encoded: {Encoded}, principal: {Principal}, meta: {Meta}}}";
    }

    public struct HostKeyStoreMeta
    {
        [JsonProperty]
        public string Name { get; set; }
        [JsonProperty]
        public readonly DateTime WhenCreated;
        [JsonProperty]
        public readonly string StoreSyntax;
        [JsonProperty]
        public readonly string SigScheme;
        [JsonProperty]
        public readonly string[] EncryptScheme;

        public HostKeyStoreMeta(string name, DateTime whenCreated, string storeSyntax, string sigScheme,
            string[] encryptScheme)
        {
            Name = name;
            WhenCreated = whenCreated;
            StoreSyntax = storeSyntax;
            SigScheme = sigScheme;
            EncryptScheme = encryptScheme;
        }
        
        public override string ToString() =>
            $"{{name: {Name}, whenCreated: {WhenCreated}, storeSyntax: {StoreSyntax}, sigScheme: {SigScheme}, encryptScheme: ({EncryptScheme[0]}, {EncryptScheme[1]})}}";
    }

    public readonly struct LoggedReceipt
    {
        [JsonProperty]
        public readonly string Principal;
        [JsonProperty]
        public readonly DateTime Deadline;
        [JsonProperty]
        public readonly LoggedType LoggedType;

        public LoggedReceipt(string principal, DateTime deadline, LoggedType loggedType)
        {
            Principal = principal;
            Deadline = deadline;
            LoggedType = loggedType;
        }
        
        public override string ToString() =>
            $"{{principal: {Principal}, deadline: {Deadline}, loggedType: {LoggedType}}}";
    }

    public enum LoggedType
    {
        II = 0,
        Host = 1,
        Ext = 2,
    }
}