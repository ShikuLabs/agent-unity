using System;
using System.IO;
using UnityEditor;
using UnityEngine;

namespace _2_SamplesAgent
{
    public class _2_SamplesAgent : MonoBehaviour
    {
        public string icNet = "https://ic0.app";
        private Principal iiCanisterId;
        private string iiCandidCont;
        private Agent agent;

        public static string RootDir
        {
            get
            {
                var g = AssetDatabase.FindAssets($"t:Script {nameof(_2_SamplesAgent)}");
                var path = AssetDatabase.GUIDToAssetPath(g[0]);

                return Path.GetDirectoryName(path);
            }
        }

        void Awake()
        {
            iiCanisterId = Principal.FromText("rdmx6-jaaaa-aaaaa-aaadq-cai");
            iiCandidCont = File.ReadAllText(RootDir + "/rdmx6-jaaaa-aaaaa-aaadq-cai.did");
        }

        void Start()
        {
            InitAgent();
            AgentQuery();
            AgentUpdate();
            AgentStatus();
        }

        void InitAgent()
        {
            var secp256K1Pem = File.ReadAllText(RootDir + "/secp256k1.pem");
            var identity = Identity.Secp256K1FromPem(secp256K1Pem);
            agent = Agent.Create(icNet, identity, iiCanisterId, iiCandidCont);
        }

    void AgentQuery()
        {
            var queryRst = agent.Query("lookup", "(1974211: nat64)");
            Debug.Log($"query result: {queryRst}");
        }

        void AgentUpdate()
        {
            var updateRst = agent.Update("create_challenge", "()");
            Debug.Log($"update result: {updateRst}");
        }

        void AgentStatus()
        {
            var status = agent.Status();
            Debug.Log($"status: {status}");
        }
    }
}