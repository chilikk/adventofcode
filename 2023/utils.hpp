#include <iostream>
#include <array>
#include <list>
#include <map>
#include <memory>
#include <set>
#include <span>
#include <tuple>
#include <vector>
#include <regex>
#include <algorithm>
#include <numeric>
#include <cassert>
#include <queue>

namespace utils {
    template<class T>
    class map {
        public:
        std::vector<T> storage;
        uint nlines = 0;
        uint linelen = 0;
        operator bool() {return linelen;}
        enum direction {DOWN, RIGHT, UP, LEFT};
        template <class M>
        class iterator {
                class badref {
                    public:
                    int x = 0;
                    int y = 0;
                    badref(int x, int y): x(x), y(y) {
                        std::clog << "badref: " << x << ":" << y << std::endl;
                    }
                };
            public:
                int x = 0;
                int y = 0;
                int xdiff = 0;
                int ydiff = 0;
                M* m = nullptr;
                bool wrap = false;
                int wx() {return wrap?w(x, m->nlines):x;};
                int wy() {return wrap?w(y, m->linelen):y;};
                inline int w(int i, int mod) { return ((i%mod)+mod)%mod; }
                const std::vector<T>::reference operator *() const {
                    if (this->out_of_bounds()) throw(badref{x, y});
                    return m->at(wx(),wy());
                }
                std::vector<T>::reference operator *() {
                    if (this->out_of_bounds()) throw(badref{x, y});
                    return m->at(wx(),wy());
                }
                bool operator ==(iterator other) const {return x == other.x && y == other.y;}
                bool operator !=(iterator other) const {return x != other.x || y != other.y;}
                iterator& operator++() { x += xdiff; y += ydiff; return *this; }
                iterator& operator+=(int i) { x += i*xdiff; y += i*ydiff; return *this; }
                iterator& operator--() { x -= xdiff; y -= ydiff; return *this; }
                bool out_of_bounds() {
                    return !wrap && (wx() < 0 || wy() < 0 || wx() >= m->nlines || wy() >= m->linelen);
                }
                void up() { set_direction(utils::map<T>::UP); }
                void right() { set_direction(utils::map<T>::RIGHT); }
                void down() { set_direction(utils::map<T>::DOWN); }
                void left() { set_direction(utils::map<T>::LEFT); }
                void set_direction(utils::map<T>::direction dir) {
                    switch (dir) {
                        case UP: xdiff = -1; ydiff = 0; break;
                        case RIGHT: xdiff = 0; ydiff = 1; break;
                        case DOWN: xdiff = 1; ydiff = 0; break;
                        case LEFT: xdiff = 0; ydiff = -1; break;
                    }
                }
                void reverse() { xdiff = -xdiff; ydiff = -ydiff; }
                void next() {
                    if (xdiff != 0) ++y;
                    else if (ydiff != 0) ++x;
                }
                void prev() {
                    if (xdiff != 0) --y;
                    else if (ydiff != 0) --x;
                    }
                uint id() {
                    if (xdiff == 0) return x;
                    if (ydiff == 0) return y;
                    assert(false);
                }
                utils::map<T>::direction direction() const {
                    if (xdiff == -1) return utils::map<T>::UP;
                    else if (ydiff == 1) return utils::map<T>::RIGHT;
                    else if (xdiff == 1) return utils::map<T>::DOWN;
                    else if (ydiff == -1) return utils::map<T>::LEFT;
                    assert(false);
                }
        };
        template<class M>
        class line {
            public:
                uint lineno;
                M * m;
                utils::map<T>::iterator<M> begin() {
                    return utils::map<T>::iterator<M>{(int)lineno, 0, 0, 1, m, false};
                }
                utils::map<T>::iterator<M> end() {
                    return utils::map<T>::iterator<M>{(int)lineno, (int)(m->linelen), 0, 1, m, false};
                }
        };
        template<class M>
        class column {
            public:
                uint colno;
                M * m;
                utils::map<T>::iterator<M> begin() {
                    return utils::map<T>::iterator<M>{0, (int)colno, 1, 0, m, false};
                }
                utils::map<T>::iterator<M> end() {
                    return utils::map<T>::iterator<M>{(int)(m->nlines), (int)colno, 1, 0, m, false};
                }
        };
        iterator<map> begin(direction direction) {
            iterator<map> it;
            switch (direction) {
                case DOWN:
                    return this->get_column(0).begin();
                case RIGHT:
                    return this->get_line(0).begin();
                case UP:
                    it = this->get_column(0).end();
                    it.reverse();
                    return ++it;
                case LEFT:
                    it = this->get_line(0).end();
                    it.reverse();
                    return ++it;
            }
        }
        iterator<const map> begin(direction direction) const {
            iterator<const map> it;
            switch (direction) {
                case DOWN:
                    return this->get_column(0).begin();
                case RIGHT:
                    return this->get_line(0).begin();
                case UP:
                    it = this->get_column(0).end();
                    (++it).reverse();
                    return it;
                case LEFT:
                    it = this->get_line(0).end();
                    (++it).reverse();
                    return it;
            }
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
        line<map> get_line(uint n) { return line<map>{n, this}; }
        line<const map> get_line(uint n) const { return line<const map>{n, this}; }
        column<map> get_column(uint n) { return column<map>{n, this}; }
        column<const map> get_column(uint n) const { return column<const map>{n, this}; }
        uint xyptr(uint x, uint y) const { return x*this->linelen + y; }
        std::vector<T>::reference at(uint x, uint y) { return this->storage.at(xyptr(x, y)); }
        const std::vector<T>::reference at(uint x, uint y) const { return this->storage.at(xyptr(x, y)); }
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
    map<char> read_map() {
        map<char> ret = {};
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
        assert(ret.storage.size() == (ulong)ret.nlines*ret.linelen);
        return ret;
    }
}

template<class T>
std::ostream& operator<<(std::ostream& os, utils::map<T> &m) {
    auto i = m.begin(utils::map<T>::RIGHT);
    bool spaces = sizeof(T) != sizeof(char);
    bool first = true;
    while (!i.out_of_bounds()) {
        for (;!i.out_of_bounds();++i){
            if (spaces && !first) os << " "; else first = false;
            os << *i;
        }
        i.y=0; i.next();
        first = true;
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
    bool spaces = sizeof(T) != sizeof(char);
    bool first = true;
    for (auto& s: vs) {
        if (spaces && !first) os << " "; else first = false;
        os << s;
    }
    return os;
}

template <class T>
std::ostream& operator<<(std::ostream& os, const std::list<T>& vs)
{
    bool spaces = sizeof(T) != sizeof(char);
    bool first = true;
    for (auto& s: vs) {
        if (spaces && !first) os << " "; else first = false;
        os << s;
    }
    return os;
}
