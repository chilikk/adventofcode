#include <iostream>
#include <span>
#include <vector>
#include <deque>
#include <set>
#include <assert.h>

const unsigned int N = 2;
const unsigned int E = 3;
const unsigned int S = 5;
const unsigned int W = 7;
const unsigned int START = 0;
const unsigned int GROUND = 1;
const unsigned int OUT = 11;
const unsigned int INVALID = 13;
const unsigned int LOOP = 17;

typedef unsigned long coord;
typedef unsigned long long coordid;
typedef unsigned char val;

struct point {
    coord x;
    coord y;
    operator coordid() const {return (coordid(x) << (4 * sizeof(coordid))) + y;}
};

val sym2ord(char sym) {
    switch (sym) {
        case '|': return (N*S);
        case '-': return (E*W);
        case 'L': return (N*E);
        case 'J': return (N*W);
        case '7': return (S*W);
        case 'F': return (S*E);
        case 'S': return START;
        case '.': return GROUND;
        default:  assert(false); return INVALID;
    }
}

//char sym(val x) {
//    switch (x) {
//        case (N*S): return '|';
//        case (E*W): return '-';
//        case (N*E): return 'L';
//        case (N*W): return 'J';
//        case (S*W): return '7';
//        case (S*E): return 'F';
//        case START: return 'S';
//        case GROUND: return '.';
//        case LOOP: return 'X';
//        case OUT: return 'O';
//        default: assert(0); return '\0';
//    }
//}
//
//void print(val x) {
//    if (x == LOOP) std::clog << "\033[0;31m";
//    if (x != LOOP && x != OUT) std::clog << "\033[0;36m";
//    std::clog << sym(x);
//    std::clog << "\033[0m";
//}

val opposite(val direction) {
    switch (direction) {
        case N: return S;
        case E: return W;
        case S: return N;
        case W: return E;
        default: assert(false); return 0;
    }
}

void connected(const point &p, std::span<std::span<val>> &map, std::deque<point> &next) {
    int update_starting = (map[p.x][p.y] == START);
    if (p.x > 0 && map[p.x][p.y] % N == 0 && map[p.x-1][p.y] % S == 0) {
        next.push_back(point{p.x-1, p.y});
        update_starting *= N;
    }
    if (p.x < map.size() - 1 && map[p.x][p.y] % S == 0 && map[p.x+1][p.y] % N == 0) {
        next.push_back(point{p.x+1,p.y});
        update_starting *= S;
    }
    if (p.y > 0 && map[p.x][p.y] % W == 0 && map[p.x][p.y-1] % E == 0) {
        next.push_back(point{p.x,p.y-1});
        update_starting *= W;
    }
    if (p.y < map[0].size() - 1 && map[p.x][p.y] % E == 0 && map[p.x][p.y+1] % W == 0) {
        next.push_back(point{p.x,p.y+1});
        update_starting *= E;
    }
    if (update_starting) map[p.x][p.y] = update_starting;
}

void fill_insert(point p, std::deque<point> &next, std::set<coordid> &seen) {
    if (!seen.contains(p)) {
        next.push_back(p);
        seen.insert(p);
    }
}

void fill(point p, val v, std::span<std::span<val>> map) {
    if (map[p.x][p.y] == LOOP || map[p.x][p.y] == v) return;
    std::deque next = {p};
    std::set<coordid> seen = {p};
    while (!next.empty()) {
        p = next.front();
        next.pop_front();
        seen.insert(p);
        //std::clog << "filling at " << p.x << ":" << p.y << std::endl;
        map[p.x][p.y] = v;
        if (p.x > 0 && map[p.x-1][p.y] != LOOP && map[p.x-1][p.y] != v) {
            fill_insert(point{p.x-1,p.y}, next, seen);
        }
        if (p.x < map.size() - 1 && map[p.x+1][p.y] != LOOP && map[p.x+1][p.y] != v) {
            fill_insert(point{p.x+1,p.y}, next, seen);
        }
        if (p.y > 0 && map[p.x][p.y-1] != LOOP && map[p.x][p.y-1] != v) {
            fill_insert(point{p.x,p.y-1}, next, seen);
        }
        if (p.y < map[0].size() - 1 && map[p.x][p.y+1] != LOOP && map[p.x][p.y+1] != v) {
            fill_insert(point{p.x,p.y+1}, next, seen);
        }
    }
}

