namespace TimeCalculator;

public class TimeLog
{
    private readonly IList<TimeInterval> _timeIntervals;

    /// <summary>
    /// Initialize a TimeLog object with a list of TimeIntervals.
    /// </summary>
    /// <param name="timeIntervals"></param>
    public TimeLog(IList<TimeInterval> timeIntervals)
    {
        _timeIntervals = timeIntervals;
    }

    /// <summary>
    /// Calculate the total time in minutes.
    /// </summary>
    private int GetTotalTimeInMinutes()
    {
        var totalTimeInMinutes = 0;
        foreach (var timeInterval in _timeIntervals)
            totalTimeInMinutes += timeInterval.GetTimeIntervalInMinutes();
        return totalTimeInMinutes;
    }

    /// <summary>
    /// Get the total time in the format "hh:mm".
    /// </summary>
    /// <returns></returns>
    public string GetTotalTimeString()
    {
        var totalTimeInMinutes = GetTotalTimeInMinutes();
        var hours = totalTimeInMinutes / 60;
        var minutes = totalTimeInMinutes % 60;
        return $"{hours, 2}:{minutes, 2}";
    }

    /// <summary>
    /// format with 1. From 12:10 to 13:10
    /// </summary>
    /// <returns></returns>
    public string GetTimeLogStrings()
    {
        var timeLogStrings = new List<string>();
        foreach (var timeInterval in _timeIntervals)
            timeLogStrings.Add(timeInterval.GetTimeIntervalString());
        return string.Join("\n", timeLogStrings);
    }
}