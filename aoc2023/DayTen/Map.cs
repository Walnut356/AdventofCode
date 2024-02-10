namespace Map;

// Overengineered maybe, but I'm getting more used to how C# handles classes and such so that's nice

public enum Pipe
{
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    None,
    Start,
}

public enum Direction
{
    Up,
    Right,
    Down,
    Left,
}

public record struct Tile
{
    public Pipe Pipe;
    public int X, Y;

    public static Pipe GetPipe(char c)
    {
        return c switch
        {
            '|' => Pipe.UpDown,
            '-' => Pipe.LeftRight,
            'L' => Pipe.UpRight,
            'J' => Pipe.UpLeft,
            '7' => Pipe.DownLeft,
            'F' => Pipe.DownRight,
            '.' => Pipe.None,
            'S' => Pipe.Start,
            _ => throw new NotImplementedException(),
        };
    }

    public Tile(ref string[] l, int x, int y)
    {
        Pipe = GetPipe(l[y][x]);
        X = x;
        Y = y;
    }

    ///<summary>
    ///
    /// </summary>
    /// <param name="l"></param>
    /// <returns>In clockwise order (Up -> Right -> Down -> Left) If the tile is Pipe.None, returns itself.</returns>
    /// <exception cref="NotImplementedException"></exception>
    public (Tile, Tile) ConnectsTo(ref string[] lines)
    {
        if (Pipe == Pipe.Start)
        {
            List<Tile> tiles = [];

            if (Y - 1 >= 0)
            {
                var u = new Tile(ref lines, X, Y - 1);
                if (u.Pipe == Pipe.UpDown || u.Pipe == Pipe.DownLeft || u.Pipe == Pipe.DownRight)
                {
                    tiles.Add(u);
                }
            }
            if (X + 1 < lines[0].Length)
            {
                var r = new Tile(ref lines, X + 1, Y);
                if (r.Pipe == Pipe.UpLeft || r.Pipe == Pipe.DownLeft || r.Pipe == Pipe.LeftRight)
                {
                    tiles.Add(r);
                }
            }
            if (tiles.Count == 2)
            {
                return (tiles[0], tiles[1]);
            }
            if (Y + 1 < lines.Length)
            {
                var d = new Tile(ref lines, X, Y + 1);
                if (d.Pipe == Pipe.UpDown || d.Pipe == Pipe.UpLeft || d.Pipe == Pipe.UpRight)
                {
                    tiles.Add(d);
                }
            }
            if (tiles.Count == 2)
            {
                return (tiles[0], tiles[1]);
            }
            if (X - 1 >= 0)
            {
                var l = new Tile(ref lines, X - 1, Y);
                if (l.Pipe == Pipe.LeftRight || l.Pipe == Pipe.DownRight || l.Pipe == Pipe.UpRight)
                {
                    tiles.Add(l);
                }
            }

            return (tiles[0], tiles[1]);
        }


        return Pipe switch
        {
            Pipe.UpDown => (new Tile(ref lines, X, Y - 1), new Tile(ref lines, X, Y + 1)),
            Pipe.LeftRight => (new Tile(ref lines, X + 1, Y), new Tile(ref lines, X - 1, Y)),
            Pipe.UpRight => (new Tile(ref lines, X, Y - 1), new Tile(ref lines, X + 1, Y)),
            Pipe.UpLeft => (new Tile(ref lines, X, Y - 1), new Tile(ref lines, X - 1, Y)),
            Pipe.DownLeft => (new Tile(ref lines, X, Y + 1), new Tile(ref lines, X - 1, Y)),
            Pipe.DownRight => (new Tile(ref lines, X + 1, Y), new Tile(ref lines, X, Y + 1)),
            Pipe.None => (this, this),
        };
    }

    public Tile GetNext(ref string[] lines, Tile prev)
    {
        (var t1, var t2) = ConnectsTo(ref lines);
        return t1.X == prev.X & t1.Y == prev.Y ? t2 : t1;
    }
}