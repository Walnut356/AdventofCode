using System;
using System.Collections;
using System.IO;

void ExpectBytes(string expected, string got)
{
    if (got != expected)
    {
        throw new InvalidDataException($"Expected {expected}, got {got}");
    }
}

bool Next(IEnumerator<string> tokens, out string token)
{
    bool result = tokens.MoveNext();

    if (result)
    {
        token = tokens.Current;
    }
    else
    {
        token = "";
    }

    return result;
}

int PartOne()
{
    int total = 0;

    using StreamReader stream = new("../../../test_data.txt");

    string? line = stream.ReadLine();

    while (line != null)
    {
        IEnumerator<string> tokens = line.Split([' ']).AsEnumerable().GetEnumerator();
        tokens.MoveNext();
        ExpectBytes("Card", tokens.Current);

        while (!tokens.Current.EndsWith(":"))
        {   // ignore whitespace and game number
            tokens.MoveNext();
        }

        HashSet<int> winners = [];
        bool checkpoint = false;
        int count = 0;
        string token;

        while (Next(tokens, out token))
        {
            if (token == "|")
            {
                checkpoint = true;
                continue;
            }
            if (token == "")
            {
                continue;
            }
            if (!checkpoint)
            {
                winners.Add(int.Parse(token));
                continue;
            }
            if (winners.Contains(int.Parse(token)))
            {
                count = count == 0 ? 1 : count * 2;
            }

        }

        total += count;

        line = stream.ReadLine();
    }
    return total;
}

Int64 PartTwo()
{

    using StreamReader stream = new("../../../test_data.txt");

    string[] lines = stream.ReadToEnd().Split(["\r\n", "\n"], StringSplitOptions.RemoveEmptyEntries);

    // could use a dictionary, but it's just sequential ints so a table works fine
    List<Int64> cardCounts = [];
    // initialize each card value with one. Assumes that there are no duplicate cards to begin with
    for (int i = 0; i < lines.Length; ++i)
    {
        cardCounts.Add(1);
    }

    foreach (string line in lines)
    {
        IEnumerator<string> tokens = line.Split(' ', StringSplitOptions.RemoveEmptyEntries).AsEnumerable().GetEnumerator();
        tokens.MoveNext();
        ExpectBytes("Card", tokens.Current);

        tokens.MoveNext();
        int gameNumber = int.Parse(tokens.Current[0..^1]);

        HashSet<int> winners = [];
        bool checkpoint = false;
        Int64 count = 0;
        string token;

        while (Next(tokens, out token))
        {
            if (token == "|")
            {
                checkpoint = true;
                continue;
            }
            if (token == "")
            {
                continue;
            }
            if (!checkpoint)
            {
                winners.Add(int.Parse(token));
                continue;
            }
            if (winners.Contains(int.Parse(token)))
            {
                count += 1;
            }

        }

        if (count > 0)
        {
            // game numbers are 1-indexed so we need to account for that when accessing the 0-indexed array
            Int64 cardsWon = cardCounts[gameNumber - 1];
            for (int i = 0; i < count; ++i)
            {
                cardCounts[gameNumber + i] += cardsWon;
            }
        }

    }
    return cardCounts.Sum();
}
Console.WriteLine(PartOne()); // Got 25183
Console.WriteLine(PartTwo()); // Got 5667240