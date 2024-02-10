using System;
using System.Collections;
using System.IO;
using static Utils.Utils;
using System.Diagnostics;
using System.Runtime.InteropServices;

// My input's max symbol length is 20, meaning all of the operations I need to do can fit in a
// u32 which is lovely

// # ascii value 35
// . ascii value 46
// ? ascii value 63

bool is_valid(UInt64[] counts, UInt64 test_val, int length)
{
    UInt64? working = null;
    var count_idx = 0;

    // state machine
    for (int _i = 0; _i < length; ++_i)
    {

        if ((test_val & 1) == 0)
        {
            // if the current bit is 0 and we had an active set of 1's
            if (working is not null)
            {   // reset the state if the set of 1's was the correct length and iterate to the next comparison value
                if (working == counts[count_idx])
                {
                    working = null;
                    count_idx += 1;
                }
                else
                {   // otherwise, we got a 0 too early and this isn't valid
                    return false;
                }
            }
        }
        else
        {
            // if the current bit is a 1 and we have active 1's
            if (working is not null)
            {   // add to our count. If it exceeds where we're supposed to be at, return false
                working += 1;
                if (working > counts[count_idx])
                {
                    return false;
                }
            }
            else
            {   // otherwise this is the first 1 in a row, thus we set working
                working = 1;
            }
        }

        test_val >>= 1;
    }

    // this catches the final grouping in case it was missed in the loop above
    if (working is not null)
    {
        return working == counts[count_idx];
    }
    else
    {
        return count_idx == counts.Length;
    }
}

UInt64 PartOne()
{
    UInt64 result = 0;
    using StreamReader stream = GetStream("Twelve");
    var lines = GetRemainingLines(stream);

    foreach (var line in lines)
    {
        var tokens = GetTokens(line);
        var springs = tokens[0].AsSpan();
        var counts = tokens[1];

        var count_vec = counts.Split(',').Select(UInt64.Parse).ToArray();
        // you... you can't call Sum() on ulongs...? What is this language.
        var working_springs = count_vec.AsEnumerable().Aggregate((a, c) => a + c);

        // we only have 2 states so binary representation makes sense
        UInt64 spring_num = 0;
        List<UInt64> q_locs = [];

        for (int i = springs.Length - 1; i >= 0; --i)
        {
            if (springs[i] == '?')
            {
                // record the location of question marks for future use
                // could have just itered backwards to avoid the bit of math but eh.
                q_locs.Add((UInt64)i);
            }

            // this hack is cute so i'm keeping it in. It's also maybe a little faster
            //
            // # = 0b0010_0011
            // . = 0b0010_1110
            // ? = 0b0011_1111
            // bit 3 is the opposite of what we want, so negating it gives us our answer.
            spring_num |= ~((UInt64)springs[i] >> 2) & 1;
            spring_num <<= 1;
        }

        UInt64 given_springs = ulong.PopCount(spring_num);

        // q_vals will let us traverse every possible combination of values simply by adding 1 in a
        // loop so long as we can translate q_test's bits into replacement questionmark values.
        UInt64 q_vals = 0;
        while (q_vals < (UInt64)(1 << q_locs.Count))
        {
            if (ulong.PopCount(q_vals) + given_springs != working_springs)
            {
                q_vals += 1;
                continue;
            }
            var test_line = spring_num;
            for (int i = 0; i < q_locs.Count; ++i)
            {
                UInt64 bit = (q_vals >> i) & 1;
                // all question marks are treated as 0's, so replacing it with a dot is a nop
                test_line |= bit << (int)((ulong)springs.Length - q_locs[i]);
            }
            if (is_valid(count_vec, test_line, springs.Length))
            {
                result += 1;
            }

            q_vals += 1;
        }
    }

    return result;
}

int PartTwo()
{
    int result = 0;
    using StreamReader stream = GetStream("Twelve");

    return result;
}

var start = Stopwatch.StartNew();
var r1 = PartOne();
start.Stop();
Console.WriteLine($"// Result: {r1} in {start.Elapsed}");


start = Stopwatch.StartNew();
var r2 = PartTwo();
start.Stop();
Console.WriteLine($"// Result: {r2} in {start.Elapsed}");
