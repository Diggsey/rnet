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
        public static long Return42i64(
        ) {
            return _FnReturn42i64();
        }
        public static bool ReturnFalseBool(
        ) {
            return (_FnReturnFalseBool() != 0);
        }
        public static bool ReturnTrueBool(
        ) {
            return (_FnReturnTrueBool() != 0);
        }
        public static float Return42f32(
        ) {
            return _FnReturn42f32();
        }
        public static double Return42f64(
        ) {
            return _FnReturn42f64();
        }
        public static String ReturnHelloString(
        ) {
            return _FreeStr(_FnReturnHelloString());
        }
        public static List<int> ReturnFibVec(
        ) {
            return _FreeSlice<int, int, List<int>>(_FnReturnFibVec(), 4, 4, _arg1 => _arg1);
        }
        public static HashSet<int> ReturnPow2HashSet(
        ) {
            return _FreeSlice<int, int, HashSet<int>>(_FnReturnPow2HashSet(), 4, 4, _arg2 => _arg2);
        }
        public static SortedSet<int> ReturnPow2BtreeSet(
        ) {
            return _FreeSlice<int, int, SortedSet<int>>(_FnReturnPow2BtreeSet(), 4, 4, _arg3 => _arg3);
        }
        public static Dictionary<int,bool> ReturnEvensHashMap(
        ) {
            return _FreeDict<int, bool, _RawTuple0, Dictionary<int, bool>>(_FnReturnEvensHashMap(), 8, 4, _arg4 => ((Func<_RawTuple0, (int,bool)>)(_arg5 => (_arg5.elem0,(_arg5.elem1 != 0))))(_arg4));
        }
        public static SortedDictionary<int,bool> ReturnEvensBtreeMap(
        ) {
            return _FreeDict<int, bool, _RawTuple0, SortedDictionary<int, bool>>(_FnReturnEvensBtreeMap(), 8, 4, _arg6 => ((Func<_RawTuple0, (int,bool)>)(_arg7 => (_arg7.elem0,(_arg7.elem1 != 0))))(_arg6));
        }
        public static (int,ulong,bool,float,double,String) ReturnOnesTuple(
        ) {
            return ((Func<_RawTuple1, (int,ulong,bool,float,double,String)>)(_arg8 => (_arg8.elem0,_arg8.elem1,(_arg8.elem2 != 0),_arg8.elem3,_arg8.elem4,_FreeStr(_arg8.elem5))))(_FnReturnOnesTuple());
        }
        public static void ReturnOkUnit(
        ) {
            _DecodeResult(_FnReturnOkUnit());
        }
        public static void ReturnErrUnit(
        ) {
            _DecodeResult(_FnReturnErrUnit());
        }
        public static uint ReturnOk42u32(
        ) {
            return _DecodeResult(_FnReturnOk42u32(), _arg9 => _arg9);
        }
        public static uint ReturnErrU32(
        ) {
            return _DecodeResult(_FnReturnErrU32(), _arg10 => _arg10);
        }
        public static String ReturnOkHelloString(
        ) {
            return _DecodeResult(_FnReturnOkHelloString(), _arg11 => _FreeStr(_arg11));
        }
        public static String ReturnErrString(
        ) {
            return _DecodeResult(_FnReturnErrString(), _arg12 => _FreeStr(_arg12));
        }
        public static List<List<String>> ReturnNestedVec(
        ) {
            return _FreeSlice<List<String>, _RawSlice, List<List<String>>>(_FnReturnNestedVec(), 16, 8, _arg13 => _FreeSlice<String, _RawSlice, List<String>>(_arg13, 16, 8, _arg14 => _FreeStr(_arg14)));
        }
        public static void Pass42i32(
            int arg
        ) {
            _DecodeResult(_FnPass42i32(arg));
        }
        public static void Pass42i64(
            long arg
        ) {
            _DecodeResult(_FnPass42i64(arg));
        }
        public static void PassFalseBool(
            bool arg
        ) {
            _DecodeResult(_FnPassFalseBool((arg ? (byte)1 : (byte)0)));
        }
        public static void PassTrueBool(
            bool arg
        ) {
            _DecodeResult(_FnPassTrueBool((arg ? (byte)1 : (byte)0)));
        }
        public static void Pass42f32(
            float arg
        ) {
            _DecodeResult(_FnPass42f32(arg));
        }
        public static void Pass42f64(
            double arg
        ) {
            _DecodeResult(_FnPass42f64(arg));
        }
        public static void PassHelloString(
            String arg
        ) {
            _DecodeResult(_FnPassHelloString(_AllocStr(arg)));
        }
        public static void PassFibVec(
            IReadOnlyCollection<int> arg
        ) {
            _DecodeResult(_FnPassFibVec(_AllocSlice<int, int>(arg, 4, 4, _arg15 => _arg15)));
        }
        public static void PassPow2HashSet(
            IReadOnlyCollection<int> arg
        ) {
            _DecodeResult(_FnPassPow2HashSet(_AllocSlice<int, int>(arg, 4, 4, _arg16 => _arg16)));
        }
        public static void PassPow2BtreeSet(
            IReadOnlyCollection<int> arg
        ) {
            _DecodeResult(_FnPassPow2BtreeSet(_AllocSlice<int, int>(arg, 4, 4, _arg17 => _arg17)));
        }
        public static void PassEvensHashMap(
            IReadOnlyDictionary<int, bool> arg
        ) {
            _DecodeResult(_FnPassEvensHashMap(_AllocDict<int, bool, _RawTuple0>(arg, 8, 4, _arg18 => ((Func<(int,bool), _RawTuple0>)(_arg19 => new _RawTuple0 { elem0 = _arg19.Item1,elem1 = (_arg19.Item2 ? (byte)1 : (byte)0) }))(_arg18))));
        }
        public static void PassEvensBtreeMap(
            IReadOnlyDictionary<int, bool> arg
        ) {
            _DecodeResult(_FnPassEvensBtreeMap(_AllocDict<int, bool, _RawTuple0>(arg, 8, 4, _arg20 => ((Func<(int,bool), _RawTuple0>)(_arg21 => new _RawTuple0 { elem0 = _arg21.Item1,elem1 = (_arg21.Item2 ? (byte)1 : (byte)0) }))(_arg20))));
        }
        public static void PassOnesTuple(
            (int,ulong,bool,float,double,String) arg
        ) {
            _DecodeResult(_FnPassOnesTuple(((Func<(int,ulong,bool,float,double,String), _RawTuple1>)(_arg22 => new _RawTuple1 { elem0 = _arg22.Item1,elem1 = _arg22.Item2,elem2 = (_arg22.Item3 ? (byte)1 : (byte)0),elem3 = _arg22.Item4,elem4 = _arg22.Item5,elem5 = _AllocStr(_arg22.Item6) }))(arg)));
        }
        public static void PassNestedVec(
            IReadOnlyCollection<List<String>> arg
        ) {
            _DecodeResult(_FnPassNestedVec(_AllocSlice<List<String>, _RawSlice>(arg, 16, 8, _arg23 => _AllocSlice<String, _RawSlice>(_arg23, 16, 8, _arg24 => _AllocStr(_arg24)))));
        }
        [DllImport("rnet_tests", EntryPoint = "rnet_export_do_nothing", CallingConvention = CallingConvention.Cdecl)]
        private static extern void _FnDoNothing(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_42i32", CallingConvention = CallingConvention.Cdecl)]
        private static extern int _FnReturn42i32(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_42i64", CallingConvention = CallingConvention.Cdecl)]
        private static extern long _FnReturn42i64(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_false_bool", CallingConvention = CallingConvention.Cdecl)]
        private static extern byte _FnReturnFalseBool(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_true_bool", CallingConvention = CallingConvention.Cdecl)]
        private static extern byte _FnReturnTrueBool(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_42f32", CallingConvention = CallingConvention.Cdecl)]
        private static extern float _FnReturn42f32(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_42f64", CallingConvention = CallingConvention.Cdecl)]
        private static extern double _FnReturn42f64(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_hello_string", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawSlice _FnReturnHelloString(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_fib_vec", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawSlice _FnReturnFibVec(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_pow2_hash_set", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawSlice _FnReturnPow2HashSet(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_pow2_btree_set", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawSlice _FnReturnPow2BtreeSet(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_evens_hash_map", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawSlice _FnReturnEvensHashMap(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_evens_btree_map", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawSlice _FnReturnEvensBtreeMap(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_ones_tuple", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple1 _FnReturnOnesTuple(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_ok_unit", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnReturnOkUnit(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_err_unit", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnReturnErrUnit(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_ok_42u32", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple3 _FnReturnOk42u32(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_err_u32", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple3 _FnReturnErrU32(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_ok_hello_string", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple4 _FnReturnOkHelloString(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_err_string", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple4 _FnReturnErrString(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_return_nested_vec", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawSlice _FnReturnNestedVec(
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_42i32", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPass42i32(
            int arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_42i64", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPass42i64(
            long arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_false_bool", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPassFalseBool(
            byte arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_true_bool", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPassTrueBool(
            byte arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_42f32", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPass42f32(
            float arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_42f64", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPass42f64(
            double arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_hello_string", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPassHelloString(
            _RawSlice arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_fib_vec", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPassFibVec(
            _RawSlice arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_pow2_hash_set", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPassPow2HashSet(
            _RawSlice arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_pow2_btree_set", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPassPow2BtreeSet(
            _RawSlice arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_evens_hash_map", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPassEvensHashMap(
            _RawSlice arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_evens_btree_map", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPassEvensBtreeMap(
            _RawSlice arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_ones_tuple", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPassOnesTuple(
            _RawTuple1 arg
        );
        [DllImport("rnet_tests", EntryPoint = "rnet_export_pass_nested_vec", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple2 _FnPassNestedVec(
            _RawSlice arg
        );
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple0 {
            public int elem0;
            public byte elem1;
        }
        private static _RawTuple0 _EncodeOption<T>(T? arg, Func<T, int> converter) where T: struct {
            if (arg.HasValue) {
                return new _RawTuple0 { elem0 = converter(arg.Value), elem1 = 1 };
            } else {
                return new _RawTuple0 { elem0 = default(int), elem1 = 0 };
            }
        }
        private static T? _DecodeOption<T>(_RawTuple0 arg, Func<int, T> converter) where T: struct {
            if (arg.elem1 != 0) {
                return converter(arg.elem0);
            } else {
                return null;
            }
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple1 {
            public int elem0;
            public ulong elem1;
            public byte elem2;
            public float elem3;
            public double elem4;
            public _RawSlice elem5;
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple2 {
            public _RawSlice elem0;
            public byte elem1;
        }
        private static _RawTuple2 _EncodeOption<T>(T? arg, Func<T, _RawSlice> converter) where T: struct {
            if (arg.HasValue) {
                return new _RawTuple2 { elem0 = converter(arg.Value), elem1 = 1 };
            } else {
                return new _RawTuple2 { elem0 = default(_RawSlice), elem1 = 0 };
            }
        }
        private static T? _DecodeOption<T>(_RawTuple2 arg, Func<_RawSlice, T> converter) where T: struct {
            if (arg.elem1 != 0) {
                return converter(arg.elem0);
            } else {
                return null;
            }
        }
        private static _RawTuple2 _EncodeResult(Action f) {
            try {
                f();
                return new _RawTuple2 { elem0 = default(_RawSlice), elem1 = 1 };
            } catch (Exception e) {
                return new _RawTuple2 { elem0 = _AllocStr(e.Message), elem1 = 0 };
            }
        }
        private static void _DecodeResult(_RawTuple2 arg) {
            if (arg.elem1 == 0) {
                throw new RustException(_FreeStr(arg.elem0));
            }
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple3 {
            public uint elem0;
            public _RawSlice elem1;
            public byte elem2;
        }
        private static _RawTuple3 _EncodeResult(Func<uint> f) {
            try {
                var res = f();
                return new _RawTuple3 { elem0 = res, elem1 = default(_RawSlice), elem2 = 1 };
            } catch (Exception e) {
                return new _RawTuple3 { elem0 = default(uint), elem1 = _AllocStr(e.Message), elem2 = 0 };
            }
        }
        private static T _DecodeResult<T>(_RawTuple3 arg, Func<uint, T> converter) {
            if (arg.elem2 != 0) {
                return converter(arg.elem0);
            } else {
                throw new RustException(_FreeStr(arg.elem1));
            }
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple4 {
            public _RawSlice elem0;
            public _RawSlice elem1;
            public byte elem2;
        }
        private static _RawTuple4 _EncodeResult(Func<_RawSlice> f) {
            try {
                var res = f();
                return new _RawTuple4 { elem0 = res, elem1 = default(_RawSlice), elem2 = 1 };
            } catch (Exception e) {
                return new _RawTuple4 { elem0 = default(_RawSlice), elem1 = _AllocStr(e.Message), elem2 = 0 };
            }
        }
        private static T _DecodeResult<T>(_RawTuple4 arg, Func<_RawSlice, T> converter) {
            if (arg.elem2 != 0) {
                return converter(arg.elem0);
            } else {
                throw new RustException(_FreeStr(arg.elem1));
            }
        }


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

        private static _RawSlice _AllocDict<TKey, TValue, U>(IReadOnlyDictionary<TKey, TValue> collection, int size, int align, Func<(TKey, TValue), U> converter) where U: unmanaged
        {
            var count = collection.Count;
            var slice = _RawSlice.Alloc((UIntPtr)count, size, align);
            var ptr = slice.ptr;
            foreach (var item in collection)
            {
                Marshal.StructureToPtr<U>(converter((item.Key, item.Value)), ptr, false);
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

        private static TDict _FreeDict<TKey, TValue, U, TDict>(_RawSlice arg, int size, int align, Func<U, (TKey, TValue)> converter) where U : unmanaged where TDict: IDictionary<TKey, TValue>, new()
        {
            unsafe
            {
                var res = new TDict();
                var ptr = arg.ptr;
                for (var i = 0; i < (int)arg.len; ++i)
                {
                    var item = converter(Marshal.PtrToStructure<U>(ptr));
                    res.Add(item.Item1, item.Item2);
                    ptr = (IntPtr)(ptr.ToInt64() + (long)size);
                }
                arg.Free(size, align);
                return res;
            }
        }
    }
}