int main() {
    std::string buf;
    std::vector<val> storage = {};
    std::vector<std::span<val>> spans = {};
    std::span<std::span<val>> map = {};
    coord linelen = 0;
    coord lines = 0;
    val cur = 0;
    std::deque<point> next = {};
    std::set<coordid> loop = {};
    point p = {};
    point from = {};
    coordid pid = 0;
    while (getline(std::cin, buf)) {
        if (!linelen) {
            linelen = buf.length();
        }
        for (const auto& c: buf) {
            cur = sym2ord(c);
            if (cur == START) {
                next.push_back(point{lines, storage.size() % linelen});
            }
            storage.push_back(cur);
        }
        lines++;
    }
    for (coord i = 0; i < storage.size(); i += linelen) {
        spans.push_back(std::span(storage).subspan(i, linelen));
    }
    map = std::span(spans);
    while (!next.empty()) {
        p = next.front();
        next.pop_front();
        pid = p;
        if (!loop.contains(pid)) {
            //std::clog << "loop insert " << std::to_string(p.x)
            //          << ":" << std::to_string(p.y) << std::endl;
            loop.insert(pid);
            connected(p, map, next);
        }
    }
    std::cout << loop.size() / 2 << "\n";
    // part 2
    // lame, but probably works
    p = point({lines/2,0});
    while (!loop.contains(p)) {
        p = point{p.x,p.y+1};
    }
    // initial direction, going clockwise
    val direction = N;
    // indicates the direction from the previous to this point
    if (map[p.x][p.y] == N*E) { direction = W; }

    // at this point, make a sparse map to indicate spaces between pipes
    std::vector<val> storage2 = {};
    storage2.resize(4*storage.size());
    std::vector<std::span<val>> spans2 = {};
    std::span<std::span<val>> sparsemap = {};
    for (coord i = 0; i < storage2.size(); i += 2*linelen) {
        spans2.push_back(std::span(storage2).subspan(i, 2*linelen));
    }
    sparsemap = std::span(spans2);

    point starting = p;
    do {
        //std::clog << "walk the loop " << p.x << ":" << p.y
        //    << " direction " << std::to_string(direction) << std::endl;
        assert(map[p.x][p.y] % (opposite(direction)) == 0);
        direction = map[p.x][p.y] / (opposite(direction));
        sparsemap[2*p.x][2*p.y] = LOOP;
        if (direction == N) {
            sparsemap[2*p.x-1][2*p.y] = LOOP;
            p = point{p.x-1,p.y};
        }
        if (direction == E) {
            sparsemap[2*p.x][2*p.y+1] = LOOP;
            p = point{p.x,p.y+1};
        }
        if (direction == S) {
            sparsemap[2*p.x+1][2*p.y] = LOOP;
            p = point{p.x+1,p.y};
        }
        if (direction == W){
            sparsemap[2*p.x][2*p.y-1] = LOOP;
            p = point{p.x,p.y-1};
        }
    } while (starting != p);
    for (coord i = 0; i < 2*linelen; i++) {
        fill(point{0,i}, OUT, sparsemap);
        fill(point{2*lines-1,i}, OUT, sparsemap);
    }
    for (coord i = 0; i < 2*lines; i++) {
        fill(point{i,0}, OUT, sparsemap);
        fill(point{i,2*linelen-1}, OUT, sparsemap);
    }
    unsigned int cnt = 0;
    val v = 0;
    for (coord i = 0; i < 2*lines; i+=2) {
        for (coord j = 0; j < 2*linelen; j+=2) {
            v = sparsemap[i][j];
            //print(v);
            if (v != OUT && v != LOOP) cnt++;
        }
        //std::clog << std::endl;
    }
    std::cout << cnt << "\n";
}
