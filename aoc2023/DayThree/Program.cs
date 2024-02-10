using System;
using System.IO;
using Parts;

// could probably do this without structs, but I want to get used to C# as a whole so I'm gonna just use structs

int PartOne()
{
    int total = 0;

    using StreamReader stream = new("../../../test_data.txt");


    string? line = stream.ReadLine();

    List<Part> parts = [];
    List<Symbol> symbols = [];

    int i = 0;
    while (line != null)
    {
        int val = 0;
        int j = 0;
        int start = 0;
        foreach (char c in line)
        {
            if (c >= 48 && c < 58)
            { // handle numbers, open an "event" if necessary
                if (val == 0)
                {
                    start = j;
                }
                val = (val * 10) + (c - 48);
            }
            else
            { // handle blank space, close "event" if necessary
                if (val > 0)
                {
                    parts.Add(new Part(val, i, start, j - start));
                    start = 0;
                    val = 0;
                }
                if (c != '.')
                {
                    symbols.Add(new Symbol(c, i, j));
                }
            }
            j += 1;
        }
        if (val > 0)
        { // account for numbers that end at the end of the line
            parts.Add(new Part(val, i, start, j - start));
        }

        line = stream.ReadLine();
        i += 1;
    }

    foreach (Part p in parts)
    {
        foreach (Symbol s in symbols)
        {   // if the symbol's position is between the start and end (+- 1 to account for diagonals)
            // then the part is valid and we can add it on. Bounds checking is unnecessary as we're
            // not indexing, and we're using signed integers so no overflow shenanigans can occur.
            if (p.Row - 1 <= s.Row && s.Row <= p.Row + 1
            && p.Col - 1 <= s.Col && s.Col <= p.Col + p.Len)
            {
                total += p.Val;
                break;
            }
            // Symbols are parsed sequentially, so once we're out of range, none of the symbols will
            // match. Saves a bit of processing time.
            if (s.Row > p.Row + 1)
            {
                break;
            }
        }
    }

    return total;
}

int PartTwo()
{
    int total = 0;

    using StreamReader stream = new("../../../test_data.txt");


    string? line = stream.ReadLine();

    List<Part> parts = [];
    List<Symbol> symbols = [];

    int i = 0;
    while (line != null)
    {
        int val = 0;
        int j = 0;
        int start = 0;
        foreach (char c in line)
        {
            if (c >= 48 && c < 58)
            { // handle numbers, open an "event" if necessary
                if (val == 0)
                {
                    start = j;
                }
                val = (val * 10) + (c - 48);
            }
            else
            { // handle blank space, close "event" if necessary
                if (val > 0)
                {
                    parts.Add(new Part(val, i, start, j - start));
                    start = 0;
                    val = 0;
                }
                if (c == '*')
                {
                    symbols.Add(new Symbol(c, i, j));
                }
            }
            j += 1;
        }
        if (val > 0)
        { // account for numbers that end at the end of the line
            parts.Add(new Part(val, i, start, j - start));
        }

        line = stream.ReadLine();
        i += 1;
    }


    foreach (Symbol s in symbols)
    {
        int intersections = 0;
        int temp = 0;
        foreach (Part p in parts)
        {   // if the symbol's position is between the start and end (+- 1 to account for diagonals)
            // then the part is valid and we can add it on. Bounds checking is unnecessary as we're
            // not indexing, and we're using signed integers so no overflow shenanigans can occur.
            if (p.Row - 1 <= s.Row && s.Row <= p.Row + 1
            && p.Col - 1 <= s.Col && s.Col <= p.Col + p.Len)
            {
                intersections++;
                switch (intersections) {
                    case 1:
                        temp = p.Val;
                        break;
                    case 2:
                        temp *= p.Val;
                        break;
                    default:
                        break;
                }
            }
        }

        if (intersections == 2) {
            total += temp;
        }
    }

    return total;
}

Console.WriteLine(PartOne()); // Got 525181
Console.WriteLine(PartTwo()); // Got 84289137