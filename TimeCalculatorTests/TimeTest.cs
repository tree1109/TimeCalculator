using NUnit.Framework;

namespace TimeCalculator.Tests
{
    [TestFixture()]
    public class TimeTest
    {
        [Test()]
        public void TestTime()
        {
            var time = new Time(12, 5);
            Assert.AreEqual(12, time.Hours);
            Assert.AreEqual(5, time.Minutes);
            Assert.Throws<ArgumentException>(() => new Time(25, 8));
            Assert.Throws<ArgumentException>(() => new Time(16, 99));
        }

        [Test()]
        public void TestTotalMinutes()
        {
            var time = new Time(12, 5);
            Assert.AreEqual(725, time.TotalMinutes);
            time = new Time(16, 48);
            Assert.AreEqual(1008, time.TotalMinutes);
        }

        [Test()]
        public void TestGetTimeDifferenceInMinutes()
        {
            var time1 = new Time(12, 5);
            var time2 = new Time(13, 10);
            Assert.AreEqual(65, time1.GetTimeDifferenceInMinutes(time2));
            time1 = new Time(12, 5);
            time2 = new Time(11, 10);
            Assert.AreEqual(1385, time1.GetTimeDifferenceInMinutes(time2));
            time1 = new Time(12, 5);
            time2 = new Time(12, 5);
            Assert.AreEqual(0, time1.GetTimeDifferenceInMinutes(time2));
        }

        [Test()]
        public void TestGetTimeString()
        {
            var time = new Time(12, 5);
            Assert.AreEqual("12:05", time.GetTimeString());
            time = new Time(16, 48);
            Assert.AreEqual("16:48", time.GetTimeString());
            time = new Time(3, 3);
            Assert.AreEqual("03:03", time.GetTimeString());
        }
    }
}