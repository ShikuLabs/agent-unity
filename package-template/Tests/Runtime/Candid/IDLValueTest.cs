using System;
using System.Numerics;
using NUnit.Framework;
using Candid;

namespace Tests.Runtime.Candid
{
    public class IDLValueTest
    {
        [Test]
        public void FromText_ShouldWork()
        {
            IDLValue.FromText("128: nat64");
            IDLValue.FromText("(128: nat64)");
            IDLValue.FromText("principal \"2vxsx-fae\"");
            IDLValue.FromText("(principal \"2vxsx-fae\")");
        }

        [Test]
        public void ToString_ShouldWork()
        {
            var v01 = IDLValue.FromText("128: nat64");
            Assert.AreEqual("128 : nat64", v01.ToString());

            var v02 = IDLValue.FromText("(128: nat64)");
            Assert.AreEqual("128 : nat64", v02.ToString());

            var v03 = IDLValue.FromText("principal \"2vxsx-fae\"");
            Assert.AreEqual("principal \"2vxsx-fae\"", v03.ToString());

            var v04 = IDLValue.FromText("(principal \"2vxsx-fae\")");
            Assert.AreEqual("principal \"2vxsx-fae\"", v04.ToString());
        }

        [Test]
        public void GetValueType_ShouldWork()
        {
            var v01 = IDLValue.FromText("128: nat64");
            Assert.AreEqual("nat64", v01.GetValueType());

            var v02 = IDLValue.FromText("principal \"2vxsx-fae\"");
            Assert.AreEqual("principal", v02.GetValueType());
        }

        [Test]
        public void AreEqual_ShouldWork()
        {
            var v01 = IDLValue.FromText("true: bool");
            var v02 = IDLValue.FromText("false: bool");
            var v03 = IDLValue.FromText("-11: int32");
            var v04 = IDLValue.FromText("true: bool");

            Assert.AreEqual(v01, v01);
            Assert.AreNotEqual(v01, v02);
            Assert.AreNotEqual(v01, v03);
            Assert.AreEqual(v01, v04);

            Assert.AreEqual(v02, v02);
            Assert.AreNotEqual(v02, v03);
            Assert.AreNotEqual(v02, v04);

            Assert.AreEqual(v03, v03);
            Assert.AreNotEqual(v03, v04);

            Assert.AreEqual(v04, v04);
        }

        [Test]
        public void AsBool_ShouldWork()
        {
            var v01 = IDLValue.FromText("true: bool");
            Assert.True(v01.AsBool());

            var v02 = IDLValue.FromText("false: bool");
            Assert.False(v02.AsBool());
        }

        [Test]
        public void IsNull_ShouldWork()
        {
            var v01 = IDLValue.FromText("null");
            Assert.True(v01.IsNull());
        }

        [Test]
        public void AsText_ShouldWork()
        {
            var actual = "Hello World";

            var v01 = IDLValue.FromText($"\"{actual}\": text");
            Assert.AreEqual(actual, v01.AsText());
        }

        [Test]
        public void AsNumber_ShouldWork()
        {
            var actual = "123456890123456890";

            var v01 = IDLValue.FromText($"{actual}");
            Assert.AreEqual(actual, v01.AsNumber());
        }

        [Test]
        public void AsFloat_ShouldWork()
        {
            var v01 = IDLValue.FromText("1.0: float32");
            Assert.AreEqual(1.0, v01.AsFloat());
        }

        [Test]
        public void AsDouble_ShouldWork()
        {
            var v01 = IDLValue.FromText("1.03: float64");
            Assert.AreEqual(1.03, v01.AsDouble());
        }

        [Test]
        public void AsOpt_ShouldWork()
        {
            var actual = IDLValue.FromText("principal \"2vxsx-fae\"");

            var v01 = IDLValue.FromText("opt principal \"2vxsx-fae\"");

            Assert.AreEqual(actual, v01.AsOpt());
        }

        [Test]
        public void AsVec_ShouldWork()
        {
            var valueVec = IDLValue.FromText("vec { true; principal \"2vxsx-fae\"; 12345 }");
            var values = valueVec.AsVec();

            Assert.True(values[0].AsBool());
            Assert.AreEqual(Principal.Anonymous(), values[1].AsPrincipal());
            Assert.AreEqual("12345", values[2].AsNumber());
        }

