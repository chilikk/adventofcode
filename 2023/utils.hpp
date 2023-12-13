#include <iostream>
#include <memory>
#include <span>
#include <vector>
#include "assert.h"

namespace utils {
    struct map {
        std::vector<char> storage;
        uint lines = 0;
        uint linelen = 0;
        operator bool() {return linelen;}
        std::span<char> line(uint n) {
            return std::span(storage).subspan((ulong)n*linelen, linelen);
        }
    };
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
    map read_map() {
        map ret = {};
        std::string line;
        while (std::getline(std::cin, line)) {
            if (ret.linelen == 0) {
                ret.linelen = line.length();
                ret.storage.reserve((ulong)ret.linelen*ret.linelen);
            } else if (line.length() != ret.linelen) {
                break;
            }
            ret.lines++;
            for (auto& c:line) ret.storage.push_back(c);
        }
        assert(ret.storage.size() == ret.lines*ret.linelen);
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
