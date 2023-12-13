#include "utils.hpp"
#include <string>
#include <set>

struct cmpres {
    bool res = true;
    bool fix = false;
};

cmpres cmp(std::span<char> s1, std::span<char> s2) {
    cmpres res = {};
    auto i1 = s1.begin();
    auto i2 = s2.begin();
    while (i1 != s1.end()) {
        if ((*i1) != (*i2)) {
            if (!res.fix) {
                res.fix = true;
            } else {
                res.res = false;
                break;
            }
        }
        i1++; i2++;
    }
    return res;
}

bool test_refl(utils::map& map, int reflpoint, uint fixes) {
    auto down = reflpoint-1;
    auto up = reflpoint;
    while (down >= 0 && up < map.lines) {
        auto cmpres = cmp(map.line(down), map.line(up));
        if (!cmpres.res) return false;
        if (cmpres.fix) {
            if (fixes == 0) return false;
            fixes--;
        }
        up++;
        down--;
    }
    return fixes == 0;
}

uint find_refl(utils::map& map, uint fixes) {
    int lineidx = 0;
    while (++lineidx < map.lines) {
        if (test_refl(map, lineidx, fixes)) {
            return lineidx;
        }
    }
    return 0;
}

utils::map transpose(const utils::map &map) {
    utils::map ret = {};
    ret.linelen = map.lines;
    ret.lines = map.linelen;
    ret.storage.reserve(map.storage.size());
    for (int i = 0; i < map.linelen; i++) {
        for (int j = 0; j < map.lines; j++) {
            ret.storage.push_back(map.storage[j*map.linelen+i]);
        }
    }
    return ret;
}

int main() {
    utils::map map;
    ulong total1 = 0;
    ulong total2 = 0;
    while ((map = utils::read_map())) {
        uint refl1 = 100*find_refl(map, 0);
        uint refl2 = 100*find_refl(map, 1);
        if (!refl1 || !refl2) {
            map = transpose(map);
            if (!refl1) {
                refl1 = find_refl(map, 0);
            }
            if (!refl2) {
                refl2 = find_refl(map, 1);
            }
        }
        total1 += refl1;
        total2 += refl2;
    }
    std::cout << total1 << endl << total2 << endl;
}
