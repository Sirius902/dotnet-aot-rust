using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace DotnetLib;

internal static class Library
{
    [UnmanagedCallersOnly(EntryPoint = "dotnet_lib_add", CallConvs = [typeof(CallConvCdecl)])]
    public static int Add(int a, int b)
    {
        return a + b;
    }
}
