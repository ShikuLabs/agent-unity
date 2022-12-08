

#nullable enable
using System;
using System.Runtime.InteropServices;
using Candid;

public class Agent
{
    private IntPtr _ptr;

    private Agent(IntPtr ptr)
    {
        _ptr = ptr;
    }

    ~Agent()
    {
        FromRust.agent_free(_ptr);
    }

    public static Agent Create(
        string url,
        Identity identity,
        Principal canisterId,
        string didContent
    )
    {
        string? outError = null;
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.agent_create(
            url,
            identity._p2FPtr,
            identity.Type,
            canisterId.Bytes,
            canisterId.Bytes.Length,
            didContent,
            out IntPtr ptr,
            errCb
        );

        if (sc == StateCode.Ok)
            return new Agent(ptr);
        else
        {
            if (outError == null)
                throw new FailedCallingRust("Failed on getting error from rust.");
            else
                throw new ErrorFromRust(outError);
        }
    }

    public IDLArgs Query(string funcName, string funcArgs)
    {
        string? outError = null;
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.agent_query(
            this._ptr,
            funcName,
            funcArgs,
            out IntPtr ptr,
            errCb
        );

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

    public IDLArgs Update(string funcName, string funcArgs)
    {
        string? outError = null;
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.agent_update(
            this._ptr,
            funcName,
            funcArgs,
            out IntPtr ptr,
            errCb
        );

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
    
    public string Status()
    {
        string? outIdlArgs = null;
        string? outError = null;
        UnsizedCallback retCb = (data, len) =>
        {
            outIdlArgs = Marshal.PtrToStringAnsi(data);
        };
        UnsizedCallback errCb = (data, len) =>
        {
            outError = Marshal.PtrToStringAnsi(data);
        };

        var sc = FromRust.agent_status(
            this._ptr,
            retCb,
            errCb
        );

        if (sc == StateCode.Ok)
        {
            if (outIdlArgs == null)
                throw new FailedCallingRust("Failed on calling function of rust.");
            else
                return outIdlArgs;
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
        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode agent_create(
            [MarshalAs(UnmanagedType.LPStr)] string url,
            IntPtr[] fptr2Identity,
            IdentityType identityType,
            byte[] canisterIdBytes,
            Int32 canisterIdBytesLen,
            [MarshalAs(UnmanagedType.LPStr)] string didContent,
            out IntPtr ptr2Agent,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode agent_query(
            IntPtr ptr2Agent,
            [MarshalAs(UnmanagedType.LPStr)] string funcName,
            [MarshalAs(UnmanagedType.LPStr)] string funcArgs,
            out IntPtr ptr2Args,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode agent_update(
            IntPtr ptr2Agent,
            [MarshalAs(UnmanagedType.LPStr)] string funcName,
            [MarshalAs(UnmanagedType.LPStr)] string funcArgs,
            out IntPtr ptr2Args,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode agent_status(
            IntPtr ptr2Agent,
            UnsizedCallback retCb,
            UnsizedCallback errCb
        );

        [DllImport("ic-agent", CallingConvention = CallingConvention.Cdecl)]
        internal static extern StateCode agent_free(
            IntPtr ptr2Agent
        );
    }
}
#nullable disable