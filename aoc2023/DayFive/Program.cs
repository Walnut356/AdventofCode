using System;
using System.Collections;
using System.IO;
using static Utils.Utils;
using DayFive;
using System.Diagnostics;


Int64 PartOne()
{
    using StreamReader stream = GetStream("DayFive");

    string seed_line = stream.ReadLine()!;
    string[] lines = GetRemainingLines(stream);
    IEnumerator<string> line_iter = lines.AsEnumerable().GetEnumerator();
    line_iter.MoveNext();

    // a little bruteforce-y but it is what it is.
    ExpectBytes("seed-to-soil map:", line_iter.Current);
    line_iter.MoveNext();
    List<Map> seed_soil = [];
    while (line_iter.Current != "soil-to-fertilizer map:")
    {
        seed_soil.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> soil_fert = [];
    while (line_iter.Current != "fertilizer-to-water map:")
    {
        soil_fert.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> fert_water = [];
    while (line_iter.Current != "water-to-light map:")
    {
        fert_water.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> water_light = [];
    while (line_iter.Current != "light-to-temperature map:")
    {
        water_light.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> light_temp = [];
    while (line_iter.Current != "temperature-to-humidity map:")
    {
        light_temp.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> temp_hum = [];
    while (line_iter.Current != "humidity-to-location map:")
    {
        temp_hum.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> hum_loc = [];
    while (line_iter.MoveNext())
    {
        hum_loc.Add(new(line_iter.Current));
    }

    IEnumerator<string> seeds = GetTokensIter(seed_line);
    //
    seeds.MoveNext();
    ExpectBytes("seeds:", seeds.Current);

    Int64 smallest_loc = Int64.MaxValue;

    while (Next(seeds, out string seed_str))
    {
        Int64 seed = Int64.Parse(seed_str);

        Int64 loc = seed;
        foreach (List<Map> map_list in new List<List<Map>>([seed_soil, soil_fert, fert_water, water_light, light_temp, temp_hum, hum_loc]))
        {
            Int64 orig = loc;
            foreach (Map m in map_list)
            {
                loc = m.MapVal(orig);
                if (loc != orig)
                {
                    break;
                }
            }
        }

        smallest_loc = Math.Min(smallest_loc, loc);
    }

    return smallest_loc;
}

Int64 PartTwo()
{
    using StreamReader stream = GetStream("DayFive");

    string seed_line = stream.ReadLine()!;
    string[] lines = GetRemainingLines(stream);
    IEnumerator<string> line_iter = lines.AsEnumerable().GetEnumerator();
    line_iter.MoveNext();

    // a little bruteforce-y but it is what it is.
    ExpectBytes("seed-to-soil map:", line_iter.Current);
    line_iter.MoveNext();
    List<Map> seed_soil = [];
    while (line_iter.Current != "soil-to-fertilizer map:")
    {
        seed_soil.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> soil_fert = [];
    while (line_iter.Current != "fertilizer-to-water map:")
    {
        soil_fert.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> fert_water = [];
    while (line_iter.Current != "water-to-light map:")
    {
        fert_water.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> water_light = [];
    while (line_iter.Current != "light-to-temperature map:")
    {
        water_light.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> light_temp = [];
    while (line_iter.Current != "temperature-to-humidity map:")
    {
        light_temp.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> temp_hum = [];
    while (line_iter.Current != "humidity-to-location map:")
    {
        temp_hum.Add(new(line_iter.Current));

        line_iter.MoveNext();
    }

    line_iter.MoveNext();
    List<Map> hum_loc = [];
    while (line_iter.MoveNext())
    {
        hum_loc.Add(new(line_iter.Current));
    }

    IEnumerator<string> seeds = GetTokensIter(seed_line);
    //
    seeds.MoveNext();
    ExpectBytes("seeds:", seeds.Current);

    Int64 smallest_loc = Int64.MaxValue;

    // List<(Int64, Int64)>

    while (Next(seeds, out string seed_str))
    {
        Int64 seed_start = Int64.Parse(seed_str);
        Next(seeds, out string len_str);
        Int64 seed_end = Int64.Parse(len_str) + seed_start;
        for (Int64 i = seed_start; i < seed_end; ++i)
        {
            // Console.WriteLine($"Handling value {i}");
            Int64 loc = i;
            foreach (List<Map> map_list in new List<List<Map>>([seed_soil, soil_fert, fert_water, water_light, light_temp, temp_hum, hum_loc]))
            {
                Int64 orig = loc;
                foreach (Map m in map_list)
                {
                    loc = m.MapVal(orig);
                    if (loc != orig)
                    {
                        break;
                    }
                }
            }

            smallest_loc = Math.Min(smallest_loc, loc);
            // Interlocked.Exchange(ref smallest_loc, result);
        };
    }


    return smallest_loc;
}

var start = Stopwatch.StartNew();
var result = PartOne();
start.Stop();
Console.WriteLine($"Result: {result} in {start.Elapsed.Milliseconds}ms");
// got 346433842

start = Stopwatch.StartNew();
result = PartTwo();
start.Stop();
Console.WriteLine($"Result: {result} in {start.Elapsed.Milliseconds}ms");
// got 60294664 in ~2 hours =)