using System;
using System.Collections;
using System.IO;
using static Utils.Utils;
using System.Diagnostics;

Int64 lcm(Int64 l, Int64 r) {
    var x = l;
    var y = r;

    while (y != 0) {
        Int64 temp = y;
        y = x % y;
        x = temp;
    }

    return l / x * r;
}

Int64 PartOne()
{
    Int64 result = 0;
    using StreamReader stream = GetStream("Eight");

    var directions = stream.ReadLine();

    string[] lines = GetRemainingLines(stream);

    Dictionary<string, (string, string)> map = [];

    foreach (string line in lines)
    {
        if (line == "")
        {
            continue;
        }
        var tokens = GetTokens(line);
        // tokens will always be of the form ["name", "=", "(DDD,", "EEE)"]
        // simple string manipulation indexing can get us the 3 values we want easily

        map.Add(tokens[0], (tokens[2][1..^1], tokens[3][..^1]));
    }

    var curr = "AAA";

    while (curr != "ZZZ")
    {
        foreach (char dir in directions)
        {
            curr = dir switch
            {
                'L' => map[curr].Item1,
                'R' => map[curr].Item2,
                _ => throw new Exception(),
            };
            result += 1;
            if (curr == "ZZZ")
            {
                break;
            }
        }
    }

    return result;
}

Int64 PartTwo()
{
    using StreamReader stream = GetStream("Eight");

    var directions = stream.ReadLine();

    string[] lines = GetRemainingLines(stream);

    Dictionary<string, (string, string)> map = [];
    List<string> starts = [];

    foreach (string line in lines)
    {
        if (line == "")
        {
            continue;
        }
        var tokens = GetTokens(line);
        // tokens will always be of the form ["name", "=", "(DDD,", "EEE)"]
        // simple string manipulation indexing can get us the 3 values we want easily

        map.Add(tokens[0], (tokens[2][1..^1], tokens[3][..^1]));
        if (tokens[0].EndsWith('A'))
        {
            starts.Add(tokens[0]);
        }
    }

    List<Int64> counts = [];
    for(int i = 0; i < starts.Count; ++i)
    {
        var count = 0;
        while (!starts[i].EndsWith('Z'))
        {
            foreach (char dir in directions)
            {
                count += 1;
                starts[i] = dir switch
                {
                    'L' => map[starts[i]].Item1,
                    'R' => map[starts[i]].Item2,
                    _ => throw new Exception(),
                };
                if (starts[i].EndsWith('Z'))
                {
                    break;
                }
            }
        }

        counts.Add(count);
    }

    return counts.Aggregate(lcm);
}

var start = Stopwatch.StartNew();
var result = PartOne();
start.Stop();
Console.WriteLine($"// Result: {result} in {start.Elapsed}");
// Result: 18727 in 00:00:00.0119718

start = Stopwatch.StartNew();
result = PartTwo();
start.Stop();
Console.WriteLine($"// Result: {result} in {start.Elapsed}");
// Result: 18024643846273 in 00:00:00.0129390
