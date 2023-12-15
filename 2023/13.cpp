#include "utils.hpp"
#include <string>
#include <set>

struct cmpres {
    bool res = true;
    bool fix = false;
};

cmpres cmp(auto i1, auto i2) {
    cmpres res = {};
    while (!i1.out_of_bounds()) {
        if ((*i1) != (*i2)) {
            if (!res.fix) {
                res.fix = true;
            } else {
                res.res = false;
                break;
            }
        }
        ++i1, ++i2;
    }
    return res;
}

bool test_refl(auto down, auto up, uint fixes) {
    while (!down.out_of_bounds() && !up.out_of_bounds()) {
        auto cmpres = cmp(down, up);
        if (!cmpres.res) return false;
        if (cmpres.fix) {
            if (fixes == 0) return false;
            fixes--;
        }
        up.next();
        down.prev();
    }
    return fixes == 0;
}

uint find_refl(auto it, uint fixes) {
    auto prev = it;
    it.next();
    while (!it.out_of_bounds()) {
        if (test_refl(prev, it, fixes)) {
            return it.id();
        }
        prev = it;
        it.next();
    }
    return 0;
}

int main() {
    utils::map map;
    ulong total1 = 0;
    ulong total2 = 0;
    while ((map = utils::read_map())) {
        auto it = map.begin(utils::map::RIGHT);
        uint refl1 = 100*find_refl(it, 0);
        uint refl2 = 100*find_refl(it, 1);
        if (!refl1 || !refl2) {
            it = map.begin(utils::map::DOWN);
            if (!refl1) {
                refl1 = find_refl(it, 0);
            }
            if (!refl2) {
                refl2 = find_refl(it, 1);
            }
        }
        total1 += refl1;
        total2 += refl2;
    }
    std::cout << total1 << std::endl << total2 << std::endl;
}
