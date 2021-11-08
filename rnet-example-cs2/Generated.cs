using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace RnetExample
{    
    public static class RnetExample
    {
        public class RustException: Exception {
            public RustException(string message) : base(message) { }
        }

        public interface IOpaqueHandle: IEquatable<IOpaqueHandle>, IDisposable {}

        
        public struct Foo {
            public String field0;
            public List<bool> field1;
            public Func<Foo, Foo> field2;
            public Dictionary<String,bool> field3;
        }
        public static void Hello(
            String name
        ) {
            _DecodeVoidResult(_FnHello(_AllocStr(name)));
        }
        public static void HelloMany(
            IReadOnlyCollection<String> names
        ) {
            _FnHelloMany(_AllocSlice<String, _RawSlice>(names, 16, 8, _arg1 => _AllocStr(_arg1)));
        }
        public static (bool,bool) IsEven(
            int value
        ) {
            return ((Func<_RawTuple<byte,byte>, (bool,bool)>)(_arg2 => ((_arg2.elem0 != 0),(_arg2.elem1 != 0))))(_FnIsEven(value));
        }
        public static List<byte> StrToBytes(
            String value
        ) {
            return _FreeSlice<byte, byte, List<byte>>(_FnStrToBytes(_AllocStr(value)), 1, 1, _arg3 => _arg3);
        }
        public static Foo Test(
            Func<bool, Foo> arg
        ) {
            return (_FnTest(((Func<Func<bool, Foo>, _RawDelegate>)(_arg4 => _AllocDelegate(new _LocalDelegate5((_arg4_arg0) => _StructFoo.Encode(_arg4((_arg4_arg0 != 0)))), _arg4)))(arg))).Decode();
        }
        public static List<List<List<String>>> Test2(
            Func<IReadOnlyCollection<List<List<String>>>> arg
        ) {
            return _FreeSlice<List<List<String>>, _RawSlice, List<List<List<String>>>>(_FnTest2(((Func<Func<IReadOnlyCollection<List<List<String>>>>, _RawDelegate>)(_arg6 => _AllocDelegate(new _LocalDelegate10(() => _AllocSlice<List<List<String>>, _RawSlice>(_arg6(), 16, 8, _arg7 => _AllocSlice<List<String>, _RawSlice>(_arg7, 16, 8, _arg8 => _AllocSlice<String, _RawSlice>(_arg8, 16, 8, _arg9 => _AllocStr(_arg9))))), _arg6)))(arg)), 16, 8, _arg11 => _FreeSlice<List<String>, _RawSlice, List<List<String>>>(_arg11, 16, 8, _arg12 => _FreeSlice<String, _RawSlice, List<String>>(_arg12, 16, 8, _arg13 => _FreeStr(_arg13))));
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _StructFoo {
            public _RawSlice field0;
            public _RawSlice field1;
            public _RawDelegate field2;
            public _RawSlice field3;
            public static _StructFoo Encode(Foo structArg) {
                return new _StructFoo {
                    field0 = _AllocStr(structArg.field0),
                    field1 = _AllocSlice<bool, byte>(structArg.field1, 1, 1, _arg14 => (_arg14 ? (byte)1 : (byte)0)),
                    field2 = ((Func<Func<Foo, Foo>, _RawDelegate>)(_arg15 => _AllocDelegate(new _LocalDelegate16((_arg15_arg0) => _StructFoo.Encode(_arg15((_arg15_arg0).Decode()))), _arg15)))(structArg.field2),
                    field3 = _AllocDict<String, bool, _RawSlice, byte>(structArg.field3, 24, 8, _arg17 => ((Func<(String,bool), _RawTuple<_RawSlice,byte>>)(_arg18 => new _RawTuple<_RawSlice,byte> { elem0 = _AllocStr(_arg18.Item1),elem1 = (_arg18.Item2 ? (byte)1 : (byte)0) }))(_arg17))
                };
            }
            public Foo Decode() {
                return new Foo {
                    field0 = _FreeStr(this.field0),
                    field1 = _FreeSlice<bool, byte, List<bool>>(this.field1, 1, 1, _arg19 => (_arg19 != 0)),
                    field2 = (Func<Foo, Foo>)_FreeDelegate(this.field2),
                    field3 = _FreeDict<String, bool, _RawSlice, byte, Dictionary<String, bool>>(this.field3, 24, 8, _arg20 => ((Func<_RawTuple<_RawSlice,byte>, (String,bool)>)(_arg21 => (_FreeStr(_arg21.elem0),(_arg21.elem1 != 0))))(_arg20))
                };
            }
        }
        [DllImport("rnet_example", EntryPoint = "hello", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple<_RawSlice, byte> _FnHello(
            _RawSlice name
        );
        [DllImport("rnet_example", EntryPoint = "hello_many", CallingConvention = CallingConvention.Cdecl)]
        private static extern void _FnHelloMany(
            _RawSlice names
        );
        [DllImport("rnet_example", EntryPoint = "is_even", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple<byte,byte> _FnIsEven(
            int value
        );
        [DllImport("rnet_example", EntryPoint = "str_to_bytes", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawSlice _FnStrToBytes(
            _RawSlice value
        );
        [DllImport("rnet_example", EntryPoint = "test", CallingConvention = CallingConvention.Cdecl)]
        private static extern _StructFoo _FnTest(
            _RawDelegate arg
        );
        [DllImport("rnet_example", EntryPoint = "test2", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawSlice _FnTest2(
            _RawDelegate arg
        );
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] delegate _StructFoo _LocalDelegate5(byte arg0);
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] delegate _RawSlice _LocalDelegate10();
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] delegate _StructFoo _LocalDelegate16(_StructFoo arg0);


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

        [DllImport("rnet_example", EntryPoint = "rnet_alloc", CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr _Alloc( UIntPtr size, UIntPtr align);

        [DllImport("rnet_example", EntryPoint = "rnet_free", CallingConvention = CallingConvention.Cdecl)]
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

