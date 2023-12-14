#include "utils.hpp"
#include <regex>
#include <tuple>

#define debug 0 && std::clog

int minlen(auto nums) {
    if (nums.size() == 0) return 0;
    int total = 0;
    for (auto &n: nums) total += 1+n;
    return total-1;
}

uint nmax(auto numsbegin, auto numsend) {
    uint max = 0;
    for (auto n = numsbegin; n != numsend; ++n) if (*n>max) max = *n;
    return max;
}

ulong solve(std::vector<char>::iterator specbegin,
        const std::vector<char>::iterator specend,
        std::vector<uint>::iterator numsbegin,
        const std::vector<uint>::iterator numsend,
        const int minlen) {
    debug << std::tuple(specbegin, specend) << " : "
        << std::tuple(numsbegin, numsend) << " (" << minlen << ") ->" << std::endl;
    auto i = specbegin;
    while (i != specend && *i == '.') ++i;
    if (std::distance(i, specend) < minlen) {
        debug << std::tuple(specbegin, specend) << " : "
            << std::tuple(numsbegin, numsend) << " -> 0 (num length guard)" << std::endl;
        return 0;
    }
    if (numsbegin == numsend) {
        auto ret = 1;
        while (i != specend) {
            if (*i=='#') {
                ret = 0;
                break;
            }
            ++i;
        }
        debug << std::tuple(specbegin, specend) << " : "
            << std::tuple(numsbegin, numsend) << " -> " << ret
            << " (has '#' when empty nums)" << std::endl;
        return ret;
    }
    auto begin = i;
    // 1: 0 -> 0
    // 2: 0 -> 1
    // 3: 0 -> 1
    // 4: 0 -> 2
    std::advance(i, std::distance(i, specend)/2);
    int dev = 0;
    // 2: 1 -> end
    // 3: 1 -> 2 -> 0 -> end
    // 4: 2 -> 3 -> 1 -> end
    while (i!=begin && i!=specend && *i == '#') {
        dev = (dev>0?-1:1)*(std::abs(dev)+1);
        i+=dev;
    }
    if (i == specend || i == begin) {
        ulong ret = 0;
        if (*begin == '#') {
            ret = (numsbegin + 1 == numsend && *numsbegin == std::distance(begin, specend));
            debug << std::tuple(specbegin, specend) << " : "
                << std::tuple(numsbegin, numsend) << " -> " << ret
                << " ('#' group)" << std::endl;
            return ret;
        }
        assert(*begin == '?');
        ret = numsbegin + 1 == numsend
              && (*numsbegin == std::distance(begin, specend)
                 || *numsbegin == std::distance(begin, specend)-1);
        debug << std::tuple(specbegin, specend) << " : "
            << std::tuple(numsbegin, numsend) << " -> " << ret
            << " ('?' followed by '#' group)" << std::endl;
        return ret;
    }
    if (std::abs(dev) > nmax(numsbegin, numsend)) {
        debug << std::tuple(specbegin, specend) << " : "
            << std::tuple(numsbegin, numsend)
            << " -> 0 ('#' group longer than max num)" << std::endl;
        return 0;
    }
    ulong total = 0;
    char orig = *i;
    if (*i == '?') {
        debug << std::distance(specbegin, i) << " try '#'" << std::endl;
        *i = '#';
        total += solve(specbegin, specend, numsbegin, numsend, minlen);
        debug << std::distance(specbegin, i) << " try '.'" << std::endl;
        *i = '.';
    }
    auto split = std::distance(specbegin, i);
    int minlen1 = -1;
    for (auto num = numsbegin; num != numsend; ++num) {
        auto half = solve(begin, i, numsbegin, num, minlen1);
        if (half) total += half * solve(i, specend, num, numsend, minlen-minlen1-1);
        minlen1 += 1 + (int)*num;
    }
    auto half = solve(i, specend, numsend, numsend, 0);
    if (half) total += half * solve(begin, i, numsbegin, numsend, minlen);
    debug << std::distance(specbegin, i) << " revert " << orig << std::endl;
    *i = orig;
    debug << std::tuple(specbegin, specend) << " : "
        << std::tuple(numsbegin, numsend) << " -> " << total
        << " (compound)" << std::endl;
    return total;
}

int main() {
    std::string line;
    ulong total1=0, total2=0;
    while (std::getline(std::cin, line)) {
        std::regex re(R"(([?.#]+) ([0-9,]+))");
        std::smatch m;
        std::regex_match(line, m, re);
        line = m[1].str();
        std::vector<char> spec = std::vector(line.begin(), line.end());
        std::string intspec = m[2].str();
        std::vector<uint> nums = utils::ints<uint>(intspec);
        std::clog << spec << " : " << nums << std::endl;
        total1 += solve(spec.begin(), spec.end(), nums.begin(), nums.end(), minlen(nums));
        std::vector<char> spec2 = std::vector(spec);
        std::vector<uint> nums2 = std::vector(nums);
        nums2.reserve(5*nums.size());
        for (int i=0; i<4; i++) {
            spec2.push_back('?');
            for (auto &i: spec) spec2.push_back(i);
            for (auto &i: nums) nums2.push_back(i);
        }
        total2 += solve(spec2.begin(), spec2.end(), nums2.begin(), nums2.end(), minlen(nums2));
    }
    std::cout << total1 << std::endl << total2 << std::endl;
}
