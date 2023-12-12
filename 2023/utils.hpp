#include <iostream>
#include <vector>
#include "assert.h"

namespace utils {
    template<class T>
    std::vector<T> ints(const std::string s, char sep = ',') {
        std::vector<T> ret = {};
        size_t prevpos = 0;
        size_t pos = s.find(sep);
        while (pos != std::string::npos) {
            std::string numstr = s.substr(prevpos, pos-prevpos);
            ret.push_back((T)std::stol(numstr));
            prevpos = pos + 1;
            pos = s.find(sep, prevpos);
        }
        if (prevpos < s.length()) {
            ret.push_back((T)std::stol(s.substr(prevpos)));
        }
        return ret;
    }
}

template <class T>
std::ostream& operator<<(std::ostream& os, const std::vector<T>& vs)
{
    bool first = true;
    for (auto& s: vs) {
        if (!first) os << " "; else first = false;
        os << s;
    }
    return os;
}