        [Test]
        public void AsRecord_ShouldWork()
        {
            var valueRecord =
                IDLValue.FromText("record { Key01 = true; 123 = principal \"2vxsx-fae\"; Key03 = 12345 }");
            var dict = valueRecord.AsRecord();

            Assert.True(dict["Key01"].AsBool());
            Assert.AreEqual(Principal.Anonymous(), dict["123"].AsPrincipal());
            Assert.AreEqual("12345", dict["Key03"].AsNumber());
        }

        [Test]
        public void AsVariant_ShouldWork()
        {
            var valueVariant = IDLValue.FromText("variant { Key = true }");
            var (id, value) = valueVariant.AsVariant();

            Assert.AreEqual("Key", id);
            Assert.True(value.AsBool());
        }

        [Test]
        public void AsPrincipal_ShouldWork()
        {
            var v01 = IDLValue.FromText("principal \"2vxsx-fae\"");
            Assert.AreEqual(Principal.Anonymous(), v01.AsPrincipal());
        }

        [Test]
        public void AsService_ShouldWork()
        {
            var v01 = IDLValue.FromText("service \"2vxsx-fae\"");
            Assert.AreEqual(Principal.Anonymous(), v01.AsService());
        }

        [Test]
        public void AsFunc_ShouldWork()
        {
            var valueFunc = IDLValue.FromText("func \"2vxsx-fae\".get_info");

            var (principal, funcName) = valueFunc.AsFunc();

            Assert.AreEqual(Principal.Anonymous(), principal);
            Assert.AreEqual("get_info", funcName);
        }

        // [Test]
        // public void IsNone_ShouldWork()
        // {
        //     var valueNone = IDLValue.FromText("null");
        //     Assert.True(valueNone.IsNone());
        // }

        [Test]
        public void AsInt_ShouldWork()
        {
            var num = "-12345678901234567890";
            var bi = BigInteger.Parse("-12345678901234567890");

            var value = IDLValue.FromText($"{num}: int");
            Assert.AreEqual(bi, value.AsInt());
        }

        [Test]
        public void AsNat_ShouldWork()
        {
            var num = "12345678901234567890";
            var bi = BigInteger.Parse("12345678901234567890");

            var value = IDLValue.FromText($"{num}: nat");
            Assert.AreEqual(bi, value.AsNat());
        }

        [Test]
        public void AsNat8_ShouldWork()
        {
            byte num = 8;

            var value = IDLValue.FromText($"{num}: nat8");
            Assert.AreEqual(num, value.AsNat8());
        }

        [Test]
        public void AsNat16_ShouldWork()
        {
            UInt16 num = 16;

            var value = IDLValue.FromText($"{num}: nat16");
            Assert.AreEqual(num, value.AsNat16());
        }

        [Test]
        public void AsNat32_ShouldWork()
        {
            UInt32 num = 32;

            var value = IDLValue.FromText($"{num}: nat32");
            Assert.AreEqual(num, value.AsNat32());
        }

        [Test]
        public void AsNat64_ShouldWork()
        {
            UInt64 num = 64;

            var value = IDLValue.FromText($"{num}: nat64");
            Assert.AreEqual(num, value.AsNat64());
        }

        [Test]
        public void AsInt8_ShouldWork()
        {
            sbyte num = -8;

            var value = IDLValue.FromText($"{num}: int8");
            Assert.AreEqual(num, value.AsInt8());
        }

        [Test]
        public void AsInt16_ShouldWork()
        {
            Int16 num = -16;

            var value = IDLValue.FromText($"{num}: int16");
            Assert.AreEqual(num, value.AsInt16());
        }

        [Test]
        public void AsInt32_ShouldWork()
        {
            Int32 num = -32;

            var value = IDLValue.FromText($"{num}: int32");
            Assert.AreEqual(num, value.AsInt32());
        }

        [Test]
        public void AsInt64_ShouldWork()
        {
            Int64 num = -64;

            var value = IDLValue.FromText($"{num}: int64");
            Assert.AreEqual(num, value.AsInt64());
        }

        [Test]
        public void IsReserved_ShouldWork()
        {
            var valueReserved = IDLValue.FromText("null : reserved");
            Assert.True(valueReserved.IsReserved());
        }
    }
}