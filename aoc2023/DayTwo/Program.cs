using System;
using System.IO;

void ExpectBytes(string expected, string got) {
    if (got != expected) {
        throw new InvalidDataException($"Expected {expected}, got {got}");
    }
}

// am i stupid? Why doesn't this exist?
bool Next(IEnumerator<string> tokens, out string token) {
    bool result = tokens.MoveNext();

    if (result)
    {
        token = tokens.Current;
    } else {
        token = "";
    }

    return result;
}

int PartOne()
{
    const int RED_MAX = 12;
    const int GREEN_MAX = 13;
    const int BLUE_MAX = 14;

    using StreamReader stream = new("../../../test_data.txt");

    int total = 0;

    string? line = stream.ReadLine();

    while (line != null)
    {
        IEnumerator<string> tokens = line.Split([' ']).AsEnumerable().GetEnumerator();
        tokens.MoveNext();
        ExpectBytes("Game", tokens.Current);
        tokens.MoveNext();
        int game_number = int.Parse(tokens.Current[0..^1]); // ignore the trailing colon

        bool too_high = false;

        string token;
        while (Next(tokens, out token)) {
            // initial token after the "header" will be a number
            int count = int.Parse(token);

            if (count > BLUE_MAX) {
                too_high = true;
                break;
            }

            // token after number will be the color, of which the first letter is unique
            Next(tokens, out token);

            too_high = token[0] switch
            {
                'r' => count > RED_MAX,
                'g' => count > GREEN_MAX,
                'b' => count > BLUE_MAX,
                _ => throw new InvalidDataException($"Expected red, green, or blue, got {token}"),
            };

            if (too_high) {
                break;
            }
        }
        line = stream.ReadLine();

        if (too_high) {
            continue;
        }

        total += game_number;

    }
        return total;
}

int PartTwo()
{
    using StreamReader stream = new("../../../test_data.txt");

    int total = 0;

    string? line = stream.ReadLine();

    while (line != null)
    {
        int red = 0;
        int blue = 0;
        int green = 0;

        IEnumerator<string> tokens = line.Split([' ']).AsEnumerable().GetEnumerator();
        tokens.MoveNext();
        ExpectBytes("Game", tokens.Current);
        tokens.MoveNext();
        int game_number = int.Parse(tokens.Current[0..^1]); // ignore the trailing colon

        string token;
        while (Next(tokens, out token))
        {
            // initial token after the "header" will be a number
            int count = int.Parse(token);

            // token after number will be the color, of which the first letter is unique
            Next(tokens, out token);

            switch (token[0])
            {
                case 'r':
                    if (count > red) {
                        red = count;
                    }
                    break;
                case 'g':
                    if (count > green)
                    {
                        green = count;
                    }
                    break;
                case 'b':
                    if (count > blue)
                    {
                        blue = count;
                    }
                    break;
                default:
                    throw new InvalidDataException($"Expected red, green, or blue, got {token}");
            };
        }
        total += red * green * blue;
        line = stream.ReadLine();
    }
    return total;
}

Console.WriteLine(PartOne()); // got 2449
Console.WriteLine(PartTwo()); // got 63981
