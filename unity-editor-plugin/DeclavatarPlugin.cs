using System;
using System.Text;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace KusakaFactory.Declavatar
{
    internal static class Native
    {
        [DllImport("declavatar.dll")]
        public static extern IntPtr DeclavatarInit();

        [DllImport("declavatar.dll")]
        public static extern StatusCode DeclavatarFree(IntPtr da);

        [DllImport("declavatar.dll")]
        public static extern StatusCode DeclavatarReset(DeclavatarHandle da);

        [DllImport("declavatar.dll")]
        public static extern StatusCode DeclavatarCompile(DeclavatarHandle da, IntPtr source);

        [DllImport("declavatar.dll")]
        public static extern StatusCode DeclavatarGetAvatarJson(DeclavatarHandle da, IntPtr source);

        [DllImport("declavatar.dll")]
        public static extern StatusCode DeclavatarGetErrorsCount(DeclavatarHandle da, ref uint errors);

        [DllImport("declavatar.dll")]
        public static extern StatusCode DeclavatarGetError(DeclavatarHandle da, uint index, ref uint errorKind, ref IntPtr message, ref uint messageLength);

        [DllImport("declavatar.dll")]
        public static extern StatusCode DeclavatarPushExampleErrors(DeclavatarHandle da);
    }

    internal enum StatusCode : uint
    {
        Success = 0,
        Utf8Error = 1,
        CompileError = 2,
        AlreadyInUse = 3,
        NotCompiled = 4,
        InvalidPointer = 128,
    }

    internal enum ErrorKind : uint
    {
        CompilerError = 0,
        SyntaxError = 1,
        SemanticError = 2,
        SemanticInfo = 3,
    }

    internal sealed class DeclavatarHandle : SafeHandle
    {
        public override bool IsInvalid => handle == IntPtr.Zero;

        private DeclavatarHandle(IntPtr newHandle) : base(IntPtr.Zero, true)
        {
            SetHandle(newHandle);
        }

        protected override bool ReleaseHandle()
        {
            return Native.DeclavatarFree(handle) == (uint)StatusCode.Success;
        }

        public static DeclavatarHandle Create()
        {
            var newHandle = Native.DeclavatarInit();
            return new DeclavatarHandle(newHandle);
        }
    }

    internal sealed class Declavatar : IDisposable
    {
        private DeclavatarHandle _handle = null;
        private bool _disposed = false;

        public Declavatar()
        {
            _handle = DeclavatarHandle.Create();
            if (_handle.IsInvalid) throw new NullReferenceException("failed to create declavatar handle");
        }

        public void Reset()
        {
            Native.DeclavatarReset(_handle);
        }

        public List<(ErrorKind, string)> FetchErrors()
        {
            var errors = new List<(ErrorKind, string)>();
            uint errorsCount = 0;
            Native.DeclavatarGetErrorsCount(_handle, ref errorsCount);

            for (uint i = 0; i < errorsCount; i++)
            {
                uint errorKind = 0;
                IntPtr errorMessage = IntPtr.Zero;
                uint errorLength = 0;
                Native.DeclavatarGetError(_handle, i, ref errorKind, ref errorMessage, ref errorLength);

                var buffer = new byte[errorLength];
                Marshal.Copy(errorMessage, buffer, 0, (int)errorLength);
                var message = Encoding.UTF8.GetString(buffer);

                errors.Add(((ErrorKind)errorKind, message));
            }

            return errors;
        }

        public void PushExampleErrors()
        {
            Native.DeclavatarPushExampleErrors(_handle);
        }

        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        private void Dispose(bool disposing)
        {
            if (_disposed) return;
            if (disposing) this._handle.Dispose();
            _disposed = true;
        }
    }
}
