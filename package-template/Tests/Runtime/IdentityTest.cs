using NUnit.Framework;

namespace Tests.Runtime
{
    public class IdentityTest
    {
        const string BasicIdentityFile = @"-----BEGIN PRIVATE KEY-----
MFMCAQEwBQYDK2VwBCIEIL9r4XBKsg4pquYBHY6rgfzuBsvCy89tgqDfDpofXRBP
oSMDIQBCkE1NL4X43clXS1LFauiceiiKW9NhjVTEpU6LpH9Qcw==
-----END PRIVATE KEY-----";

        const string Secp256K1IdentityFile = @"-----BEGIN EC PARAMETERS-----
BgUrgQQACg==
-----END EC PARAMETERS-----
-----BEGIN EC PRIVATE KEY-----
MHQCAQEEIAgy7nZEcVHkQ4Z1Kdqby8SwyAiyKDQmtbEHTIM+WNeBoAcGBSuBBAAK
oUQDQgAEgO87rJ1ozzdMvJyZQ+GABDqUxGLvgnAnTlcInV3NuhuPv4O3VGzMGzeB
N3d26cRxD99TPtm8uo2OuzKhSiq6EQ==
-----END EC PRIVATE KEY-----";

        static readonly byte[] PubKeyExpected =
        {
            0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00, 0x42, 0x90,
            0x4d, 0x4d, 0x2f, 0x85, 0xf8, 0xdd, 0xc9, 0x57, 0x4b, 0x52, 0xc5, 0x6a, 0xe8, 0x9c,
            0x7a, 0x28, 0x8a, 0x5b, 0xd3, 0x61, 0x8d, 0x54, 0xc4, 0xa5, 0x4e, 0x8b, 0xa4, 0x7f,
            0x50, 0x73
        };

        static readonly byte[] SignatureExpected =
        {
            0x6d, 0x7a, 0x2f, 0x85, 0xeb, 0x6c, 0xc2, 0x18, 0x80, 0xc8, 0x3d, 0x9b, 0xb1, 0x70,
            0xe2, 0x4b, 0xf5, 0xd8, 0x9a, 0xa9, 0x96, 0x92, 0xb6, 0x89, 0xac, 0x9d, 0xe9, 0x5c,
            0x1e, 0x3e, 0x50, 0xdc, 0x98, 0x12, 0x2f, 0x94, 0x11, 0x2f, 0x6c, 0xc6, 0x6a, 0x0b,
            0xbf, 0xc0, 0x56, 0x5b, 0xdb, 0x87, 0xa9, 0xe2, 0x2c, 0x8e, 0x56, 0x94, 0x56, 0x12,
            0xde, 0xbf, 0x22, 0x4a, 0x3f, 0xdb, 0xf1, 0x03
        };

        [Test]
        public void Anonymous_ShouldWork()
        {
            var identity = Identity.Anonymous();
            var principal = identity.Sender();

            Assert.AreEqual(principal, Principal.Anonymous());
        }

        [Test]
        public void BasicRandom_ShouldWork()
        {
            var identity = Identity.BasicRandom();

            Assert.AreEqual(IdentityType.Basic, identity.Type);
        }

        [Test]
        public void BasicFromPem_ShouldWork()
        {
            var identity = Identity.BasicFromPem(BasicIdentityFile);

            Assert.AreEqual(IdentityType.Basic, identity.Type);
            Assert.AreEqual(
                "emrl6-qe3wz-fh5ib-sx2r4-fbx46-6g4ql-5ro3g-zhbtm-nxdrq-q2oqo-jqe",
                identity.Sender().ToString()
            );
        }

        [Test]
        public void Secp256k1Random_ShouldWork()
        {
            var identity = Identity.Secp256K1Random();

            Assert.AreEqual(IdentityType.Secp256K1, identity.Type);
        }

        [Test]
        public void Secp256K1FromPem_ShouldWork()
        {
            var identity = Identity.Secp256K1FromPem(Secp256K1IdentityFile);

            Assert.AreEqual(IdentityType.Secp256K1, identity.Type);
            Assert.AreEqual(
                "t2kpu-6xt6l-tyb3d-rll2p-irv5c-no5nd-h6spj-jsetq-bmqdz-iap77-pqe",
                identity.Sender().ToString()
            );
        }

        [Test]
        public void Sender_ShouldWork()
        {
            var identity = Identity.Anonymous();
            var principal = identity.Sender();

            Assert.AreEqual(principal, Principal.Anonymous());
        }

        [Test]
        public void Sign_ShouldWork()
        {
            var basic = Identity.BasicFromPem(BasicIdentityFile);
            var (pubKey, signature) = basic.Sign(new byte[] { });

            Assert.AreEqual(PubKeyExpected, pubKey);
            Assert.AreEqual(SignatureExpected, signature);
        }
    }
}