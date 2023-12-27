#include <algorithm>
#include <iostream>
#include <set>
#include <map>
#include <queue>

#define debug 1 && std::clog

struct graph {
    std::map<std::string,uint> vertices{};
    std::vector<std::string> rvertices{};
    std::vector<std::set<uint>> edges{0,{0,false}};
    void add(std::string a, std::string b) {
        for (auto &v: {a, b}) {
            if (!vertices.contains(v)) {
                vertices[v] = edges.size();
                rvertices.push_back(v);
                edges.push_back({});
            }
        }
        edges.at(vertices[a]).insert(vertices[b]);
        edges.at(vertices[b]).insert(vertices[a]);
    }
    void remove(uint a, uint b) {
        edges.at(a).erase(b);
        edges.at(b).erase(a);
    }
};

ulong check_partition(graph &g) {
    std::set<uint> seen{0};
    std::queue<uint> q{{0}};
    while (!q.empty()) {
        uint v = q.front();
        q.pop();
        for (auto nei: g.edges[v]) {
            if (!seen.contains(nei)) {
                seen.insert(nei);
                q.push(nei);
            }
        }
    }
    if (seen.size() < g.rvertices.size()) {
        return seen.size() * (g.rvertices.size()-seen.size());
    } else {
        return 0;
    }
}

void walk(graph &g, std::vector<std::vector<uint>> &weights) {
    for (uint k = 0; k < g.edges.size(); ++k) {
        std::set<uint> seen{k};
        std::queue<uint> q{{k}};
        while (!q.empty()) {
            uint v = q.front();
            q.pop();
            for (auto nei: g.edges[v]) {
                if (!seen.contains(nei)) {
                    seen.insert(nei);
                    q.push(nei);
                    weights.at(v).at(nei)++;
                }
            }
        }
    }
}

int main() {
    std::string cur, ref="";
    graph g{};
    uint count = 0;
    while (std::cin >> cur) {
        if (cur.back() == ':') {
            ref = cur.substr(0,cur.length()-1);
        } else {
            g.add(ref, cur);
            count++;
        }
    }
    debug << "size: " << g.rvertices.size() << " vertices " << count << " edges" << std::endl;
    std::vector<std::vector<uint>> weights(g.edges.size(),std::vector<uint>(g.edges.size(),0));
    debug << "walk...";
    walk(g, weights);
    debug << " done" << std::endl;
    std::vector<std::pair<uint,uint>> edges{};
    edges.reserve(g.rvertices.size()*g.rvertices.size());
    for (uint x = 0; x < weights.size(); x++) {
        for (uint y = x+1; y < weights.size(); y++) {
            if (weights.at(x).at(y) != 0) {
                weights.at(x).at(y) += weights.at(y).at(x);
                weights.at(y).at(x) = 0;
                edges.push_back(std::pair(x,y));
            }
        }
    }
    debug << "sort...";
    std::sort(edges.begin(), edges.end(), [weights](const auto &a, const auto &b){
            return weights.at(a.first).at(a.second) > weights.at(b.first).at(b.second);
            });
    debug << " done" << std::endl;
    debug << "top 20 weights:" << std::endl;
    for (auto i = 0u; i < std::min(20u, count); i++) {
        debug << g.rvertices[edges.at(i).first]
                  << " -> " << g.rvertices[edges.at(i).second]
                  << ": " << weights.at(edges.at(i).first).at(edges.at(i).second)
                  << std::endl;
    }
    ulong res = 0;
    for (auto e3 = edges.begin()+2; res == 0 && e3 != edges.end(); e3++) {
        for (auto e1 = edges.begin(); res == 0 && e1 != e3 - 1; e1++) {
            for (auto e2 = e1 + 1; res == 0 && e2 != e3; e2++) {
                auto gcopy = g;
                gcopy.remove(e1->first, e1->second);
                gcopy.remove(e2->first, e2->second);
                gcopy.remove(e3->first, e3->second);
                res = check_partition(gcopy);
                if (res!=0) {
                    debug << "remove " <<
                              g.rvertices[e1->first] << " -> " << g.rvertices[e1->second] << ", " <<
                              g.rvertices[e2->first] << " -> " << g.rvertices[e2->second] << ", " <<
                              g.rvertices[e3->first] << " -> " << g.rvertices[e3->second] << std::endl;
                }
            }
        }
    }
    std::cout << res << std::endl;
}
