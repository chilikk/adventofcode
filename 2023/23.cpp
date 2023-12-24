#include "utils.hpp"
#include <tuple>

#define debug 0 && std::clog

typedef utils::map<char> cmap;

struct vertex {
    uint x, y;
    auto operator <=> (const auto &other) const {
        return (x<<16)+y <=> (other.x<<16)+other.y;
    }
};

std::ostream& operator<<(std::ostream& os, const std::map<vertex,std::map<vertex,uint>> &graph)
{
    uint cnt = 0;
    for (auto &[begin, m]: graph) {
        for (auto &[end, dist]: m) {
            cnt++;
            os << begin.x << ":" << begin.y << " -> "
               << end.x << ":" << end.y << " = "
               << dist << std::endl;
        }
    }
    os << "size " << cnt << std::endl;
    return os;
}

void ins_graph(vertex &begin, vertex &&end, uint pathlen, auto &graph) {
    if (!graph.contains(begin)) graph[begin] = std::map<vertex,uint>{};
    if (!graph[begin].contains(end)) {
        graph[begin][end] = pathlen;
    } else {
        graph[begin][end] = std::max(pathlen, graph[begin][end]);
    }
}

void build_graph(cmap::iterator<cmap> pos, cmap &m, std::map<vertex,std::map<vertex,uint>> &graph, unsigned char subtask) {
    vertex begin = vertex{(uint)pos.x, (uint)pos.y};
    std::queue<std::tuple<vertex, cmap::iterator<cmap>>> q{{std::tuple{begin, pos}}};
    utils::map<bool> seen{std::vector<bool>(m.storage.size(), false), m.nlines, m.linelen};
    seen.at(pos.x, pos.y) = true;
    while (!q.empty()) {
        std::tie(begin, pos) = q.front();
        q.pop();
        long pathlen = std::abs(pos.x-(int)begin.x) + std::abs(pos.y-(int)begin.y);
        vertex prev = begin;
        while (true) {
            //debug << begin.x << ":" << begin.y << " " << pos.x << ":" << pos.y << " " << pathlen << std::endl;
            if (pos.x + 1 == m.nlines) {
                ins_graph(begin, vertex{(uint)pos.x, (uint)pos.y}, pathlen, graph);
                break;
            }
            std::vector<cmap::iterator<cmap>> choices{};
            for (auto &dir: {cmap::UP, cmap::RIGHT, cmap::DOWN, cmap::LEFT}) {
                if (subtask == 1 && *pos != '.') {
                    if (*pos == 'v' && dir != cmap::DOWN) continue;
                    if (*pos == '<' && dir != cmap::LEFT) continue;
                    if (*pos == '^' && dir != cmap::UP) continue;
                    if (*pos == '>' && dir != cmap::RIGHT) continue;
                }
                pos.set_direction(dir);
                ++pos;
                if (!pos.out_of_bounds() && *pos != '#'
                        && (pos.x != prev.x || pos.y != prev.y)) {
                    choices.push_back(pos);
                }
                --pos;
            }
            if (choices.size() == 0) {
                break;
            } else if (choices.size() == 1) {
                prev = vertex{(uint)pos.x, (uint)pos.y};
                pos = choices.at(0);
                pathlen++;
            } else {
                // we need a directional graph for part 1, so only insert
                // the forward direction
                // this will cause some overhead for part 2, but that's ok
                ins_graph(begin, vertex{(uint)pos.x, (uint)pos.y}, pathlen, graph);
                if (!seen.at(pos.x, pos.y)) {
                    seen.at(pos.x, pos.y) = true;
                    // also explore the reverse path, because the
                    // graph for part 1 is directional
                    auto nextpos = pos;
                    nextpos.x = (int)prev.x;
                    nextpos.y = (int)prev.y;
                    q.push(std::tuple{vertex{(uint)pos.x, (uint)pos.y}, nextpos});
                    for (auto &nextpos: choices) {
                        q.push(std::tuple{vertex{(uint)pos.x, (uint)pos.y}, nextpos});
                    }
                }
                break;
            }
        }
    }
}

void search(vertex pos, ulong pathlen, ulong &longest, uint endxpos, auto seen, auto &graph) {
    seen.insert(pos);
    for (auto &[vertex, dist]: graph.at(pos)) {
        if (vertex.x == endxpos) {
            if (pathlen + dist > longest) {
                longest = pathlen + dist;
            }
        } else if (!seen.contains(vertex)) {
            search(vertex, pathlen+dist, longest, endxpos, seen, graph);
        }
    }
}

ulong find_longest(vertex start, uint endxpos, auto &graph) {
    ulong longest = 0;
    std::set<vertex> seen{};
    search(start, 0, longest, endxpos, seen, graph);
    return longest;
}

int main() {
    cmap m = utils::read_map();
    std::map<vertex,std::map<vertex,uint>> graph;
    cmap::iterator<cmap> start = m.begin(cmap::RIGHT);
    for (;*start != '.';++start);
    vertex startvtx{(uint)start.x, (uint)start.y};
    build_graph(start, m, graph, 1);
    debug << graph << std::endl;
    std::cout << find_longest(startvtx, m.nlines-1, graph) << std::endl;
    build_graph(start, m, graph, 2);
    debug << graph << std::endl;
    std::cout << find_longest(startvtx, m.nlines-1, graph) << std::endl;
}
