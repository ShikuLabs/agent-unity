using System.IO;
using IC;
using UnityEngine;

namespace Example._01_code_snippet
{
    public class CodeSnippet : MonoBehaviour
    {
        // Start is called before the first frame update
        void Start()
        {
            // 1. Create keystore;
            // 2. Login by keystore;
            // 3. Get login info by principal;
            // 4. List all login info;
            LoginFlow();
            
            // 
            CallFlow();
        }

        void LoginFlow()
        {
            var keyStore1 = Agent.CreateKeyStore("Allen Pocket", "123456");
            Debug.Log($"[x] Create first keystore: {keyStore1}\n");
            
            var keyStore2 = Agent.CreateKeyStore("Allen Pocket", "123456");
            Debug.Log($"[x] Create second keystore: {keyStore2}\n");
            
            var receipt1 = Agent.LoginByHost(keyStore1, "123456");
            Debug.Log($"[x] Login by first keystore: {receipt1}\n");
            
            var receipt2 = Agent.LoginByHost(keyStore2, "123456");
            Debug.Log($"[x] Login by second keystore: {receipt2}\n");
            
            receipt1 = Agent.GetLoggedReceipt(receipt1.Principal);
            Debug.Log($"[x] Get first login info: {receipt1}\n");

            receipt2 = Agent.GetLoggedReceipt(receipt2.Principal);
            Debug.Log($"[x] Get second login info: {receipt2}\n");

            var receipts = Agent.ListLoggedReceipt();
            Debug.Log("[x] List login infos:");
            for (int i = 0; i < receipts.Length; i++)
            {
                Debug.Log($"    {i}. {receipts[i]}");
            }

            Agent.Logout(receipt1.Principal);
            Debug.Log($"[x] Logout first keystore: {receipt1}\n");

            Agent.Logout(receipt2.Principal);
            Debug.Log($"[x] Logout second keystore: {receipt2}\n");
        }

        void CallFlow()
        {
            const string II_CANISTER_ID = "rdmx6-jaaaa-aaaaa-aaadq-cai";
            var II_IDL_PATH = Path.Join(Application.dataPath, $"Example/01_code_snippet/{II_CANISTER_ID}.did");
            var II_IDL_CONTENT = System.IO.File.ReadAllText(II_IDL_PATH);

            var idlContent = Agent.RemoveIdl(II_CANISTER_ID);
            Debug.Log($"[x] Remove IDL Content: {idlContent}");
            
            Agent.RegisterIdl(II_CANISTER_ID, II_IDL_CONTENT);
            Debug.Log("[x] Register II IDL File");

            var canisterIds = Agent.ListIdl();
            Debug.Log("List IDL canisterIds:");
            foreach (var canisterId in canisterIds)
            {
                Debug.Log($"    [x] {canisterId}");
            }

            idlContent = Agent.GetIdl(II_CANISTER_ID);
            Debug.Log($"[x] Get IDL Content: {idlContent}");
        }
    }
}