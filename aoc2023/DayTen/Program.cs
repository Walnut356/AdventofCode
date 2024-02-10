using System;
using System.Collections;
using System.IO;
using static Utils.Utils;
using System.Diagnostics;
using Map;

int PartOne()
{
    int result = 0;
    using StreamReader stream = GetStream("Ten");

    var lines = GetRemainingLines(stream);

    Tile? start = null;
    int i = 0;
    foreach(var line in lines) {
        int j = 0;
        foreach(char c in line) {
            if(c == 'S') {
                start = new(ref lines, j, i);
            }
            j += 1;
        }
        if(start.HasValue) {
            break;
        }
        i += 1;

    }

    Tile current = start!.Value.ConnectsTo(ref lines).Item1;
    Tile prev = start!.Value;

    while(current != start.Value) {
        var temp = current.GetNext(ref lines, prev);
        prev = current;
        current = temp;
        result += 1;
    }

    return result / 2 + 1;
}

int PartTwo()
{
    int result = 0;
    using StreamReader stream = GetStream("Ten");

    var lines = GetRemainingLines(stream);

    Tile? start = null;
    int i = 0;
    foreach (var line in lines)
    {
        int j = 0;
        foreach (char c in line)
        {
            if (c == 'S')
            {
                start = new(ref lines, j, i);
            }
            j += 1;
        }
        if (start.HasValue)
        {
            break;
        }
        i += 1;

    }

    Tile current = start!.Value.ConnectsTo(ref lines).Item1;

    List<Tile> points = [start!.Value];

    while (current != start.Value)
    {
        var temp = current.GetNext(ref lines, points[^1]);
        points.Add(current);
        current = temp;
    }

    // shoelace formula
    var area = 0;
    for(int k = 0; k < points.Count - 1; ++k) {
        var p1 = points[k];
        var p2 = points[k + 1];

        area += (p1.Y + p2.Y) * (p1.X - p2.X);
    }

    area = Math.Abs(area) / 2;

    // pick's theorem
    result = area - ((points.Count / 2) - 1);

    return result;
}

var start = Stopwatch.StartNew();
var result = PartOne();
start.Stop();
Console.WriteLine($"// Result: {result} in {start.Elapsed}");
// Result: 6738 in 00:00:00.0111715

start = Stopwatch.StartNew();
result = PartTwo();
start.Stop();
Console.WriteLine($"// Result: {result} in {start.Elapsed}");
// Result: 579 in 00:00:00.0055626