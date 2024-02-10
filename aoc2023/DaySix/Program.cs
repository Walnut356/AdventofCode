using System;
using System.Collections;
using System.IO;
using static Utils.Utils;
using System.Diagnostics;

/*
    I could solve this with math but brute forcing it is still ridiculously fast for numbers this
    small. Due to the flat relationship between time held and speed, the distance travelled
    will always be a palindrome. Thus we only need to find the first instance of a number higher
    than the record, and our range will be mirrored over the midpoint, resulting in a range of
    (i, time - i + 1). We also never need to check 0 or time because those could never beat any record
*/
int PartOne()
{
    int result = 0;
    using StreamReader stream = GetStream("DaySix");

    string[] time_str = GetTokens(stream.ReadLine()!)[1..]; // skip the label token
    string[] distance_str = GetTokens(stream.ReadLine()!)[1..];

    List<int> times = [];
    foreach (string val in time_str)
    {
        times.Add(int.Parse(val));
    }

    List<int> dists = [];
    foreach (string val in distance_str)
    {
        dists.Add(int.Parse(val));
    }

    List<int> results = [];

    for (int i = 0; i < times.Count; ++i)
    {
        int time = times[i];
        int dist = dists[i];

        int temp = 0;

        for (int j = 1; j < time; ++j)
        {
            // calc distance and break on the first winning value
            if (j * (time - j) > dist)
            {
                temp = j;
                break;
            }
        }
        // the total number of values that can win will be the range (i, time - i + 1)
        // odd numbered times need an extra 1 added on.
        results.Add(time - temp - temp + 1);
    }

    return results.Aggregate((a, b) => a * b);
}

Int64 PartTwo()
{
    using StreamReader stream = GetStream("DaySix");

    string[] time_str = GetTokens(stream.ReadLine()!)[1..]; // skip the label token
    string[] distance_str = GetTokens(stream.ReadLine()!)[1..];

    string temp = "";
    foreach (string val in time_str)
    {
        temp += val;
    }

    Int64 time = Int64.Parse(temp);

    temp = "";
    foreach (string val in distance_str)
    {
        temp += val;
    }

    Int64 dist = Int64.Parse(temp);

    Int64 res = 0;

    for (Int64 j = 1; j < time; ++j)
    {
        // calc distance and break on the first winning value
        if (j * (time - j) > dist)
        {
            res = j;
            break;
        }
    }


    return time - res - res + 1;
}

var start = Stopwatch.StartNew();
var result = PartOne();
start.Stop();
Console.WriteLine($"Result: {result} in {start.Elapsed.Milliseconds}ms");
// got 4568778 in 12ms

start = Stopwatch.StartNew();
var result2 = PartTwo();
start.Stop();
Console.WriteLine($"Result: {result2} in {start.Elapsed.Milliseconds}ms");
// got 28973936 in 30ms