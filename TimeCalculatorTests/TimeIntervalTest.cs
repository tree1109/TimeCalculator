using NUnit.Framework;

namespace TimeCalculator.Tests
{
    [TestFixture()]
    public class TimeIntervalTest
    {
        [Test()]
        public void TestTimeInterval()
        {
            var time1 = new Time(12, 5);
            var time2 = new Time(13, 10);
            var timeInterval = new TimeInterval(time1, time2);
            Assert.AreEqual(12, timeInterval.StartTime.Hours);
            Assert.AreEqual(5, timeInterval.StartTime.Minutes);
            Assert.AreEqual(13, timeInterval.EndTime.Hours);
            Assert.AreEqual(10, timeInterval.EndTime.Minutes);
        }

        [Test()]
        public void TestGetTimeIntervalInMinutes()
        {
            var time1 = new Time(12, 5);
            var time2 = new Time(13, 10);
            var timeInterval = new TimeInterval(time1, time2);
            Assert.AreEqual(65, timeInterval.GetTimeIntervalInMinutes());
            time1 = new Time(12, 5);
            time2 = new Time(11, 10);
            timeInterval = new TimeInterval(time1, time2);
            Assert.AreEqual(1385, timeInterval.GetTimeIntervalInMinutes());
            time1 = new Time(12, 5);
            time2 = new Time(12, 5);
            timeInterval = new TimeInterval(time1, time2);
            Assert.AreEqual(0, timeInterval.GetTimeIntervalInMinutes());
        }

        [Test()]
        public void TestGetTimeIntervalString()
        {
            var time1 = new Time(12, 5);
            var time2 = new Time(13, 10);
            var timeInterval = new TimeInterval(time1, time2);
            Assert.AreEqual("12:05 ~ 13:10", timeInterval.GetTimeIntervalString());
            time1 = new Time(12, 5);
            time2 = new Time(11, 10);
            timeInterval = new TimeInterval(time1, time2);
            Assert.AreEqual("12:05 ~ 11:10", timeInterval.GetTimeIntervalString());
            time1 = new Time(12, 5);
            time2 = new Time(12, 5);
            timeInterval = new TimeInterval(time1, time2);
            Assert.AreEqual("12:05 ~ 12:05", timeInterval.GetTimeIntervalString());
        }
    }
}