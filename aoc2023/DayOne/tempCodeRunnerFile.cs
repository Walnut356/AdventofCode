
// int PartOne()
// {
//     StreamReader stream = new StreamReader("G:/Coding/My Projects/Learning/adventofcode23/DayOne/test_data.txt");

//     int total = 0;

//     string? line = stream.ReadLine();

//     while (line != null)
//     {
//         // find the first digit via ASCII code. After subtracting the ascii code, we can multiply by 10
//         // and add to the total so there's no need to store intermediate values
//         foreach (byte b in line)
//         {
//             if (b >= 48 && b < 58)
//             {
//                 total += (b - 48) * 10;
//                 break;
//             }
//         }

//         for (int i = line.Length - 1; i >= 0; --i)
//         {
//             if (line[i] >= 48 && line[i] < 58)
//             {
//                 total += line[i] - 48;
//                 break;
//             }
//         }


//         line = stream.ReadLine();
//     }

//     return total;
// }
// /* ------------------------------------------ Part Two ------------------------------------------ */
// int PartTwo()
// {
//     StreamReader stream = new StreamReader("G:/Coding/My Projects/Learning/adventofcode23/DayOne/test_data.txt");

//     int total = 0;

//     string? line = stream.ReadLine();

//     while (line != null)
//     {
//         for (int i = 0; i < line.Length; ++i)
//         {
//             if (line[i] >= 48 && line[i] < 58)
//             {
//                 total += (line[i] - 48) * 10;
//                 break;
//             }

//             // extract the current window
//             string temp = line[i..];


//             // Digits have 3, 4, or 5 total characters. We can just TryParse them into ints since
//             // there's such little variation
//             // a little lazy but oh well
//             int result;
//             if (
//                 (temp.Length >= 3 && int.TryParse(temp[0..4], out result))
//                 || (temp.Length >= 4 && int.TryParse(temp[0..5], out result))
//                 || (temp.Length >= 5 && int.TryParse(temp[0..6], out result))
//             )
//             {
//                 Console.WriteLine(result * 10);
//                 total += result * 10;
//                 break;
//             }
//         }

//         for (int i = line.Length - 1; i >= 0; --i)
//         {
//             if (line[i] >= 48 && line[i] < 58)
//             {
//                 total += (line[i] - 48) * 10;
//                 break;
//             }

//             // extract the current window
//             string temp = line[i..];


//             // Only difference here is that we need to do bounds checking
//             int result;
//             if ((temp.Length >= 3 && int.TryParse(temp[0..4], out result)) || (temp.Length >= 4 && int.TryParse(temp[0..5], out result)) || (temp.Length >= 5 && int.TryParse(temp[0..6], out result)))
//             {
//                 Console.WriteLine(result * 10);
//                 total += result;
//                 break;
//             }
//         }


//         line = stream.ReadLine();
//     }
//     return total;
// }
