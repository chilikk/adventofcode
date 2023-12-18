#include "utils.hpp"
#include <list>
#include <set>
#include <regex>
#include <queue>

typedef utils::map<char> cmap;

struct instruction {
    cmap::direction dir;
    ulong steps;
};

ulong ul(cmap::iterator<cmap> it) {
    return ((uint)(it.x) << 16) + it.y;
}

void fill(auto it, char sym) {
    std::queue<cmap::iterator<cmap>> q{{it}};
    std::set<ulong> seen = {};
    while (!q.empty()) {
        it = q.front();
        q.pop();
        *it = sym;
        for (auto &dir: {cmap::UP, cmap::DOWN, cmap::RIGHT, cmap::LEFT}) {
            it.set_direction(dir);
            ++it;
            if (!it.out_of_bounds() && *it == '.' && !seen.contains(ul(it))) {
                q.push(it);
                seen.insert(ul(it));
            }
            --it;
        }
    }
}

std::vector<ulong> mk_weights(auto &coords) {
    std::sort(coords.begin(), coords.end());
    auto end = std::unique(coords.begin(), coords.end());
    coords.resize(std::distance(coords.begin(), end));
    auto minx = coords.front();
    std::vector<ulong> weights{1};
    long prev = 1-minx;
    for (auto &c: coords) {
        if (c-prev > 1) {
            weights.push_back(c-prev-1);
        }
        weights.push_back(1);
        prev = c;
    }
    weights.push_back(1);
    return weights;
}

ulong solve(auto &li) {
    auto it = cmap::iterator<cmap>{};
    it.x = 0;
    it.y = 0;
    std::vector<long> xs{it.x}, ys{it.y};
    for (auto &in: li) {
        it.set_direction(in.dir);
        it+=(int)in.steps;
        xs.push_back(it.x);
        ys.push_back(it.y);
    }
    auto xweights = mk_weights(xs);
    auto minx = xs.front(); // it was sorted by mk_weights
    auto yweights = mk_weights(ys);
    auto miny = ys.front(); // it was sorted by mk_weights

    uint sizex = xweights.size();
    uint sizey = yweights.size();
    // m is a sparse map where each row or column width is determined
    // by the corresponding position in xweights or yweights
    cmap m = cmap{std::vector<char>((ulong)sizex*sizey, '.'), sizex, sizey};
    it = m.begin(m.RIGHT);
    // find starting position
    it.x = 0;
    for (ulong sum = 0; sum != 1-minx; it.x++)
        sum += xweights.at(it.x);
    it.y = 0;
    for (ulong sum = 0; sum != 1-miny; it.y++)
        sum += yweights.at(it.y);
    *it = '#';
    // execute instructions
    for (auto &in: li) {
        it.set_direction(in.dir);
        for (ulong acc = 0; acc != in.steps;) {
            ++it;
            // use weights to count steps
            if (it.xdiff != 0) acc += xweights.at(it.x);
            if (it.ydiff != 0) acc += yweights.at(it.y);
            *it = '#';
        }
    }
    it = m.begin(m.RIGHT);
    fill(it, 'O');
    ulong area = 0;
    while (!it.out_of_bounds()) {
        auto it2 = it;
        while (!it2.out_of_bounds()) {
            if (*it2 != 'O') {
                // count area in weights
                area += xweights.at(it2.x)*yweights.at(it2.y);
            }
            ++it2;
        }
        it.next();
    }
    return area;
}

int main() {
    std::string line;
    std::regex re(R"(([R|L|U|D]) (\d+) \(#([0-9a-f]{5})([0-3])\))");
    std::smatch s;
    std::list<instruction> li1{}, li2;
    while (std::getline(std::cin, line)) {
        std::regex_match(line, s, re);
        instruction in{};
        switch (s[1].str()[0]) {
            case 'U': in.dir = cmap::UP; break;
            case 'D': in.dir = cmap::DOWN; break;
            case 'R': in.dir = cmap::RIGHT; break;
            case 'L': in.dir = cmap::LEFT; break;
            default: assert(false);
        }
        in.steps = std::stoi(s[2].str());
        li1.push_back(in);
        switch (s[4].str()[0]) {
            case '0': in.dir = cmap::RIGHT; break;
            case '1': in.dir = cmap::DOWN; break;
            case '2': in.dir = cmap::LEFT; break;
            case '3': in.dir = cmap::UP; break;
            default: assert(false);
        }
        in.steps = std::stoul(s[3].str(), nullptr, 16);
        li2.push_back(in);
    }
    std::cout << solve(li1) << std::endl << solve(li2) << std::endl;
}
