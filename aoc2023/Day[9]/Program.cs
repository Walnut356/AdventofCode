using System;
using System.Collections;
using System.IO;
using static Utils.Utils;
using System.Diagnostics;

// There's probably a math-y way to do this but my math skills are very rusty, so we'll do it the programmer way

int PartOne()
{
    int result = 0;
    using StreamReader stream = GetStream("Nine");

    var lines = GetRemainingLines(stream);

    foreach(var line in lines) {
        List<int> temp = [];
        foreach(var token in GetTokens(line)) {
            temp.Add(int.Parse(token));
        }
        List<List<int>> vals = [temp];

        bool prev_zero = false;

        while (!prev_zero) {
            vals.Add([]);
            // dumb hack that lets me exit the while loop without re-traversing any arrays, see below:
            prev_zero = true;
            for(int i = 1; i < vals[^2].Count; ++i) {
                var x = vals[^2][i] - vals[^2][i - 1];
                vals[^1].Add(x);

                // i.e. if all previous values are 0, and this one is 0, we're still on track to exit the while loop
                // but the moment any of them fail the x == 0 check, all will.
                prev_zero = prev_zero && x == 0;
            }
        }

        var prev = 0;

        for(int i = vals.Count - 2; i >= 0; --i) {
            prev = vals[i][^1] + prev;
        }

        result += prev;
    }

    return result;
}

int PartTwo()
{
    int result = 0;
    using StreamReader stream = GetStream("Nine");

    var lines = GetRemainingLines(stream);

    foreach (var line in lines)
    {
        List<int> temp = [];
        foreach (var token in GetTokens(line))
        {
            temp.Add(int.Parse(token));
        }
        List<List<int>> vals = [temp];

        bool prev_zero = false;

        while (!prev_zero)
        {
            vals.Add([]);
            // dumb hack that lets me exit the while loop without re-traversing any arrays, see below:
            prev_zero = true;
            for (int i = 1; i < vals[^2].Count; ++i)
            {
                var x = vals[^2][i] - vals[^2][i - 1];
                vals[^1].Add(x);

                // i.e. if all previous values are 0, and this one is 0, we're still on track to exit the while loop
                // but the moment any of them fail the x == 0 check, all will.
                prev_zero = prev_zero && x == 0;
            }
        }

        var prev = 0;

        for (int i = vals.Count - 2; i >= 0; --i)
        {
            prev = vals[i][0] - prev;
        }

        result += prev;
    }

    return result;
}

var start = Stopwatch.StartNew();
var result = PartOne();
start.Stop();
Console.WriteLine($"// Result: {result} in {start.Elapsed}");
// Result: 1762065988 in 00:00:00.0267655ms

start = Stopwatch.StartNew();
result = PartTwo();
start.Stop();
Console.WriteLine($"// Result: {result} in {start.Elapsed}");
// Result: 1066 in 00:00:00.0027235ms

// easiest part 2 of my life? I changed 3 characters lol