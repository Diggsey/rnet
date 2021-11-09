using NUnit.Framework;
using System.Collections.Generic;
using System.Linq;

namespace RnetTests
{
    public class Tests
    {
        [SetUp]
        public void Setup()
        {
        }

        [Test]
        public void TestDoNothing()
        {
            RnetTests.DoNothing();
        }

        [Test]
        public void TestReturnPrimitive()
        {
            Assert.AreEqual(RnetTests.Return42i32(), 42);
            Assert.AreEqual(RnetTests.Return42i64(), 42L);
            Assert.AreEqual(RnetTests.ReturnFalseBool(), false);
            Assert.AreEqual(RnetTests.ReturnTrueBool(), true);
            Assert.AreEqual(RnetTests.Return42f32(), 42f);
            Assert.AreEqual(RnetTests.Return42f64(), 42.0);
        }

        private bool UnorderedEqual<T>(IEnumerable<T> a, IEnumerable<T> b)
        {
            return (a.Count() == b.Count()) && !a.Except(b).Any();
        }

        [Test]
        public void TestReturnContainer()
        {
            Assert.AreEqual(RnetTests.ReturnHelloString(), "hello");
            Assert.AreEqual(RnetTests.ReturnFibVec(), new List<int>() { 0, 1, 1, 2, 3, 5, 8, 13, 21 });
            Assert.True(UnorderedEqual(RnetTests.ReturnPow2HashSet(), new HashSet<int>() { 1, 2, 4, 8, 16, 32, 64, 128, 256 }));
            Assert.True(UnorderedEqual(RnetTests.ReturnPow2BtreeSet(), new SortedSet<int>() { 1, 2, 4, 8, 16, 32, 64, 128, 256 }));
            Assert.True(UnorderedEqual(RnetTests.ReturnEvensHashMap(), new Dictionary<int, bool>() { { 0, true }, { 1, false }, { 2, true }, { 3, false } }));
            Assert.True(UnorderedEqual(RnetTests.ReturnEvensBtreeMap(), new SortedDictionary<int, bool>() { { 0, true }, { 1, false }, { 2, true }, { 3, false } }));
            Assert.AreEqual(RnetTests.ReturnOnesTuple(), (1, 1L, true, 1f, 1.0, "one"));
            Assert.AreEqual(RnetTests.ReturnNestedVec(), new List<List<string>>() { new List<string>() { "foo" } });
        }

        [Test]
        public void TestReturnFallible()
        {
            RnetTests.ReturnOkUnit();
            Assert.Throws<RnetTests.RustException>(() => RnetTests.ReturnErrUnit(), "Err");
            Assert.AreEqual(RnetTests.ReturnOk42u32(), 42u);
            Assert.Throws<RnetTests.RustException>(() => RnetTests.ReturnErrU32(), "Err");
            Assert.AreEqual(RnetTests.ReturnOkHelloString(), "hello");
            Assert.Throws<RnetTests.RustException>(() => RnetTests.ReturnErrString(), "Err");
        }
        
        [Test]
        public void TestPassPrimitive()
        {
            RnetTests.Pass42i32(42);
            RnetTests.Pass42i64(42L);
            RnetTests.PassFalseBool(false);
            RnetTests.PassTrueBool(true);
            RnetTests.Pass42f32(42f);
            RnetTests.Pass42f64(42.0);
        }

        [Test]
        public void TestPassContainer()
        {
            RnetTests.PassHelloString("hello");
            RnetTests.PassFibVec(new List<int>() { 0, 1, 1, 2, 3, 5, 8, 13, 21 });
            RnetTests.PassPow2HashSet(new HashSet<int>() { 1, 2, 4, 8, 16, 32, 64, 128, 256 });
            RnetTests.PassPow2BtreeSet(new SortedSet<int>() { 1, 2, 4, 8, 16, 32, 64, 128, 256 });
            RnetTests.PassEvensHashMap(new Dictionary<int, bool>() { { 0, true }, { 1, false }, { 2, true }, { 3, false } });
            RnetTests.PassEvensBtreeMap(new SortedDictionary<int, bool>() { { 0, true }, { 1, false }, { 2, true }, { 3, false } });
            RnetTests.PassOnesTuple((1, 1L, true, 1f, 1.0, "one"));
            RnetTests.PassNestedVec(new List<List<string>>() { new List<string>() { "foo" } });
        }

    }
}