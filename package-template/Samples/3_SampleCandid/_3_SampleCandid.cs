using System;
using System.IO;
using Candid;
using UnityEditor;
using UnityEngine;
using UnityEngine.Assertions;

namespace _3_SampleCandid
{
    public class _3_SampleCandid : MonoBehaviour
    {
        void Start()
        {
            // Create an [`IDLValue`] with bool
            var valueBool = IDLValue.WithBool(true);
            // Get the real value from [`IDLValue`]
            Assert.AreEqual(true, valueBool.AsBool());
            // Or Try to display the [`IDLValue`]
            Debug.Log($"{valueBool}");
            
            // With [`Principal`] is ok
            var valuePrincipal = IDLValue.WithPrincipal(Principal.Anonymous());
            // Compare
            Assert.AreEqual(Principal.Anonymous(), valuePrincipal.AsPrincipal());
            // Again, display it
            Debug.Log($"{valuePrincipal}");
            
            // It's easy, right?
            // Now, we do some little more complicated thing, create an IDLValue::Vec;
            var values = new[]
            {
                IDLValue.WithBool(true),
                IDLValue.WithPrincipal(Principal.Anonymous()),
                IDLValue.WithNat8(128),
                IDLValue.WithInt32(-129)
            };
            var valueVec = IDLValue.WithVec(values);
            // Now, display it
            Debug.Log($"{valueVec}");
            // Of couse, you can unwrap the vec value
            var lists = valueVec.AsVec();
            for (int i = 0; i < values.Length; i++)
            {
                Assert.AreEqual(values[i], lists[i]);
            }
            
            // [`IDLArgs`] actually is a vec of [`IDLValue`],
            // it's easy to create, like previous code
            var args = IDLArgs.WithVec(values);
            // Display it
            Debug.Log($"{args}");
            // NOTE: `query` & `update` only return [`IDLArgs`] rather than [`IDLValue`],
            // [`IDLArgs`] is the smallest unit transmitting in ic network
        }
    }
}