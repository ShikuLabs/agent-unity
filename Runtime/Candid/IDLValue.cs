using System;
using System.Linq;
using System.Collections.Generic;
using System.Numerics;
using System.Runtime.InteropServices;

namespace Candid
{
#nullable enable
public class IDLValue : IEquatable<IDLValue>
{
    internal IntPtr _ptr;

    internal IDLValue(IntPtr ptr)
    {
        _ptr = ptr;
    }

    public static IDLValue FromText(string text)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_from_text(text, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLValue WithBool(bool value)
    {
        FromRust.idl_value_ct_bool(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithNull()
    {
        FromRust.idl_value_ct_null(out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithText(string text)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.idl_value_ct_text(text, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLValue WithNumber(string number)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.idl_value_ct_number(number, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLValue WithDouble(double value)
    {
        FromRust.idl_value_ct_float64(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithOpt(IDLValue value)
    {
        FromRust.idl_value_ct_opt(value._ptr, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithVec(IDLValue[] values)
    {
        var ptrs = values.Select(value => value._ptr).ToArray();

        FromRust.idl_value_ct_vec(ptrs, ptrs.Length, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithRecord(Dictionary<String, IDLValue> records)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        string[] keys = records.Keys.ToArray();
        IntPtr[] vals = records.Values.Select(value => value._ptr).ToArray();

        var sc = FromRust.idl_value_ct_record(keys, keys.Length, vals, vals.Length, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLValue WithVariant(string key, IDLValue value, UInt64 index)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.idl_value_ct_variant(key, value._ptr, index, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLValue WithPrincipal(Principal principal)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.idl_value_ct_principal(principal.Bytes, principal.Bytes.Length, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLValue WithService(Principal principal)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.idl_value_ct_service(principal.Bytes, principal.Bytes.Length, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLValue WithFunc(Principal principal, string funcName)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.idl_value_ct_func(principal.Bytes, principal.Bytes.Length, funcName, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLValue WithNone()
    {
        FromRust.idl_value_ct_none(out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithInt(BigInteger bi)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.idl_value_ct_int(bi.ToString(), out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLValue WithNat(BigInteger bi)
    {
        if (bi < BigInteger.Zero)
            throw new Exception("Nat must be bigger than 0");

        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.idl_value_ct_nat(bi.ToString(), out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLValue WithNat8(byte value)
    {
        FromRust.idl_value_ct_nat8(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithNat16(UInt16 value)
    {
        FromRust.idl_value_ct_nat16(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithNat32(UInt32 value)
    {
        FromRust.idl_value_ct_nat32(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithNat64(UInt64 value)
    {
        FromRust.idl_value_ct_nat64(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithInt8(sbyte value)
    {
        FromRust.idl_value_ct_int8(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithInt16(Int16 value)
    {
        FromRust.idl_value_ct_int16(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithInt32(Int32 value)
    {
        FromRust.idl_value_ct_int32(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithInt64(Int64 value)
    {
        FromRust.idl_value_ct_int64(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithFloat(float value)
    {
        FromRust.idl_value_ct_float32(value, out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public static IDLValue WithReserved()
    {
        FromRust.idl_value_ct_reserved(out IntPtr ptr);

        return new IDLValue(ptr);
    }

    public bool Equals(IDLValue? idlValue)
    {
        if (idlValue == null) return false;

        return FromRust.idl_value_equal(_ptr, idlValue._ptr);
    }

    public override bool Equals(object? obj) => Equals(obj as IDLValue);

    public string GetValueType()
    {
        string? outTexts = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outTexts = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_type(_ptr, retCb);

        if (outTexts == null)
            throw new FailedCallingRust("Failed on calling function of rust.");
        else
            return outTexts;
    }

    public bool AsBool()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_bool(_ptr, out Boolean value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public bool IsNull()
    {
        UnsizedCallback errCb = (data, len) => { };
        var sc = FromRust.idl_value_is_null(_ptr, errCb);

        return sc == StateCode.Ok;
    }

    public string AsText()
    {
        string? outTexts = null;
        string? outError = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outTexts = Marshal.PtrToStringAnsi(data);
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_text(_ptr, retCb, errCb);

        if (sc == StateCode.Ok)
        {
            if (outTexts == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return outTexts;
        }
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public string AsNumber()
    {
        string? outNumber = null;
        string? outError = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outNumber = Marshal.PtrToStringAnsi(data);
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_number(_ptr, retCb, errCb);

        if (sc == StateCode.Ok)
        {
            if (outNumber == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return outNumber;
        }
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public float AsFloat()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_float32(_ptr, out float value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public double AsDouble()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_float64(_ptr, out double value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public IDLValue AsOpt()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_opt(_ptr, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLValue(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public IDLValue[] AsVec()
    {
        IDLValue[]? outValues = null;
        string? outError = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outValues = new IDLValue[len];

            IntPtr[] outValuePtrs = new IntPtr[len];
            Marshal.Copy(data, outValuePtrs, 0, len);

            for (int i = 0; i < len; i++)
            {
                var valuePtr = outValuePtrs[i];
                outValues[i] = new IDLValue(valuePtr);
            }
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_vec(_ptr, retCb, errCb);

        if (sc == StateCode.Ok)
        {
            if (outValues == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return outValues;
        }
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public Dictionary<String, IDLValue> AsRecord()
    {
        Dictionary<String, IDLValue> records = new Dictionary<string, IDLValue>();
        String[]? keys = null;
        IDLValue[]? vals = null;
        string? outError = null;

        UnsizedCallback keyCb = (data, len) =>
        {
            keys = new string[len];

            IntPtr[] keyPtrs = new IntPtr[len];
            Marshal.Copy(data, keyPtrs, 0, len);

            for (int i = 0; i < len; i++)
            {
                var keyPtr = keyPtrs[i];
                var str = Marshal.PtrToStringAnsi(keyPtr);
                if (str == null)
                    throw new FailedCallingRust("Failed on calling function of rust.");
                else
                    keys[i] = str;
            }
        };
        UnsizedCallback valCb = (data, len) =>
        {
            vals = new IDLValue[len];

            IntPtr[] valPtrs = new IntPtr[len];
            Marshal.Copy(data, valPtrs, 0, len);

            for (int i = 0; i < len; i++)
            {
                var valuePtr = valPtrs[i];
                vals[i] = new IDLValue(valuePtr);
            }
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_record(_ptr, keyCb, valCb, errCb);

        if (sc == StateCode.Ok)
        {
            if (keys == null || vals == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
            {
                for (int i = 0; i < keys.Length; i++)
                {
                    records.Add(keys[i], vals[i]);
                }

                return records;
            }
        }
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public (string, IDLValue) AsVariant()
    {
        string? outId = null;
        string? outError = null;

        UnsizedCallback idCb = (data, len) =>
        {
            outId = Marshal.PtrToStringAnsi(data);
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_variant(_ptr, idCb, out IntPtr ptr, out UInt64 code, errCb);

        if (sc == StateCode.Ok)
        {
            if (outId == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return (outId, new IDLValue(ptr));
        }
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public Principal AsPrincipal()
    {
        byte[]? outBytes = null;
        string? outError = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outBytes = new byte[len];
            Marshal.Copy(data, outBytes, 0, len);
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_principal(_ptr, retCb, errCb);

        if (sc == StateCode.Ok)
            if (outBytes == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return new Principal(outBytes);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public Principal AsService()
    {
        byte[]? outBytes = null;
        string? outError = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outBytes = new byte[len];
            Marshal.Copy(data, outBytes, 0, len);
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_service(_ptr, retCb, errCb);

        if (sc == StateCode.Ok)
            if (outBytes == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return new Principal(outBytes);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public (Principal, String) AsFunc()
    {
        byte[]? outBytes = null;
        string? outTexts = null;
        string? outError = null;

        UnsizedCallback retCb01 = (data, len) =>
        {
            outBytes = new byte[len];
            Marshal.Copy(data, outBytes, 0, len);
        };
        UnsizedCallback retCb02 = (data, len) =>
        {
            outTexts = Marshal.PtrToStringAnsi(data);
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_func(_ptr, retCb01, retCb02, errCb);

        if (sc == StateCode.Ok)
            if (outBytes == null || outTexts == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return (new Principal(outBytes), outTexts);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public bool IsNone()
    {
        UnsizedCallback errCb = (data, len) => { };
        var sc = FromRust.idl_value_is_none(_ptr, errCb);

        return sc == StateCode.Ok;
    }

    public BigInteger AsInt()
    {
        string? outNat = null;
        string? outError = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outNat = Marshal.PtrToStringAnsi(data);
            outNat = outNat?.Replace("_", "");
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_int(_ptr, retCb, errCb);

        if (sc == StateCode.Ok)
            if (outNat == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return BigInteger.Parse(outNat);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public BigInteger AsNat()
    {
        string? outNat = null;
        string? outError = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outNat = Marshal.PtrToStringAnsi(data);
            outNat = outNat?.Replace("_", "");
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_nat(_ptr, retCb, errCb);

        if (sc == StateCode.Ok)
            if (outNat == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return BigInteger.Parse(outNat);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public byte AsNat8()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_nat8(_ptr, out byte value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public UInt16 AsNat16()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_nat16(_ptr, out UInt16 value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public UInt32 AsNat32()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_nat32(_ptr, out UInt32 value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public UInt64 AsNat64()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_nat64(_ptr, out UInt64 value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public sbyte AsInt8()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_int8(_ptr, out sbyte value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public Int16 AsInt16()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_int16(_ptr, out Int16 value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public Int32 AsInt32()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_int32(_ptr, out Int32 value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public Int64 AsInt64()
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_as_int64(_ptr, out Int64 value, errCb);

        if (sc == StateCode.Ok)
            return value;
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public bool IsReserved()
    {
        UnsizedCallback errCb = (data, len) => { };
        var sc = FromRust.idl_value_is_reserved(_ptr, errCb);

        return sc == StateCode.Ok;
    }

    public override string ToString()
    {
        string? outTexts = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outTexts = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_value_to_text(_ptr, retCb);

        if (outTexts == null)
            throw new FailedCallingRust("Failed on calling function of rust.");
        else
            return outTexts;
    }

    ~IDLValue()
    {
        FromRust.idl_value_free(_ptr);
    }

    internal static class FromRust
    {
        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_to_text(
            IntPtr ptr2Value,
            UnsizedCallback retCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_from_text(
            [MarshalAs(UnmanagedType.LPStr)] string text,
            out IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_bool(
            [MarshalAs(UnmanagedType.Bool)] bool value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_null(
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_ct_text(
            [MarshalAs(UnmanagedType.LPStr)] string text,
            out IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_ct_number(
            [MarshalAs(UnmanagedType.LPStr)] string number,
            out IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_float64(
            double value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_opt(
            IntPtr value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_vec(
            IntPtr[] p2ArrPtr,
            Int32 arrLen,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_ct_record(
            [MarshalAs(UnmanagedType.LPArray)] string[] keys,
            Int32 keysLen,
            IntPtr[] vals,
            Int32 valsLen,
            out IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_ct_variant(
            [MarshalAs(UnmanagedType.LPStr)] string key,
            IntPtr valPtr,
            UInt64 index,
            out IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_ct_principal(
            byte[] bytes,
            Int32 bytesLen,
            out IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_ct_service(
            byte[] bytes,
            Int32 bytesLen,
            out IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_ct_func(
            byte[] bytes,
            Int32 bytesLen,
            [MarshalAs(UnmanagedType.LPStr)] string funcName,
            out IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_none(
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_ct_int(
            [MarshalAs(UnmanagedType.LPStr)] string value,
            out IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_ct_nat(
            [MarshalAs(UnmanagedType.LPStr)] string value,
            out IntPtr ptr2Value,
            UnsizedCallback errCb
        );


        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_nat8(
            byte value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_nat16(
            UInt16 value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_nat32(
            UInt32 value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_nat64(
            UInt64 value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_int8(
            sbyte value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_int16(
            Int16 value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_int32(
            Int32 value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_int64(
            Int64 value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_float32(
            float value,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_ct_reserved(
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_type(
            IntPtr ptr2Value,
            UnsizedCallback retCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_bool(
            IntPtr ptr2Value,
            out Boolean value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.Bool)]
        internal static extern bool idl_value_equal(
            IntPtr ptr01,
            IntPtr ptr02
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_is_null(
            IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_text(
            IntPtr ptr2Value,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_number(
            IntPtr ptr2Value,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_float64(
            IntPtr ptr2Value,
            out Double value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_opt(
            IntPtr ptr2Value,
            out IntPtr ptr2Opt,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_vec(
            IntPtr ptr2Value,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_record(
            IntPtr ptr2Value,
            UnsizedCallback retCb01,
            UnsizedCallback retCb02,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_variant(
            IntPtr ptr2Value,
            UnsizedCallback idCb,
            out IntPtr ptr2Val,
            out UInt64 code,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_principal(
            IntPtr ptr2Value,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_service(
            IntPtr ptr2Value,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_func(
            IntPtr ptr2Value,
            UnsizedCallback retCb01,
            UnsizedCallback retCb02,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_is_none(
            IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_int(
            IntPtr ptr2Value,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_nat(
            IntPtr ptr2Value,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_nat8(
            IntPtr ptr2Value,
            out Byte value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_nat16(
            IntPtr ptr2Value,
            out UInt16 value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_nat32(
            IntPtr ptr2Value,
            out UInt32 value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_nat64(
            IntPtr ptr2Value,
            out UInt64 value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_int8(
            IntPtr ptr2Value,
            out SByte value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_int16(
            IntPtr ptr2Value,
            out Int16 value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_int32(
            IntPtr ptr2Value,
            out Int32 value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_int64(
            IntPtr ptr2Value,
            out Int64 value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_as_float32(
            IntPtr ptr2Value,
            out float value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_value_is_reserved(
            IntPtr ptr2Value,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_value_free(IntPtr ptr2Value);
    }

    public override int GetHashCode()
    {
        throw new NotImplementedException();
    }
}
#nullable disable
}