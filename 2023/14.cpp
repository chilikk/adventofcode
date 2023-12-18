#include "utils.hpp"

struct rng {
    utils::map<char>::iterator<utils::map<char>> begin, end;
};

uint score(utils::map<char>& m) {
    uint total = 0;
    for (int i=0; i<m.nlines; i++) {
        uint counto = 0;
        for (auto &c: m.get_line(i)) {
            if (c=='O') counto++;
        }
        total += counto * (m.nlines-i);
    }
    return total;
}

int main() {
    utils::map map = utils::read_map();
    std::vector<utils::map<char>> maps = {};
    std::span<char> prev = {};
    std::span<char> cur = {};
    std::vector<rng> nranges = {}, sranges = {},  wranges = {}, eranges = {};
    auto dir = map.DOWN;
    for (auto rangeset: {&nranges, &wranges, &sranges, &eranges}) {
        auto p = map.begin(dir);
        for (auto seq = p; !seq.out_of_bounds(); seq.next()) {
            auto begin = seq;
            auto last = begin;
            for (auto i = begin; !i.out_of_bounds(); ++i) {
                if (*i == '#') {
                    if (begin != last) {
                        rangeset->push_back(rng{begin, i});
                    }
                    begin = i; ++begin;
                }
                last = i;
            }
            if (begin != last) {
                rangeset->push_back(rng{begin, ++last});
            }
        }
        dir=utils::map<char>::direction((dir+1)%4);
    }
    bool first_done = false;
    bool period_found = false;
    const auto rounds = 1000000000;
    for (int i = 0; i < rounds; ++i) {
        if (!period_found) {
            for (auto xmap = maps.begin(); xmap != maps.end(); ++xmap) {
                if (map == *xmap) {
                    period_found = true;
                    auto period = i-(int)std::distance(maps.begin(), xmap);
                    while (i+period < rounds) i+=period;
                }
            }
            maps.push_back(utils::map<char>{std::vector{map.storage}, map.nlines, map.linelen});
        }
        for (auto &rangeset: {nranges, wranges, sranges, eranges}) {
            for (auto &range: rangeset) {
                uint counto = 0;
                for (auto e = range.begin; e != range.end; ++e) {
                    if (*e == 'O') counto++;
                }
                for (auto e = range.begin; e != range.end; ++e) {
                    if (counto>0) {
                        *e = 'O';
                        counto--;
                    } else {
                        *e = '.';
                    }
                }
            }
            if (!first_done) {
                std::cout << score(map) << std::endl;
                first_done = true;
            }
        }
        //std::clog << map << std::endl;
    }
    std::cout << score(map) << std::endl;
}
