#include <iostream>
#include <memory>
#include <span>
#include <vector>
#include "assert.h"

namespace utils {
    class map {
        public:
        std::vector<char> storage;
        uint nlines = 0;
        uint linelen = 0;
        operator bool() {return linelen;}
        class iterator {
            public:
                int x = 0;
                int y = 0;
                int xdiff = 0;
                int ydiff = 0;
                map* m = nullptr;
                char& operator *() {
                    if (this->out_of_bounds()) throw("badref");
                    return m->storage.at(m->xyptr(x,y));
                }
                bool operator ==(iterator other) const {return x == other.x && y == other.y;}
                bool operator !=(iterator other) const {return x != other.x || y != other.y;}
                iterator& operator++() { x += xdiff; y += ydiff; return *this; }
                iterator& operator--() { x -= xdiff; y -= ydiff; return *this; }
                bool out_of_bounds() { return (x < 0 || y < 0 || x >= m->nlines || y >= m->linelen); }
                void up() { xdiff = -1; ydiff = 0; }
                void right() { xdiff = 0; ydiff = 1; }
                void down() { xdiff = 1; ydiff = 0; }
                void left() { xdiff = 0; ydiff = -1; }
                void right_once() { y += 1; }
                void left_once() { y -= 1; }
                void up_once() { x -= 1; }
                void down_once() { x += 1; }
                void reverse() { xdiff = -xdiff; ydiff = -ydiff; }
                void turn() {
                    if (xdiff == -1) right();
                    else if (ydiff == 1) down();
                    else if (xdiff == 1) left();
                    else if (ydiff == -1) up();
                }
                void next() {
                    if (xdiff != 0) right_once();
                    else if (ydiff != 0) down_once();
                }
        };
        class line {
            public:
                uint lineno;
                map * m;
                utils::map::iterator begin() { return utils::map::iterator{(int)lineno, 0, 0, 1, m}; }
                utils::map::iterator end()   { return utils::map::iterator{(int)lineno, (int)(m->linelen), 0, 1, m}; }
        };
        class column {
            public:
                uint colno;
                map * m;
                utils::map::iterator begin() { return utils::map::iterator{0, (int)colno, 1, 0, m}; }
                utils::map::iterator end() { return utils::map::iterator{(int)(m->nlines), (int)colno, 1, 0, m}; }
        };
        iterator begin(int direction) {
            if (direction == 0) {
                //down
                return this->get_column(0).begin();
            } else if (direction == 1) {
                //right
                return this->get_line(0).begin();
            } else if (direction == 2) {
                //up
                auto it = this->get_column(0).end();
                it.reverse();
                it.up_once();
                return it;
            } else if (direction == 3) {
                //left
                auto it = this->get_line(0).end();
                it.reverse();
                it.left_once();
                return it;
            }
            assert(false);
        }
        bool operator ==(map other) const {
            auto it1 = storage.begin();
            auto it2 = other.storage.begin();
            while (it1 != storage.end()) {
                if (*it1 != *it2) return false;
                it1++; it2++;
            }
            return true;
        }
        line get_line(uint n) { return line{n, this}; }
        column get_column(uint n) { return column{n, this}; }
        uint xyptr(uint x, uint y) { return x*this->linelen + y; }
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
            ret.nlines++;
            for (auto& c:line) ret.storage.push_back(c);
        }
        assert(ret.storage.size() == ret.nlines*ret.linelen);
        return ret;
    }
}

std::ostream& operator<<(std::ostream& os, utils::map &m) {
    auto i = m.begin(1);
    while (!i.out_of_bounds()) {
        for (;!i.out_of_bounds();++i) os << *i;
        i.y=0; i.down_once();
        os << std::endl;
    }
    return os;
}

//print a pair of iterators (begin, end)
template <class T>
std::ostream& operator<<(std::ostream& os, const std::tuple<T, T> tup) {
    T begin, endx;
    std::tie(begin, endx) = tup;
    T beginx = begin;
    bool first = true;
    while (beginx != endx) {
        if (!first) os << " "; else first = false;
        os << *beginx;
        beginx++;
    }
    return os;
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
