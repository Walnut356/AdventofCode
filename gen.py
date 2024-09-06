# ------------------------------------------------------------------------------------------------ #
#                                               rust                                               #
# ------------------------------------------------------------------------------------------------ #

# for i in range(1, 26):
#     with open(f"./aoc2017/src/day{i}.rs", "w") as f:
#         f.write(
#             f"""#![allow(unused_imports)]

# use crate::get_data;
# use crate::utils::*;


# pub fn p1(data: String) -> usize {{
#     0
# }}

# pub fn p2(data: String) -> usize {{
#     0
# }}

# #[test]
# fn test_d{i}() {{
#     let data = get_data({i});
#     assert_eq!(p1(data.clone()), 0);
#     assert_eq!(p2(data), 0);
# }}"""
#         )

#     with open(f"./aoc2017/test_data/day{i}.txt", 'w') as f:
#         pass

# ------------------------------------------------------------------------------------------------ #
#                                                cpp                                               #
# ------------------------------------------------------------------------------------------------ #

for i in range(1, 26):
    with open(f"./aoc2018/src/day{i}.hpp", "w") as f:
        f.write(
            """
#include "utils.hpp"
#include <string>
#include <vector>
#include <iostream>
#include <map>
#include <tuple>

u64 p1(std::string data) {
    u64 result = 0;

    return result;
}

u64 p2(std::string data) {
    u64 result = 0;

    return result;
}
"""
        )

    with open(f"./aoc2018/test_data/day{i}.txt", 'w') as f:
        pass