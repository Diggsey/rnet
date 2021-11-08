using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace RnetTests
{    
    public static class RnetTests
    {
        public class RustException: Exception {
            public RustException(string message) : base(message) { }
        }

        public interface IOpaqueHandle: IEquatable<IOpaqueHandle>, IDisposable {}

        
        public static void DoNothing(
        ) {
            _FnDoNothing();
        }
        public static int Return42i32(
        ) {
            return _FnReturn42i32();
        }
        [DllImport("rnet_tests", EntryPoint = "do_nothing", CallingConvention = CallingConvention.Cdecl)]
        private static extern void _FnDoNothing(
        );
        [DllImport("rnet_tests", EntryPoint = "return_42i32", CallingConvention = CallingConvention.Cdecl)]
        private static extern int _FnReturn42i32(
        );


        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate void _ManageDelegateDelegate(IntPtr ptr, int adjust);

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate void _DropOpaqueDelegate(IntPtr ptr);

        private static Dictionary<IntPtr, (int, Delegate, Delegate)> _ActiveDelegates = new Dictionary<IntPtr, (int, Delegate, Delegate)>();

        private static _ManageDelegateDelegate _manageDelegate = _ManageDelegate;
        private static IntPtr _manageDelegatePtr = Marshal.GetFunctionPointerForDelegate(_manageDelegate);

        private static void _ManageDelegate(IntPtr ptr, int adjust)
        {
            lock (_ActiveDelegates)
            {
                var item = _ActiveDelegates[ptr];
                item.Item1 += adjust;
                if (item.Item1 > 0)
                {
                    _ActiveDelegates[ptr] = item;
                }
                else
                {
                    _ActiveDelegates.Remove(ptr);
                }
            }
        }

        private static _RawDelegate _AllocDelegate(Delegate d, Delegate original)
        {
            var ptr = Marshal.GetFunctionPointerForDelegate(d);
            lock (_ActiveDelegates)
            {
                if (_ActiveDelegates.ContainsKey(ptr))
                {
                    var item = _ActiveDelegates[ptr];
                    item.Item1 += 1;
                    _ActiveDelegates[ptr] = item;
                } else
                {
                    _ActiveDelegates.Add(ptr, (1, d, original));
                }
            }
            return new _RawDelegate
            {
                call_fn = ptr,
                drop_fn = _manageDelegatePtr,
            };
        }

        private static Delegate _FreeDelegate(_RawDelegate d)
        {
            var ptr = d.call_fn;
            lock (_ActiveDelegates)
            {
                var item = _ActiveDelegates[ptr];
                item.Item1 -= 1;
                if (item.Item1 > 0)
                {
                    _ActiveDelegates[ptr] = item;
                }
                else
                {
                    _ActiveDelegates.Remove(ptr);
                }
                return item.Item3;
            }
        }

        [DllImport("rnet_tests", EntryPoint = "rnet_alloc", CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr _Alloc( UIntPtr size, UIntPtr align);

        [DllImport("rnet_tests", EntryPoint = "rnet_free", CallingConvention = CallingConvention.Cdecl)]
        private static extern void _Free(IntPtr ptr, UIntPtr size, UIntPtr align);

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawSlice
        {
            public IntPtr ptr;
            public UIntPtr len;

            public static _RawSlice Alloc(UIntPtr len, int size, int align)
            {
                if (len == UIntPtr.Zero)
                {
                    return new _RawSlice {
                        ptr = (IntPtr)align,
                        len = UIntPtr.Zero,
                    };
                } else
                {
                    return new _RawSlice
                    {
                        ptr = _Alloc((UIntPtr)((UInt64)len * (UInt64)size), (UIntPtr)align),
                        len = len,
                    };
                }
            }

            public void Free(int size, int align)
            {
                if (len != UIntPtr.Zero)
                {
                    _Free(ptr, (UIntPtr)((UInt64)len * (UInt64)size), (UIntPtr)align);
                    ptr = (IntPtr)1;
                    len = UIntPtr.Zero;
                }
            }
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawOpaqueHandle
        {
            public IntPtr ptr;
            public IntPtr drop_fn;
            public ulong type_id;

            public void Drop()
            {
                if (ptr != IntPtr.Zero)
                {
                    var drop = Marshal.GetDelegateForFunctionPointer<_DropOpaqueDelegate>(drop_fn);
                    drop(ptr);
                    ptr = IntPtr.Zero;
                }
            }
        }

        private class _OpaqueHandle : IOpaqueHandle
        {
            private _RawOpaqueHandle inner;

            public _OpaqueHandle(_RawOpaqueHandle inner)
            {
                this.inner = inner;
            }

            public _RawOpaqueHandle ToInner(ulong type_id)
            {
                if (type_id != inner.type_id)
                {
                    throw new InvalidCastException("Opaque handle does not have the correct type");
                }
                return this.inner;
            }

            ~_OpaqueHandle()
            {
                inner.Drop();
            }

            public override bool Equals(object obj)
            {
                return Equals(obj as _OpaqueHandle);
            }

            public bool Equals(IOpaqueHandle other)
            {
                var casted = other as _OpaqueHandle;
                return casted != null &&
                       inner.ptr == casted.inner.ptr && inner.type_id == casted.inner.type_id;
            }

            public override int GetHashCode()
            {
                return inner.ptr.GetHashCode() + inner.type_id.GetHashCode();
            }

            public void Dispose()
            {
                inner.Drop();
            }
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawDelegate
        {
            public IntPtr call_fn;
            public IntPtr drop_fn;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple<T0, T1> where T0 : unmanaged where T1 : unmanaged
        {
            public T0 elem0;
            public T1 elem1;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple<T0, T1, T2>
        {
            public T0 elem0;
            public T1 elem1;
            public T2 elem2;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple<T0, T1, T2, T3>
        {
            public T0 elem0;
            public T1 elem1;
            public T2 elem2;
            public T3 elem3;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple<T0, T1, T2, T3, T4>
        {
            public T0 elem0;
            public T1 elem1;
            public T2 elem2;
            public T3 elem3;
            public T4 elem4;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple<T0, T1, T2, T3, T4, T5>
        {
            public T0 elem0;
            public T1 elem1;
            public T2 elem2;
            public T3 elem3;
            public T4 elem4;
            public T5 elem5;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple<T0, T1, T2, T3, T4, T5, T6>
        {
            public T0 elem0;
            public T1 elem1;
            public T2 elem2;
            public T3 elem3;
            public T4 elem4;
            public T5 elem5;
            public T6 elem6;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple<T0, T1, T2, T3, T4, T5, T6, T7>
        {
            public T0 elem0;
            public T1 elem1;
            public T2 elem2;
            public T3 elem3;
            public T4 elem4;
            public T5 elem5;
            public T6 elem6;
            public T7 elem7;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple<T0, T1, T2, T3, T4, T5, T6, T7, T8>
        {
            public T0 elem0;
            public T1 elem1;
            public T2 elem2;
            public T3 elem3;
            public T4 elem4;
            public T5 elem5;
            public T6 elem6;
            public T7 elem7;
            public T8 elem8;
        }

        private static IntPtr _AllocBox<T>(T arg, int size, int align)
        {
            if (size > 0) {
                var ptr = _Alloc((UIntPtr)size, (UIntPtr)align);
                Marshal.StructureToPtr(arg, ptr, false);
                return ptr;
            } else {
                return (IntPtr)align;
            }
        }

        private static _RawSlice _AllocStr(string arg)
        {
            var nb = Encoding.UTF8.GetByteCount(arg);
            var slice = _RawSlice.Alloc((UIntPtr)nb, 1, 1);
            unsafe
            {
                fixed (char* firstChar = arg)
                {
                    nb = Encoding.UTF8.GetBytes(firstChar, arg.Length, (byte*)slice.ptr, nb);
                }
            }
            return slice;
        }

        private static _RawSlice _AllocSlice<T, U>(IReadOnlyCollection<T> collection, int size, int align, Func<T, U> converter) {
            var count = collection.Count;
            var slice = _RawSlice.Alloc((UIntPtr)count, size, align);
            var ptr = slice.ptr;
            foreach (var item in collection) {
                Marshal.StructureToPtr(converter(item), ptr, false);
                ptr = (IntPtr)(ptr.ToInt64() + (long)size);
            }
            return slice;
        }

        private static _RawSlice _AllocDict<TKey, TValue, UKey, UValue>(IReadOnlyDictionary<TKey, TValue> collection, int size, int align, Func<(TKey, TValue), _RawTuple<UKey, UValue>> converter) where UKey: unmanaged where UValue: unmanaged
        {
            var count = collection.Count;
            var slice = _RawSlice.Alloc((UIntPtr)count, size, align);
            var ptr = slice.ptr;
            foreach (var item in collection)
            {
                Marshal.StructureToPtr(converter((item.Key, item.Value)), ptr, false);
                ptr = (IntPtr)(ptr.ToInt64() + (long)size);
            }
            return slice;
        }

        private static T _FreeBox<T>(IntPtr ptr, int size, int align)
        {
            var res = Marshal.PtrToStructure<T>(ptr);
            if (size > 0) {
                _Free(ptr, (UIntPtr)size, (UIntPtr)align);
            }
            return res;
        }

        private static String _FreeStr(_RawSlice arg)
        {
            unsafe
            {
                var res = Encoding.UTF8.GetString((byte*)arg.ptr, (int)arg.len);
                arg.Free(1, 1);
                return res;
            }
        }

        private static TList _FreeSlice<T, U, TList>(_RawSlice arg, int size, int align, Func<U, T> converter) where TList: ICollection<T>, new()
        {
            unsafe
            {
                var res = new TList();
                var ptr = arg.ptr;
                for (var i = 0; i < (int)arg.len; ++i) {
                    res.Add(converter(Marshal.PtrToStructure<U>(ptr)));
                    ptr = (IntPtr)(ptr.ToInt64() + (long)size);
                }
                arg.Free(size, align);
                return res;
            }
        }

        private static TDict _FreeDict<TKey, TValue, UKey, UValue, TDict>(_RawSlice arg, int size, int align, Func<_RawTuple<UKey, UValue>, (TKey, TValue)> converter) where UKey : unmanaged where UValue : unmanaged where TDict: IDictionary<TKey, TValue>, new()
        {
            unsafe
            {
                var res = new TDict();
                var ptr = arg.ptr;
                for (var i = 0; i < (int)arg.len; ++i)
                {
                    var item = converter(Marshal.PtrToStructure<_RawTuple<UKey, UValue>>(ptr));
                    res.Add(item.Item1, item.Item2);
                    ptr = (IntPtr)(ptr.ToInt64() + (long)size);
                }
                arg.Free(size, align);
                return res;
            }
        }

        private static _RawTuple<U, byte> _EncodeOption<T, U>(T? arg, Func<T, U> converter) where T: struct where U : unmanaged
        {
            if (arg.HasValue)
            {
                return new _RawTuple<U, byte> { elem0 = converter(arg.Value), elem1 = 1 };
            } else
            {
                return new _RawTuple<U, byte> { elem0 = default(U), elem1 = 0 };
            }
        }

        private static T? _DecodeOption<T, U>(_RawTuple<U, byte> arg, Func<U, T> converter) where T : struct where U : unmanaged
        {
            if (arg.elem1 != 0)
            {
                return converter(arg.elem0);
            }
            else
            {
                return null;
            }
        }

        private static _RawTuple<U, _RawSlice, byte> _EncodeResult<U>(Func<U> f) where U : unmanaged
        {
            try
            {
                var res = f();
                return new _RawTuple<U, _RawSlice, byte> { elem0 = res, elem1 = default(_RawSlice), elem2 = 1 };
            } catch (Exception e)
            {
                return new _RawTuple<U, _RawSlice, byte> { elem0 = default(U), elem1 = _AllocStr(e.Message), elem2 = 0 };
            }
        }

        private static _RawTuple<_RawSlice, byte> _EncodeVoidResult(Action f)
        {
            try
            {
                f();
                return new _RawTuple<_RawSlice, byte> { elem0 = default(_RawSlice), elem1 = 1 };
            }
            catch (Exception e)
            {
                return new _RawTuple<_RawSlice, byte> { elem0 = _AllocStr(e.Message), elem1 = 0 };
            }
        }

        private static T _DecodeResult<T, U>(_RawTuple<U, _RawSlice, byte> arg, Func<U, T> converter) where U : unmanaged
        {
            if (arg.elem2 != 0)
            {
                return converter(arg.elem0);
            }
            else
            {
                throw new RustException(_FreeStr(arg.elem1));
            }
        }

        private static void _DecodeVoidResult(_RawTuple<_RawSlice, byte> arg)
        {
            if (arg.elem1 == 0)
            {
                throw new RustException(_FreeStr(arg.elem0));
            }
        }
    }
}

