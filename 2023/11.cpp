#include <iostream>
#include <vector>
#include <span>

#define DIST 1000000;

typedef unsigned long ulong;

struct point {
    ulong x;
    ulong y;
};

ulong dist(const point& p1, const point& p2) {
    return ((p1.x>p2.x)?p1.x-p2.x:p2.x-p1.x) + ((p1.y>p2.y)?p1.y-p2.y:p2.y-p1.y);
}

int main() {
    std::vector<point> galaxies = {};
    std::string line;
    ulong lineidx = 0;
    ulong ixmax = 0;
    while (std::getline(std::cin, line)) {
        ulong pos = line.find('#');
        while (pos != std::string::npos) {
            galaxies.push_back(point{lineidx,pos});
            if (pos > ixmax) ixmax = pos;
            pos = line.find('#', pos+1);
        }
        if (galaxies.empty() || galaxies.back().x != lineidx) {
            lineidx += DIST;
        } else {
            lineidx++;
        }
    }
    for (ulong i = 0; i < ixmax; i++) {
        bool empty = true;
        for (auto& galaxy: galaxies) {
            if (galaxy.y == i) {
                empty = false;
            }
        }
        if (empty) {
            for (auto& galaxy: galaxies) {
                if (galaxy.y > i) --galaxy.y += DIST;
            }
            i += DIST;
            ixmax += DIST;
        }
    }
    unsigned long long total = 0;
    std::span s = galaxies;
    while (s.size() > 1) {
        point ref = s.front();
        s = s.subspan(1);
        for (auto& galaxy: s) {
            total += dist(ref, galaxy);
        }
    }
    std::cout << total << std::endl;
}
