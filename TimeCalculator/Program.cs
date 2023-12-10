using TimeCalculator;

internal class Program
{
    public static void Main(string[] args)
    {
        string? filePath;

        if (args.Length == 0)
        {
            Console.WriteLine("請指定要讀取的文字檔案，可將檔案拖曳到視窗~");
            Console.Write("檔案路徑 -> ");
            filePath = Console.ReadLine();
        }
        else
        {
            filePath = args[0];
        }

        File.Exists(filePath);

        var lines = File.ReadAllLines(filePath);
        var timeIntervals = TimeParser.GetTimeIntervals(lines);
        var timeLog = new TimeLog(timeIntervals);
        var totalTimeString = timeLog.GetTotalTimeString();
        var timeLogString = timeLog.GetTimeLogStrings();
        Console.Clear();
        Console.WriteLine($"時段: \n{timeLogString}");
        Console.WriteLine($"總時數為: {totalTimeString}");
        Console.WriteLine("\n按任意鍵結束...");
        Console.ReadLine();
    }
}