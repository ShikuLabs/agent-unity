using System.IO;
using UnityEditor;
using UnityEngine;

namespace _1_SamplesIdentity
{
    public class _1_SamplesIdentity : MonoBehaviour
    {
        public static string RootDir
        {
            get
            {
                var g = AssetDatabase.FindAssets ( $"t:Script {nameof(_1_SamplesIdentity)}" );
                var path = AssetDatabase.GUIDToAssetPath ( g [ 0 ] );

                return Path.GetDirectoryName(path);
            }
        }
        
        void Start()
        {
            AnonymousIdentityExample();
            BasicIdentityExample();
            Secp256K1IdentityExample();
        }

        void AnonymousIdentityExample()
        {
            // Create an [`AnonymousIdentity`]
            var identity = Identity.Anonymous();
            // Get the [`Principal`] of the `anonymousIdentity`
            var principal = identity.Sender();
            // Of course, you can get the textual representation of [`Principal`]
            Debug.Log($"anonymous: {principal}");
        }
        
        void BasicIdentityExample()
        {
            // Create an random [`BasicIdentity`] which uses ed25519 as DSA.
            var identityRandom = Identity.BasicRandom();
            Debug.Log($"basic random: {identityRandom.Sender()}");
            
            // Or Create a [`BasicIdentity`] from pem file.
            var basicPem = File.ReadAllText(RootDir + "/basic.pem");
            var identityFromPem = Identity.BasicFromPem(basicPem);
            Debug.Log($"basic pem: {identityFromPem.Sender()}");
        }
        
        void Secp256K1IdentityExample()
        {
            // Create an random [`BasicIdentity`] which uses ed25519 as DSA.
            var identityRandom = Identity.Secp256K1Random();
            Debug.Log($"secp256k1 random: {identityRandom.Sender()}");
            
            // Or Create a [`BasicIdentity`] from pem file.
            var secp256K1Pem = File.ReadAllText(RootDir + "/secp256k1.pem");
            var identityFromPem = Identity.Secp256K1FromPem(secp256K1Pem);
            Debug.Log($"secp256k1 pem: {identityFromPem.Sender()}");
        }
    }
}