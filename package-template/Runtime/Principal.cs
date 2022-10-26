using System;
using System.Linq;
using System.Numerics;
using System.Runtime.InteropServices;

#nullable enable
public class Principal : IEquatable<Principal>
{
    public byte[] Bytes { get; }

    internal Principal(byte[] data)
    {
        Bytes = data;
    }

    public static Principal ManagementCanister()
    {
        byte[]? outBytes = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outBytes = new byte[len];
            Marshal.Copy(data, outBytes, 0, len);
        };
        FromRust.principal_management_canister(retCb);

        if (outBytes == null)
            throw new FailedCallingRust("Failed on calling function of rust.");

        return new Principal(outBytes);
    }

    public static Principal SelfAuthenticating(byte[] publicKey)
    {
        byte[]? outBytes = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outBytes = new byte[len];
            Marshal.Copy(data, outBytes, 0, len);
        };
        FromRust.principal_self_authenticating(publicKey, publicKey.Length, retCb);

        if (outBytes == null)
            throw new FailedCallingRust("Failed on calling function of rust.");

        return new Principal(outBytes);
    }

    public static Principal Anonymous()
    {
        byte[]? outBytes = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outBytes = new byte[len];
            Marshal.Copy(data, outBytes, 0, len);
        };
        FromRust.principal_anonymous(retCb);

        if (outBytes == null)
            throw new FailedCallingRust("Failed on calling function of rust.");

        return new Principal(outBytes);
    }

    public static Principal FromBytes(byte[] bytes)
    {
        byte[]? outBytes = null;
        string? outError = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outBytes = new byte[len];
            Marshal.Copy(data, outBytes, 0, len);
        };
        UnsizedCallback errCb = (data, len) => { outError = Marshal.PtrToStringAnsi(data); };
        var sc = FromRust.principal_from_bytes(bytes, bytes.Length, retCb, errCb);

        if (sc == StateCode.Ok)
        {
            if (outBytes == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return new Principal(outBytes);
        }
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static Principal FromText(string text)
    {
        byte[]? outBytes = null;
        string? outError = null;

        UnsizedCallback retCb = (data, len) =>
        {
            outBytes = new byte[len];
            Marshal.Copy(data, outBytes, 0, len);
        };
        UnsizedCallback errCb = (data, len) => { outError = Marshal.PtrToStringAnsi(data); };
        var sc = FromRust.principal_from_text(text, retCb, errCb);

        if (sc == StateCode.Ok)
        {
            if (outBytes == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return new Principal(outBytes);
        }
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public override string ToString()
    {
        string? outTexts = null;
        string? outError = null;

        UnsizedCallback retCb = (data, len) => { outTexts = Marshal.PtrToStringAnsi(data); };
        UnsizedCallback errCb = (data, len) => { outError = Marshal.PtrToStringAnsi(data); };
        var sc = FromRust.principal_to_text(Bytes, Bytes.Length, retCb, errCb);

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

    public bool Equals(Principal? principal)
    {
        if (principal == null) return false;

        if (GetType() != principal.GetType()) return false;

        if (ReferenceEquals(this, principal)) return true;

        return Enumerable.SequenceEqual(Bytes, principal.Bytes);
    }

    public override bool Equals(object? obj) => Equals(obj as Principal);

    public override int GetHashCode()
    {
        return new BigInteger(Bytes).GetHashCode();
    }

    internal static class FromRust
    {
        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void principal_management_canister(
            [MarshalAs(UnmanagedType.FunctionPtr)] UnsizedCallback retCb
        );

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void principal_self_authenticating(
            byte[] publicKey,
            Int32 publicKeyLen,
            [MarshalAs(UnmanagedType.FunctionPtr)] UnsizedCallback retCb
        );

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode principal_anonymous(
            [MarshalAs(UnmanagedType.FunctionPtr)] UnsizedCallback retCb
        );

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode principal_from_bytes(
            byte[] bytes,
            Int32 bytesLen,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode principal_from_text(
            [MarshalAs(UnmanagedType.LPStr)] string text,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode principal_to_text(
            byte[] bytes,
            Int32 bytesLen,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );
    }
}
#nullable disable