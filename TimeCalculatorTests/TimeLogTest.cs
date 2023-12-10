using NUnit.Framework;
using Assert = NUnit.Framework.Assert;

namespace TimeCalculator.Tests
{
    [TestFixture()]
    public class TimeLogTest
    {
        [Test()]
        public void TestTimeCalculator()
        {
            var timeInterval1 = new TimeInterval(new Time(6, 0), new Time(18, 30));
            var timeInterval2 = new TimeInterval(new Time(8, 0), new Time(12, 0));
            var timeInterval3 = new TimeInterval(new Time(13, 0), new Time(2, 0));
            var timeIntervals = new List<TimeInterval> { timeInterval1, timeInterval2, timeInterval3 };
            var timeCalculator = new TimeLog(timeIntervals);
            Assert.IsNotNull(timeCalculator);
        }

        [Test()]
        public void TestGetTotalTime()
        {
            var timeInterval1 = new TimeInterval(new Time(6, 0), new Time(18, 30));
            var timeInterval2 = new TimeInterval(new Time(8, 0), new Time(12, 0));
            var timeInterval3 = new TimeInterval(new Time(13, 0), new Time(2, 0));
            var timeIntervals = new List<TimeInterval> { timeInterval1, timeInterval2, timeInterval3 };
            var timeCalculator = new TimeLog(timeIntervals);
            Assert.AreEqual("29:30", timeCalculator.GetTotalTimeString());
        }

        [Test()]
        public void TestGetTimeLogStrings()
        {
            var timeInterval1 = new TimeInterval(new Time(6, 0), new Time(18, 30));
            var timeInterval2 = new TimeInterval(new Time(8, 0), new Time(12, 0));
            var timeInterval3 = new TimeInterval(new Time(13, 0), new Time(2, 0));
            var timeIntervals = new List<TimeInterval> { timeInterval1, timeInterval2, timeInterval3 };
            var timeCalculator = new TimeLog(timeIntervals);
            Assert.AreEqual("06:00 ~ 18:30\n08:00 ~ 12:00\n13:00 ~ 02:00", timeCalculator.GetTimeLogStrings());
        }
    }
}