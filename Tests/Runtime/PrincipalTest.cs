using System;
using NUnit.Framework;

namespace Tests.Runtime
{
    public class PrincipalTest
    {
        [Test]
        public void ManagementCanister_ShouldWork()
        {
            var bytesExpected = Array.Empty<byte>();

            var principal = Principal.ManagementCanister();

            Assert.AreEqual(principal.Bytes, bytesExpected);
        }

        [Test]
        public void SelfAuthenticating_ShouldWork()
        {
            byte[] publicKey =
            {
                0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
                0x11, 0x00, 0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44,
                0x33, 0x22, 0x11, 0x00,
            };

            byte[] bytesExpected =
            {
                0x2f, 0x8e, 0x47, 0x38, 0xf9, 0xd7, 0x68, 0x16, 0x82, 0x99, 0x85, 0x41, 0x52, 0x67,
                0x86, 0x38, 0x07, 0xd3, 0x7d, 0x20, 0x6a, 0xd9, 0x0f, 0xea, 0x72, 0xbf, 0x9d, 0xcf,
                0x02,
            };

            var principal = Principal.SelfAuthenticating(publicKey);

            Assert.AreEqual(principal.Bytes, bytesExpected);
        }

        [Test]
        public void Anonymous_ShouldWork()
        {
            byte[] bytesExpected = { 4 };

            var principal = Principal.Anonymous();

            Assert.AreEqual(principal.Bytes, bytesExpected);
        }

        [Test]
        public void FromBytes_ShouldWork()
        {
            byte[] bytes = new byte[16];

            var principal = Principal.FromBytes(bytes);

            Assert.AreEqual(principal.Bytes, bytes);
        }

        [Test]
        public void FromText_ShouldWork()
        {
            var anonymousText = new string("2vxsx-fae");
            byte[] bytesExpected = { 4 };

            var principal = Principal.FromText(anonymousText);

            Assert.AreEqual(principal.Bytes, bytesExpected);
        }

        [Test]
        public void ToString_ShouldWork()
        {
            byte[] anonymousBytes = { 4 };
            var textExpected = new string("2vxsx-fae");

            var principal = Principal.FromBytes(anonymousBytes);

            Assert.AreEqual(textExpected, principal.ToString());
        }

        [Test]
        public void Equal_ShouldWork()
        {
            var anonymous01 = Principal.Anonymous();
            var anonymous02 = Principal.Anonymous();
            Assert.AreEqual(anonymous01, anonymous02);

            var managementCanister = Principal.ManagementCanister();
            Assert.AreNotEqual(anonymous01, managementCanister);
        }
    }
}