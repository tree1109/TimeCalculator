namespace TimeCalculator;

public class TimeInterval
{
    private readonly Time _startTime;
    private readonly Time _endTime;

    public TimeInterval(Time startTime, Time endTime)
    {
        _startTime = startTime;
        _endTime = endTime;
    }

    public Time StartTime => _startTime;

    public Time EndTime => _endTime;

    /// <summary>
    /// Calculate the time interval in minutes.
    /// </summary>
    /// <returns></returns>
    public int GetTimeIntervalInMinutes()
    {
        return _startTime.GetTimeDifferenceInMinutes(_endTime);
    }

    /// <summary>
    /// format hh:mm ~ hh:mm
    /// </summary>
    /// <returns></returns>
    public string GetTimeIntervalString()
    {
        return $"{StartTime.GetTimeString()} ~ {EndTime.GetTimeString()}";
    }
}