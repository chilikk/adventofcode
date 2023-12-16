#include "utils.hpp"
#include <bitset>
#include <queue>

#define debug 0 && std::clog

void follow(utils::map<std::bitset<4>>::iterator<utils::map<std::bitset<4>>> &l,
        const utils::map<char> &mirrors,
        std::queue<utils::map<std::bitset<4>>::iterator<utils::map<std::bitset<4>>>> &q) {
    while (!l.out_of_bounds() && (*l)[l.direction()] == 0) {
        debug << "light at " << l.x << ":" << l.y << " direction " << l.direction() << " value " << *l << std::endl;
        (*l).set(l.direction());
        debug << "set " << *l << std::endl;
        switch (mirrors.at(l.x,l.y)) {
            case '.': break;
            case '/': switch(l.direction()) {
                          case utils::map<std::bitset<4>>::RIGHT: l.up(); break;
                          case utils::map<std::bitset<4>>::DOWN: l.left(); break;
                          case utils::map<std::bitset<4>>::UP: l.right(); break;
                          case utils::map<std::bitset<4>>::LEFT: l.down(); break;
                          default: assert(false);
                      };
                      break;
            case '\\': switch(l.direction()) {
                           case utils::map<std::bitset<4>>::RIGHT: l.down(); break;
                           case utils::map<std::bitset<4>>::DOWN: l.right(); break;
                           case utils::map<std::bitset<4>>::UP: l.left(); break;
                           case utils::map<std::bitset<4>>::LEFT: l.up(); break;
                           default: assert(false);
                       };
                       break;
            case '|': switch(l.direction()) {
                          case utils::map<std::bitset<4>>::DOWN:
                          case utils::map<std::bitset<4>>::UP:
                              break;
                          case utils::map<std::bitset<4>>::RIGHT:
                          case utils::map<std::bitset<4>>::LEFT:
                              l.up();
                              q.push(utils::map<std::bitset<4>>::iterator{l});
                              l.down();
                              break;
                          default: assert(false);
                      };
                      break;
            case '-': switch(l.direction()) {
                          case utils::map<std::bitset<4>>::DOWN:
                          case utils::map<std::bitset<4>>::UP:
                              l.left();
                              q.push(utils::map<std::bitset<4>>::iterator{l});
                              l.right();
                              break;
                          case utils::map<std::bitset<4>>::RIGHT:
                          case utils::map<std::bitset<4>>::LEFT:
                              break;
                          default: assert(false);
                      };
                      break;
            default: assert(false);
        }
        ++l;
    }
}

ulong test(int x, int y, auto direction, auto &mirrors) {
    utils::map lights = utils::map<std::bitset<4>>{
        std::vector<std::bitset<4>>(mirrors.storage.size(), std::bitset<4>{}),
            mirrors.nlines,
            mirrors.linelen};
    std::queue<utils::map<std::bitset<4>>::iterator<utils::map<std::bitset<4>>>> q(
            {{x, y, 0, 0, &lights}}
            );
    direction(q.front());
    while (!q.empty()) {
        follow(q.front(), mirrors, q);
        q.pop();
    }
    ulong total = 0;
    for (auto &bits: lights.storage) {
        if (bits.any()) total++;
    }
    return total;
}

int main() {
    utils::map mirrors = utils::read_map();
    ulong count = test(0, 0, [](auto &i) {i.right();}, mirrors);
    std::cout << count << std::endl;
    ulong max = 0;
    for (int i = 0; i < mirrors.nlines; i++) {
        count = test(i, 0, [](auto &i) {i.right();}, mirrors);
        if (count > max) max = count;
        count = test(i, (int)mirrors.linelen-1, [](auto &i) {i.left();}, mirrors);
        if (count > max) max = count;
    }
    for (int i = 0; i < mirrors.linelen; i++) {
        count = test(0, i, [](auto &i) {i.down();}, mirrors);
        if (count > max) max = count;
        count = test((int)mirrors.nlines-1, i, [](auto &i) {i.up();}, mirrors);
        if (count > max) max = count;
    }
    std::cout << max << std::endl;
}
