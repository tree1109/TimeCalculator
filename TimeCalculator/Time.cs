namespace TimeCalculator;

public class Time
{
    private int _hours;
    private int _minutes;

    /// <summary>
    /// time is 00:00 to 23:59
    /// </summary>
    /// <param name="hours"></param>
    /// <param name="minutes"></param>
    public Time(int hours, int minutes)
    {
        Hours = hours;
        Minutes = minutes;
    }

    public int Hours
    {
        get => _hours;
        set
        {
            if (value is < 0 or > 23)
                throw new ArgumentException("Hours must be between 0 and 23");
            _hours = value;
        }
    }

    public int Minutes
    {
        get => _minutes;
        set
        {
            if (value is < 0 or > 59)
                throw new ArgumentException("Minutes must be between 0 and 59");
            _minutes = value;
        }
    }

    /// <summary>
    /// Total minutes from 00:00
    /// </summary>
    public int TotalMinutes => Hours * 60 + Minutes;

    /// <summary>
    /// endTime is always greater than startTime
    /// </summary>
    /// <param name="endTime"></param>
    /// <returns></returns>
    public int GetTimeDifferenceInMinutes(Time endTime)
    {
        var endTimeMinutes = endTime.TotalMinutes;
        if (endTime.Hours < Hours)
            endTimeMinutes += 24 * 60;
        return endTimeMinutes - TotalMinutes;
    }

    /// <summary>
    /// format hh:mm
    /// </summary>
    /// <returns></returns>
    public string GetTimeString()
    {
        return $"{Hours:D2}:{Minutes:D2}";
    }
}