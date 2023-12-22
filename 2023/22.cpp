#include "utils.hpp"
using std::vector;

#define debug 0 && std::clog

struct point {
    uint x, y, z;
    point(int x, int y, int z): x(x), y(y), z(z) {};
    point(uint x, uint y, uint z): x(x), y(y), z(z) {};
};

struct block {
    uint id;
    point p1, p2;
    block(uint id, point p1, point p2):
        id(id),
        p1(std::min(p1.x, p2.x), std::min(p1.y, p2.y), std::min(p1.z, p2.z)),
        p2(std::max(p1.x, p2.x), std::max(p1.y, p2.y), std::max(p1.z, p2.z))
    {};
};

std::ostream& operator<<(std::ostream& os, const point& p)
{
    os << p.x << ":" << p.y << ":" << p.z;
    return os;
}

std::ostream& operator<<(std::ostream& os, const block& bl)
{
    os << bl.id << "(";
    if (bl.p1.x == bl.p2.x) {
        os << bl.p1.x;
    } else {
        os << bl.p1.x << "-" << bl.p2.x;
    }
    os << ":";
    if (bl.p1.y == bl.p2.y) {
        os << bl.p1.y;
    } else {
        os << bl.p1.y << "-" << bl.p2.y;
    }
    os << ":";
    if (bl.p1.z == bl.p2.z) {
        os << bl.p1.z;
    } else {
        os << bl.p1.z << "-" << bl.p2.z;
    }
    os << ")";
    return os;
}


int main() {
    vector<vector<vector<uint>>> m(300, vector(10, vector(10, 0u)));
    vector<block> blocks{};
    std::string line;
    uint blockcnt = 0;
    uint maxz = 0;
    while (std::getline(std::cin, line)) {
        blockcnt++;
        ulong pos = line.find_first_of('~');
        assert(line[1] == ',' && line[3] == ',' && line[pos+2] == ',' && line[pos+4] == ',');
        auto p1 = point{line[0]-'0', line[2]-'0', std::stoi(line.substr(4, pos))-1};
        auto p2 = point{line[pos+1]-'0', line[pos+3]-'0', std::stoi(line.substr(pos+5))-1};
        auto bl = block(blockcnt, p1, p2);
        blocks.push_back(bl);
        for (uint i = bl.p1.z; i <= std::max(p1.z, p2.z); i++) {
            for (uint j = bl.p1.y; j <= bl.p2.y; j++) {
                for (uint k = bl.p1.x; k <= bl.p2.x; k++) {
                    m.at(i).at(j).at(k) = blockcnt;
                }
            }
        }
        maxz = std::max(maxz, std::max(p1.z, p2.z));
    }
    m.resize(maxz+1);
    std::set<uint> unsafe{};
    std::sort(blocks.begin(), blocks.end(), [](auto &a, auto &b){return a.p1.z < b.p1.z;});
    for (auto &bl: blocks) {
        debug << bl << " -> ";
        std::set<uint> refs{};
        for (int i = (int)bl.p1.z; i>0 && refs.size() == 0;) {
            i--;
            for (uint j = bl.p1.y; j <= bl.p2.y; j++) {
                for (uint k = bl.p1.x; k <= bl.p2.x; k++) {
                    auto v = m.at(i).at(j).at(k);
                    if (v != 0) {
                        refs.insert(v);
                    }
                }
            }
            if (refs.size() == 0) {
                bl.p1.z--;
                for (uint j = bl.p1.y; j <= bl.p2.y; j++) {
                    for (uint k = bl.p1.x; k <= bl.p2.x; k++) {
                        m.at(bl.p1.z).at(j).at(k) = bl.id;
                        m.at(bl.p2.z).at(j).at(k) = 0;
                    }
                }
                bl.p2.z--;
            }
        }
        debug << bl << " refs: ";
        for (auto &ref: refs) debug << ref << " ";
        debug << std::endl;
        if (refs.size() == 1) unsafe.insert(*(refs.begin()));
    }
    auto cnt = 0;
    for (auto &bl: blocks) if (!unsafe.contains(bl.id)) cnt++;
    std::cout << cnt << std::endl;
    // part 2
    ulong sum = 0;
    std::set<uint> fallen;
    for (auto bldestr = blocks.cbegin(); bldestr+1 != blocks.cend(); bldestr++) {
        if (!unsafe.contains(bldestr->id)) continue;
        debug << *bldestr << " -> ";
        fallen = {bldestr->id};
        for (auto bl = bldestr + 1; bl != blocks.cend(); bl++) {
            auto hasrefs = false;
            for (int i = (int)bl->p1.z; i>0 && !hasrefs;) {
                i--;
                for (uint j = bl->p1.y; j <= bl->p2.y; j++) {
                    for (uint k = bl->p1.x; k <= bl->p2.x; k++) {
                        auto v = m.at(i).at(j).at(k);
                        if (v != 0 && !fallen.contains(v)) {
                            hasrefs = true;
                        }
                    }
                }
                if (!hasrefs) {
                    fallen.insert(bl->id);
                }
            }
        }
        uint totalfallen = fallen.size() - 1; // do not count the originally destroyed block
        debug << totalfallen << std::endl;
        sum += totalfallen;
    }
    std::cout << sum << std::endl;
}
