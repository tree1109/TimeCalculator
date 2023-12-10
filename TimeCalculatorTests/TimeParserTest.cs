using NUnit.Framework;

namespace TimeCalculator.Tests
{
    [TestFixture()]
    public class TimeParserTest
    {
        [Test()]
        public void TestParse()
        {
            var time = TimeParser.GetTime("12:30");
            Assert.AreEqual(12, time.Hours);
            Assert.AreEqual(30, time.Minutes);
            time = TimeParser.GetTime("2:30");
            Assert.AreEqual(2, time.Hours);
            Assert.AreEqual(30, time.Minutes);
        }

        [Test()]
        public void TestGetTimeIntervals()
        {
            var lines = new List<string>
            {
                "1. From 12:10 to 13:10",
                "2. From 12:30 to 13:30",
                "3. From 12:40 to 13:40",
                "4. From 12:50 to 13:50",
                "5. From 13:00 to 14:00",
                "6. From 13:10 to 14:10",
                "7. From 13:20 to 14:20",
                "8. From 13:30 to 14:30",
                "9. From 13:40 to 14:40",
                "10. From 13:50 to 14:50",
                "11. From 14:00 to 15:00",
                "12. From 14:10 to 15:10",
                "13. From 14:20 to 15:20",
                "14. From 14:30 to 15:30",
                "15. From 14:40 to 15:40",
                "16. From 14:50 to 15:50",
                "17. From 15:00 to 16:00",
                "18. From 15:10 to 16:10",
                "19. From 15:20 to 16:20",
                "20. From 15:30 to 16:30",
                "21. From 15:40 to 16:40",
                "22. From 15:50 to 16:50",
                "23. From 16:00 to 17:00",
                "24. From 16:10 to 17:10",
                "25. From 16:20 to 17:20",
                "26. From 16:30 to 17:30",
                "27. From 16:40 to 17:40",
                "28. From 16:50 to 17:50",
                "29. From 17:00 to 18:00",
                "30. From 17:10 to 18:10",
                "31. From 17:20 to 18:20",
                "32. From 17:30 to 18:30"
            };
            var timeIntervals = TimeParser.GetTimeIntervals(lines);
            Assert.AreEqual(32, timeIntervals.Count);
        }
    }
}