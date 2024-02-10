namespace DayFive;

using System.Security;
using static Utils.Utils;

public record struct Map {
    public Int64 src, dest, len;

    public Map(string line) {
        string[] tokens = GetTokens(line);
        dest = Int64.Parse(tokens[0]);
        src = Int64.Parse(tokens[1]);
        len = Int64.Parse(tokens[2]);
    }

    public Range SourceRange() {
        return new Range((Index)src, (Index)(src + len));
    }

    public Range DestRange()
    {
        return new Range((Index)dest, (Index)(dest + len));
    }

    public bool SourceContains(Int64 val) {
        return src <= val && val <= src + len;
    }

    public bool DestContains(Int64 val)
    {
        return dest <= val && val <= dest + len;
    }

    public Int64 MapVal(Int64 val) {
        if(SourceContains(val)) {
            return val - src + dest;
        }
        else {
            return val;
        }
    }
}
