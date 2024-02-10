namespace Utils;

public static class Utils
{
    public static void ExpectBytes(string expected, string got)
    {
        if (got != expected)
        {
            throw new InvalidDataException($"Expected {expected}, got {got}");
        }
    }

    public static bool Next(IEnumerator<string> tokens, out string token)
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

    public static StreamReader GetStream(string day)
    {
        return new($"../../../test_data.txt");
    }

    public static string[] GetRemainingLines(StreamReader stream)
    {
        return stream.ReadToEnd().Split(["\r\n", "\n"], StringSplitOptions.RemoveEmptyEntries);
    }

    public static IEnumerator<string> GetTokensIter(string line)
    {
        return line.Split(' ', StringSplitOptions.RemoveEmptyEntries).AsEnumerable().GetEnumerator();
    }

    public static string[] GetTokens(string line) {
        return line.Split(' ', StringSplitOptions.RemoveEmptyEntries);
    }
}
