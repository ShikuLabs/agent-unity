using System;

public enum StateCode
{
    Ok = 0,
    Err = -1,
}

public class FailedCallingRust : Exception
{
    public FailedCallingRust()
    {
    }

    public FailedCallingRust(string message) : base(message)
    {
    }

    public FailedCallingRust(string message, Exception inner) : base(message, inner)
    {
    }
}

public class ErrorFromRust : Exception
{
    public ErrorFromRust()
    {
    }

    public ErrorFromRust(string message) : base(message)
    {
    }

    public ErrorFromRust(string message, Exception inner) : base(message, inner)
    {
    }
}

internal delegate void UnsizedCallback(IntPtr data, Int32 len);