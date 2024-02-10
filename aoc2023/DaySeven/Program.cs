using System;
using System.Collections;
using System.IO;
using static Utils.Utils;
using System.Diagnostics;

int PartOne()
{
    int result = 0;
    using StreamReader stream = GetStream("Seven");

    string[] lines = GetRemainingLines(stream);

    List<Hand> hands = [];

    foreach (string line in lines)
    {
        // all hands are 5 characters long and separated from the bid via a space so we can do it the obtuse way.
        string hand = line[0..5];
        int bid = int.Parse(line[6..]);

        hands.Add(new(hand, bid, 1));
    }

    hands.Sort();

    int ind = 1;
    foreach (Hand h in hands)
    {
        result += h.Bid * ind;
        ind++;
    }

    return result;
}

int PartTwo()
{
    int result = 0;
    using StreamReader stream = GetStream("Seven");

    string[] lines = GetRemainingLines(stream);

    List<Hand> hands = [];

    foreach (string line in lines)
    {
        // this is an attempt to not break the part 1 logic by implementing part 2
        string hand = line[0..5].Replace("J", "j");

        int bid = int.Parse(line[6..]);

        hands.Add(new(hand, bid, 2));
    }

    hands.Sort();

    int ind = 1;
    foreach (Hand h in hands)
    {
        result += h.Bid * ind;
        ind++;
    }

    return result;
}

var start = Stopwatch.StartNew();
var result = PartOne();
start.Stop();
Console.WriteLine($"// Result: {result} in {start.Elapsed}");
// Result: 246163188 in 00:00:00.0308235

start = Stopwatch.StartNew();
result = PartTwo();
start.Stop();
Console.WriteLine($"// Result: {result} in {start.Elapsed}");
// Result: 245794069 in 00:00:00.0062947


// maybe a little overengineered but to be fair this would be trivial to pull off with rust's derive macros and stronger enums =(
enum Card
{
    // this is an attempt to not break the part 1 logic by implementing part 2
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}


enum HandType
{
    High,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

class Hand : IComparable<Hand>
{
    public HandType Type;
    public string Cards;
    public int Bid;
    public int JCount;

    public static Card IntoCard(char c)
    {
        return c switch
        {
            'j' => Card.Joker,
            '2' => Card.Two,
            '3' => Card.Three,
            '4' => Card.Four,
            '5' => Card.Five,
            '6' => Card.Six,
            '7' => Card.Seven,
            '8' => Card.Eight,
            '9' => Card.Nine,
            'T' => Card.Ten,
            'J' => Card.Jack,
            'Q' => Card.Queen,
            'K' => Card.King,
            'A' => Card.Ace,
            _ => throw new InvalidCastException(),
        };
    }

    public Hand(string cards, int bid, int part)
    {
        Cards = cards;
        Bid = bid;
        JCount = 0;

        foreach (char c in cards)
        {
            if (c == 'j')
            {
                JCount += 1;
            }
        }
        if (JCount == 5 && part == 2)
        {
            Type = HandType.FiveKind;
            return;
        }

        var parsed = cards
            .GroupBy(c => c)
            .Select(c => new { Card = c, Count = c.Count() })
            .OrderByDescending(c => c.Count)
            .ToList();


        var thing = parsed[0];
        var t = thing.Card;

        // there's few enough combinations that I think I can just hard code it - though there are also
        // few enough lines in the sample data that brute forcing it probably wouldn't take too long.
        Type = parsed[0].Count switch
        {
            5 => HandType.FiveKind,
            4 => JCount > 0 ? HandType.FiveKind : HandType.FourKind,
            3 => JCount switch
            {
                0 => parsed[1..].Find(x => !x.Card.Equals('j')).Count == 2 ? HandType.FullHouse : HandType.ThreeKind,
                1 => HandType.FourKind, // FH with 1 joker is impossible, so always 4kind
                2 => HandType.FiveKind, // 3kind with 2 jokers is impossible, so always 5kind
                3 => parsed[1..].Find(x => !x.Card.Equals('j')).Count == 2 ? HandType.FiveKind : HandType.FourKind,
            },
            2 => parsed[1..].Find(x => !x.Card.Equals('j')).Count switch
            {
                2 => JCount switch
                {
                    0 => HandType.TwoPair,
                    1 => HandType.FullHouse,
                    2 => HandType.FourKind,
                },
                _ => JCount switch
                { // 1 pair. 1 joker will make a pair into threekind at best
                  // 2 jokers (with a max of 2 of any card, but not 2 pairs) will combine with 1 other card to make 3 kind
                    0 => HandType.Pair,
                    _ => HandType.ThreeKind,
                }
            },
            1 => JCount switch
            {
                0 => HandType.High,
                1 => HandType.Pair,
            },
            _ => throw new Exception(),
        };
    }

    public int CompareTo(Hand? other)
    {
        var comp = Type.CompareTo(other.Type);
        if (comp != 0)
        {
            return comp;
        }

        foreach ((char a, char b) in Cards.Zip(other.Cards))
        {
            var cmp = IntoCard(a).CompareTo(IntoCard(b));
            if (cmp != 0)
            {
                return cmp;
            }
        }

        return 0;
    }
}