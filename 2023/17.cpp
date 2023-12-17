#include "utils.hpp"
#include <queue>

#define debugon 0
#if debugon==1
#include <list>
#endif
#define debug debugon && std::clog

struct pathpoint {
    utils::map<char>::iterator<utils::map<char>> it;
    uint straight = 0;
    uint total_heat = 0;
    utils::map<uint> *min_estimate;
#if debugon==1
    std::list<pathpoint> path = {};
#endif
    auto operator <(const pathpoint &other) const {
        // the lower the total_heat (plus estimate) the better
        return score() < other.score();
    }
    uint score() const {
        return total_heat + min_estimate->at(it.x, it.y);
    }
    operator ulong() const {
        return 64*(it.x*it.m->linelen+it.y)+16*it.direction() + straight;
    }
};

char cdir(const auto &dir) {
    switch (dir) {
        case utils::map<char>::RIGHT: return '>';
        case utils::map<char>::LEFT: return '<';
        case utils::map<char>::UP: return '^';
        case utils::map<char>::DOWN: return 'v';
    }
}

std::ostream& operator<<(std::ostream& os, const pathpoint& pp)
{
    os << pp.it.x << ":" << pp.it.y << " "
        << cdir(pp.it.direction()) << "(" << pp.straight << ")"
        << " acc " << pp.total_heat
        << " score " << pp.score();
    return os;
}

void print(auto map, auto &path) {
    for (auto &c: map.storage) c += '0';
    for (auto &pp: path) {
        map.at(pp.it.x, pp.it.y) = cdir(pp.it.direction());
        debug << pp << std::endl;
    }
    debug << map << std::endl;
}

void explore(auto &map, auto &total, uint subtask, auto &paths) {
    std::vector<uint> seen(64*map.storage.size(), UINT_MAX);
    debug << "explore " << subtask << std::endl;
    while (!paths.empty()) {
        auto pp = paths.top();
        paths.pop();
        if (seen.at(pp) <= pp.total_heat) {
            continue;
        } else {
            seen.at(pp) = pp.total_heat;
        }
        if (pp.score() >= total) {
            continue;
        }
        pp.total_heat += *(pp.it);
#if debugon==1
        pp.path.push_back(pathpoint{pp.it, pp.straight, pp.total_heat, pp.min_estimate});
#endif
        if ((pp.it.x + 1) == map.nlines && (pp.it.y + 1) == map.linelen) {
            if (subtask==1 || (subtask==2 && pp.straight >= 4)) {
                total = pp.total_heat;
#if debugon==1
                print(map, pp.path);
#endif
                debug << total << std::endl;
            }
            continue;
        }
        // see utils::map<T>::direction
        // -1 -> turn right
        // 0 -> continue straight
        // +1 -> turn left
        for (int dirdiff: {-1, 0, 1}) {
            if (subtask == 1 && dirdiff == 0 && pp.straight >= 3) continue;
            if (subtask == 2 && dirdiff != 0 && pp.straight < 4) continue;
            if (subtask == 2 && dirdiff == 0 && pp.straight >= 10) continue;
            auto it = utils::map<char>::iterator{pp.it};
            uint straight = 0;
            if (dirdiff != 0) {
                straight = 1;
                int dir = (int)pp.it.direction();
                dir += 4 + dirdiff;
                dir %= 4;
                it.set_direction(utils::map<char>::direction(dir));
            } else {
                straight = pp.straight+1;
            }
            ++it;
            auto next = pathpoint{it, straight, pp.total_heat, pp.min_estimate
#if debugon==1
                , pp.path
#endif
            };
            if (!it.out_of_bounds() && next.score() < total && seen.at(next) > next.total_heat) {
                paths.push(next);
            }
        }
    }
}

utils::map<uint> make_min_estimates(auto &map) {
    utils::map<uint> estimates{std::vector<uint>(map.storage.size(), INT_MAX), map.nlines, map.linelen};
    int init_x = map.nlines-1;
    int init_y = map.linelen-1;
    auto it = estimates.begin(estimates.LEFT);
    it.x = init_x;
    it.y = init_y;
    *it = (unsigned char)map.at(init_x, init_y);
    auto neigh = it;
    char v = 0;
    bool change = true;
    while (change) {
        change = false;
        auto it2 = it;
        while (!it2.out_of_bounds()) {
            auto it3 = it2;
            while (!it3.out_of_bounds()) {
                for (auto &dir: {estimates.UP, estimates.RIGHT, estimates.DOWN, estimates.LEFT}) {
                    neigh = it3;
                    neigh.set_direction(dir);
                    ++neigh;
                    v = map.at(it3.x, it3.y);
                    if (!neigh.out_of_bounds() && *it3 > *neigh + v) {
                        *it3 = *neigh + v;
                        change = true;
                    }
                }
                ++it3;
            }
            it2.prev();
        }
    }
    return estimates;
}

int main() {
    auto map = utils::read_map();
    for (auto &c: map.storage) c -= '0';
    auto estimates = make_min_estimates(map);
    debug << "minimal estimates:" << std::endl << estimates << std::endl;
    for (auto &subtask: {1, 2}) {
        ulong total = ULONG_MAX;
        std::priority_queue<pathpoint, std::vector<pathpoint>, std::greater<pathpoint>> paths{};
        paths.push(pathpoint{map.begin(map.RIGHT), 0, 0, &estimates});
        explore(map, total, subtask, paths);
        std::cout << total - map.at(0, 0) << std::endl;
    }
}
