using System;
using System.Runtime.InteropServices;

public class Identity
{
    internal IntPtr[] _p2FPtr;

    public IdentityType Type { get; }

    private Identity(IntPtr[] p2FPtr, IdentityType type)
    {
        this._p2FPtr = p2FPtr;
        this.Type = type;
    }

    ~Identity()
    {
        FromRust.identity_free(_p2FPtr);
    }

    /// <summary>
    /// Create an [`Identity`] with `Anonymous` type.
    /// </summary>
    public static Identity Anonymous()
    {
        var p2FPtr = new IntPtr[2];
        FromRust.identity_anonymous(p2FPtr);

        return new Identity(p2FPtr, IdentityType.Anonymous);
    }

    /// <summary>
    /// Create an [`Identity`] with `Basic` type.
    /// </summary>
    public static Identity BasicRandom()
    {
        string? outError = null;
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var p2FPtr = new IntPtr[2];

        var sc = FromRust.identity_basic_random(p2FPtr, errCb);

        if (sc == StateCode.Ok)
            return new Identity(p2FPtr, IdentityType.Basic);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static Identity BasicFromPem(string pem)
    {
        string? outError = null;
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var p2FPtr = new IntPtr[2];

        var sc = FromRust.identity_basic_from_pem(pem, p2FPtr, errCb);

        if (sc == StateCode.Ok)
            return new Identity(p2FPtr, IdentityType.Basic);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    /// <summary>
    /// Create an [`Identity`] with `Secp256k1` type.
    /// </summary>
    public static Identity Secp256K1Random()
    {
        var p2FPtr = new IntPtr[2];
        FromRust.identity_secp256k1_random(p2FPtr);

        return new Identity(p2FPtr, IdentityType.Secp256K1);
    }

    public static Identity Secp256K1FromPem(string pem)
    {
        string? outError = null;
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var p2FPtr = new IntPtr[2];

        var sc = FromRust.identity_secp256k1_from_pem(pem, p2FPtr, errCb);

        if (sc == StateCode.Ok)
            return new Identity(p2FPtr, IdentityType.Secp256K1);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public Principal Sender()
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

        var sc = FromRust.identity_sender(_p2FPtr, retCb, errCb);

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

    public (byte[], byte[]) Sign(byte[] bytes)
    {
        byte[]? publicKey = null;
        byte[]? signature = null;
        string? outError = null;

        UnsizedCallback pubKeyCb = (data, len) =>
        {
            publicKey = new byte[len];
            Marshal.Copy(data, publicKey, 0, len);
        };
        UnsizedCallback sigCb = (data, len) =>
        {
            signature = new byte[len];
            Marshal.Copy(data, signature, 0, len);
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.identity_sign(bytes, bytes.Length, _p2FPtr, pubKeyCb, sigCb, errCb);

        if (sc == StateCode.Ok)
        {
            if (publicKey == null || signature == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return (publicKey, signature);
        }
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    internal static class FromRust
    {
        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void identity_anonymous(IntPtr[] p2FPtr);

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode identity_basic_random(
            IntPtr[] p2FPtr,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode identity_basic_from_pem(
            [MarshalAs(UnmanagedType.LPStr)] string pem,
            IntPtr[] p2FPtr,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void identity_secp256k1_random(IntPtr[] p2FPtr);

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode identity_secp256k1_from_pem(
            [MarshalAs(UnmanagedType.LPStr)] string pem,
            IntPtr[] p2FPtr,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode identity_sender(
            IntPtr[] p2FPtr,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode identity_sign(
            byte[] bytes,
            Int32 bytesLen,
            IntPtr[] p2FPtr,
            UnsizedCallback pubKeyCb,
            UnsizedCallback sigCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent-ffi", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode identity_free(IntPtr[] p2FPtr);
    }
}

public enum IdentityType
{
    Anonymous = 0,
    Basic = 1,
    Secp256K1 = 2,
}