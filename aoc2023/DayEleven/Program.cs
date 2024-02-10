using System;
using System.Collections;
using System.IO;
using static Utils.Utils;
using System.Diagnostics;
using static System.Math;
using System.Runtime.InteropServices;

int Distance((int x, int y) p1, (int x, int y) p2) {
    return Abs(p1.x - p2.x) + Abs(p1.y - p2.y);
}

Int64 Distance64((Int64 x, Int64 y) p1, (Int64 x, Int64 y) p2)
{
    return Abs(p1.x - p2.x) + Abs(p1.y - p2.y);
}

int PartOne()
{
    int result = 0;
    using StreamReader stream = GetStream("Eleven");
    string[] lines = GetRemainingLines(stream);

    List<(int x, int y)> targets = [];

    List<int> empty_rows = [];
    List<int> empty_cols = [];
    for (int i = 0; i < lines.Length; ++i) {
        if (!lines[i].Contains('#'))
        {
            empty_rows.Add(i);
        }
        for (int j = 0; j < lines[0].Length; ++j) {
            if(lines[i][j] == '#') {
                targets.Add((j, i));
            }
        }
    }

    // a little awkward but prevents me having to do a column-wise loop
    HashSet<int> cols = [];
    foreach(var t in targets) {
        cols.Add(t.x);
    }

    for(int i = 0; i < lines[0].Length; ++i) {
        if(!cols.Contains(i)) {
            empty_cols.Add(i);
        }
    }

    // is there really not an easier way to get a reference to a list element...?
    var why_do_i_have_to_do_this = CollectionsMarshal.AsSpan(targets);

    foreach(ref var p in why_do_i_have_to_do_this) {
        // store the initial values so that it can't "grow past" a column that should expand ahead of it
        int x = p.x;
        int y = p.y;
        foreach(var c in empty_cols) {
            if(c < x) {
                p.x += 1;
            }
        }
        foreach(var r in empty_rows) {
            if(r < y) {
                p.y += 1;
            }
        }
    }

    var comparisons = 0;
    for (int i = 0; i < targets.Count; ++i)
    {
        // assigning j to i + 1 means we never get any duplicate comparisons
        for (int j = i + 1; j < targets.Count; ++j)
        {
            result += Distance(why_do_i_have_to_do_this[i], why_do_i_have_to_do_this[j]);
            comparisons += 1;
        }
    }

    return result;
}

Int64 PartTwo()
{
    Int64 result = 0;
    using StreamReader stream = GetStream("Eleven");
    string[] lines = GetRemainingLines(stream);

    List<(Int64 x, Int64 y)> targets = [];

    List<Int64> empty_rows = [];
    List<Int64> empty_cols = [];
    for (int i = 0; i < lines.Length; ++i)
    {
        if (!lines[i].Contains('#'))
        {
            empty_rows.Add(i);
        }
        for (int j = 0; j < lines[0].Length; ++j)
        {
            if (lines[i][j] == '#')
            {
                targets.Add((j, i));
            }
        }
    }

    // a little awkward but prevents me having to do a column-wise loop
    HashSet<Int64> cols = [];
    foreach (var t in targets)
    {
        cols.Add(t.x);
    }

    for (int i = 0; i < lines[0].Length; ++i)
    {
        if (!cols.Contains(i))
        {
            empty_cols.Add(i);
        }
    }

    // is there really not an easier way to get a reference to a list element...?
    var why_do_i_have_to_do_this = CollectionsMarshal.AsSpan(targets);

    foreach (ref var p in why_do_i_have_to_do_this)
    {
        // store the initial values so that it can't "grow past" a column that should expand ahead of it
        Int64 x = p.x;
        Int64 y = p.y;
        foreach (var c in empty_cols)
        {
            if (c < x)
            {
                p.x += 999_999;
            }
        }
        foreach (var r in empty_rows)
        {
            if (r < y)
            {
                p.y += 999_999;
            }
        }
    }

    var comparisons = 0;
    for (int i = 0; i < targets.Count; ++i)
    {
        // assigning j to i + 1 means we never get any duplicate comparisons
        for (int j = i + 1; j < targets.Count; ++j)
        {
            result += Distance64(why_do_i_have_to_do_this[i], why_do_i_have_to_do_this[j]);
            comparisons += 1;
        }
    }

    return result;
}

var start = Stopwatch.StartNew();
var result = PartOne();
start.Stop();
Console.WriteLine($"// Result: {result} in {start.Elapsed}");
// Result: 9947476 in 00:00:00.0111505

start = Stopwatch.StartNew();
var r2 = PartTwo();
start.Stop();
Console.WriteLine($"// Result: {r2} in {start.Elapsed}");
// Result: 519939907614 in 00:00:00.0055228