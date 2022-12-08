using System;
using System.Linq;
using System.Runtime.InteropServices;

namespace Candid
{
#nullable enable
public class IDLArgs
{
    private IntPtr _ptr;

    private IDLArgs(IntPtr ptr)
    {
        _ptr = ptr;
    }

    ~IDLArgs()
    {
        FromRust.idl_args_free(_ptr);
    }

    public static IDLArgs FromText(string text)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_args_from_text(text, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLArgs(ptr);
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

        UnsizedCallback retCb = (data, len) =>
        {
            outTexts = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_args_to_text(_ptr, retCb);

        if (outTexts == null)
            throw new FailedCallingRust("Failed on calling function of rust.");
        else
            return outTexts;
    }

    public static IDLArgs FromBytes(byte[] bytes)
    {
        string? outError = null;

        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };
        var sc = FromRust.idl_args_from_bytes(bytes, bytes.Length, out IntPtr ptr, errCb);

        if (sc == StateCode.Ok)
            return new IDLArgs(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public static IDLArgs WithVec(IDLValue[] values)
    {
        var ptrs = values.Select(value => value._ptr).ToArray();

        FromRust.idl_args_ct_vec(ptrs, ptrs.Length, out IntPtr ptr);

        return new IDLArgs(ptr);
    }

    public byte[] ToBytes()
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
        var sc = FromRust.idl_args_to_bytes(_ptr, retCb, errCb);

        if (sc == StateCode.Ok)
        {
            if (outBytes == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return outBytes;
        }
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
        FromRust.idl_args_as_vec(_ptr, retCb);

        if (outValues == null)
            throw new FailedCallingRust("Failed on calling function of rust.");
        else
            return outValues;
    }

    internal static class FromRust
    {
        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_args_to_text(
            IntPtr ptr2Args,
            UnsizedCallback retCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_args_from_text(
            [MarshalAs(UnmanagedType.LPStr)] string text,
            out IntPtr ptr2Args,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_args_to_bytes(
            IntPtr ptr2Args,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode idl_args_from_bytes(
            byte[] bytes,
            Int32 bytesLen,
            out IntPtr ptr2Args,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_args_ct_vec(
            IntPtr[] p2ArrPtr,
            Int32 arrLen,
            out IntPtr ptr2Value
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_args_as_vec(
            IntPtr ptr2Value,
            UnsizedCallback retCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void idl_args_free(IntPtr ptr2Args);
    }
}
#nullable disable
}