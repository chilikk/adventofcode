#include <iostream>
#include <memory>
#include <span>
#include <vector>
#include "assert.h"

namespace utils {
    template<class T>
    class map {
        public:
        std::vector<T> storage;
        uint nlines = 0;
        uint linelen = 0;
        operator bool() {return linelen;}
        template <class M>
        class iterator {
                class badref {};
            public:
                int x = 0;
                int y = 0;
                int xdiff = 0;
                int ydiff = 0;
                M* m = nullptr;
                const T& operator *() const {
                    if (this->out_of_bounds()) throw(badref{});
                    return m->at(x,y);
                }
                T& operator *() {
                    if (this->out_of_bounds()) throw(badref{});
                    return m->at(x,y);
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
                void prev() {
                    if (xdiff != 0) left_once();
                    else if (ydiff != 0) up_once();
                    }
                uint id() {
                    if (xdiff == 0) return x;
                    if (ydiff == 0) return y;
                    assert(false);
                }
                int direction() {
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
                utils::map<T>::iterator<M> begin() { return utils::map<T>::iterator<M>{(int)lineno, 0, 0, 1, m}; }
                utils::map<T>::iterator<M> end()   { return utils::map<T>::iterator<M>{(int)lineno, (int)(m->linelen), 0, 1, m}; }
        };
        template<class M>
        class column {
            public:
                uint colno;
                M * m;
                utils::map<T>::iterator<M> begin() {
                    return utils::map<T>::iterator<M>{0, (int)colno, 1, 0, m};
                }
                utils::map<T>::iterator<M> end() {
                    return utils::map<T>::iterator<M>{(int)(m->nlines), (int)colno, 1, 0, m};
                }
        };
        static const int DOWN = 0;
        static const int RIGHT = 1;
        static const int UP = 2;
        static const int LEFT = 3;
        iterator<map> begin(int direction) {
            if (direction == DOWN) {
                //down
                return this->get_column(0).begin();
            } else if (direction == RIGHT) {
                //right
                return this->get_line(0).begin();
            } else if (direction == UP) {
                //up
                auto it = this->get_column(0).end();
                it.reverse();
                it.up_once();
                return it;
            } else if (direction == LEFT) {
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
        line<map> get_line(uint n) { return line<map>{n, this}; }
        line<const map> get_line(uint n) const { return line<const map>{n, this}; }
        column<map> get_column(uint n) { return column<map>{n, this}; }
        column<const map> get_column(uint n) const { return column<const map>{n, this}; }
        uint xyptr(uint x, uint y) const { return x*this->linelen + y; }
        T& at(uint x, uint y) { return this->storage.at(xyptr(x, y)); }
        const T& at(uint x, uint y) const { return this->storage.at(xyptr(x, y)); }
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
    auto i = m.begin(1);
    bool spaces = sizeof(T) != sizeof(char);
    bool first = true;
    while (!i.out_of_bounds()) {
        for (;!i.out_of_bounds();++i){
            if (spaces && !first) os << " "; else first = false;
            os << *i;
        }
        i.y=0; i.down_once();
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
