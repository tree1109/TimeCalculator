using System.Text.RegularExpressions;

namespace TimeCalculator;

public class TimeParser
{
    /// <summary>
    /// Parse a string in the format "hh:mm" into a Time object.
    /// </summary>
    /// <param name="timeString"></param>
    /// <returns></returns>
    public static Time GetTime(string timeString)
    {
        var timeParts = timeString.Split(':');
        var hours = int.Parse(timeParts[0]);
        var minutes = int.Parse(timeParts[1]);
        return new Time(hours, minutes);
    }

    /// <summary>
    /// Parse a list of strings into a list of TimeIntervals.
    /// </summary>
    /// <param name="lines"></param>
    /// <returns></returns>
    public static List<TimeInterval> GetTimeIntervals(IList<string> lines)
    {
        var timeIntervals = new List<TimeInterval>();
        // 文字格式 --> 1. From 12:10 to 13:10
        // 定義時間範圍的正則表達式
        var pattern = @"(\d{1,2}:\d{2})\s*to\s*(\d{1,2}:\d{2})";
        foreach (var line in lines)
        {
            // 使用正則表達式進行匹配
            var match = Regex.Match(line, pattern);
            if (!match.Success) continue;
            // 提取匹配到的時間範圍
            var startTimeString = match.Groups[1].Value;
            var endTimeString = match.Groups[2].Value;
            // 將時間範圍轉換為 TimeInterval 物件
            var startTime = GetTime(startTimeString);
            var endTime = GetTime(endTimeString);
            var timeInterval = new TimeInterval(startTime, endTime);
            timeIntervals.Add(timeInterval);
        }
        return timeIntervals;
    }
}