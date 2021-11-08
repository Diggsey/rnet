using NUnit.Framework;

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
        public void TestReturnInt()
        {
            Assert.AreEqual(RnetTests.Return42i32(), 42);
        }
    }
}