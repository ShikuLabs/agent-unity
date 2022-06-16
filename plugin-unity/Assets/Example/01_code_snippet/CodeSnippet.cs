using IC;
using UnityEngine;

namespace Example._01_code_snippet
{
    public class CodeSnippet : MonoBehaviour
    {
        // Start is called before the first frame update
        void Start()
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
            Debug.Log($"[x] List login infos:");
            for (int i = 0; i < receipts.Length; i++)
            {
                Debug.Log($"    {i}. {receipts[i]}");
            }

            Agent.Logout(receipt1.Principal);
            Debug.Log($"[x] Logout first keystore: {receipt1}\n");

            Agent.Logout(receipt2.Principal);
            Debug.Log($"[x] Logout second keystore: {receipt2}\n");
        }
    }
}