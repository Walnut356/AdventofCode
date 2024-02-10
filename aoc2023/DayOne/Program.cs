using System;
using System.IO;


int PartOne()
{
    using StreamReader stream = new("../../../test_data.txt");

    int total = 0;


    string? line = stream.ReadLine();

    while (line != null) {
        // find the first digit via ASCII code. After subtracting the ascii code, we can multiply by 10
        // and add to the total so there's no need to store intermediate values
        foreach (byte b in line)
        {
            if (b >= 48 && b < 58)
            {
                total += (b - 48) * 10;
                break;
            }
        }

        for (int i = line.Length - 1; i >= 0; --i)
        {
            if (line[i] >= 48 && line[i] < 58)
            {
                total += line[i] - 48;
                break;
            }
        }


        line = stream.ReadLine();
    }

    return total;
}
/* ------------------------------------------ Part Two ------------------------------------------ */
int PartTwo()
{
    using StreamReader stream = new("../../../test_data.txt");

    int total = 0;

    string? line = stream.ReadLine();

    while (line != null)
    {
        for (int i = 0; i < line.Length; ++i)
        {
            if (line[i] >= 48 && line[i] < 58)
            {
                total += (line[i] - 48) * 10;
                break;
            }

            // extract the current window
            string temp = line[i..];

            // very ugly
            if (temp.StartsWith("zero"))
            {
                break;
            }
            if (temp.StartsWith("one"))
            {
                total += 10;
                break;
            }
            if (temp.StartsWith("two"))
            {
                total += 20;
                break;
            }
            if (temp.StartsWith("three"))
            {
                total += 30;
                break;
            }
            if (temp.StartsWith("four"))
            {
                total += 40;
                break;
            }
            if (temp.StartsWith("five"))
            {
                total += 50;
                break;
            }
            if (temp.StartsWith("six"))
            {
                total += 60;
                break;
            }
            if (temp.StartsWith("seven"))
            {
                total += 70;
                break;
            }
            if (temp.StartsWith("eight"))
            {
                total += 80;
                break;
            }
            if (temp.StartsWith("nine"))
            {
                total += 90;
                break;
            }
        }

        for (int i = line.Length - 1; i >= 0; --i)
        {
            if (line[i] >= 48 && line[i] < 58)
            {
                total += (line[i] - 48) * 1;
                break;
            }

            // extract the current window
            string temp = line[i..];

            // even worse the second time
            if (temp.StartsWith("zero"))
            {
                break;
            }
            if (temp.StartsWith("one"))
            {
                total += 1;
                break;
            }
            if (temp.StartsWith("two"))
            {
                total += 2;
                break;
            }
            if (temp.StartsWith("three"))
            {
                total += 3;
                break;
            }
            if (temp.StartsWith("four"))
            {
                total += 4;
                break;
            }
            if (temp.StartsWith("five"))
            {
                total += 5;
                break;
            }
            if (temp.StartsWith("six"))
            {
                total += 6;
                break;
            }
            if (temp.StartsWith("seven"))
            {
                total += 7;
                break;
            }
            if (temp.StartsWith("eight"))
            {
                total += 8;
                break;
            }
            if (temp.StartsWith("nine"))
            {
                total += 9;
                break;
            }
        }


        line = stream.ReadLine();
    }
    return total;
}

Console.WriteLine(PartOne()); // got 54597
Console.WriteLine(PartTwo()); // got 54504
