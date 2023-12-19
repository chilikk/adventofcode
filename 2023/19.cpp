#include "utils.hpp"
#include <list>
#include <map>
#include <regex>

#define debug1 0 && std::clog
#define debug2 0 && std::clog

enum cat { x, m, a, s };
enum operation { lt, gt };
enum outcome { accept, reject, jmp };
struct part {
    unsigned short int x, m, a, s;
    uint rating() const { return (uint)x+m+a+s; }
};
struct rule {
    bool condition = false;
    cat c = x;
    operation op = lt;
    short int cmp = 0;
    outcome res = jmp;
    uint jump = UINT_MAX;
};
struct workflow {
    std::string name;
    std::list<rule> rules{};
};
struct wfres {
    bool applied = false;
    outcome out = reject;
};

void add_rule(std::string spec, std::string &name, auto &wfnames, auto &workflows) {
    rule r = rule{};
    ulong split = spec.find_first_of(':');
    if (split != std::string::npos) {
        auto cond = spec.substr(0, split);
        r.condition = true;
        auto split2 = cond.find_first_of("<>");
        auto what = cond.substr(0,split2);
        if (what == "x") r.c = x;
        else if (what == "m") r.c = m;
        else if (what == "a") r.c = a;
        else if (what == "s") r.c = s;
        else assert(false);
        if (cond[split2] == '<') r.op = lt;
        else if (cond[split2] == '>') r.op = gt;
        else assert(false);
        r.cmp = (short int)std::stoi(cond.substr(split2+1));
        spec = spec.substr(split+1);
    } else {
        r.condition = false;
    }
    if (spec == "A") r.res = accept;
    else if (spec == "R") r.res = reject;
    else {
        r.res = jmp;
        if (!wfnames.contains(spec)) {
            workflows.emplace_back(workflow{spec, std::list<rule>{}});
            wfnames[spec] = workflows.size()-1;
        }
        r.jump = wfnames.at(spec);
    }
    if (!wfnames.contains(name)) {
        workflows.emplace_back(workflow{name, std::list<rule>{}});
        wfnames[name] = workflows.size()-1;
    }
    workflows.at(wfnames.at(name)).rules.push_back(r);
}

wfres apply_workflow(part &p, auto &wf, auto &workflows) {
    debug1 << "eval part "
        << p.x << "/" << p.m << "/" << p.a << "/" << p.s
        << "(" << p.rating() << ") "  << wf.name << std::endl;
    for (auto &r: wf.rules) {
        debug1 << "next rule" << std::endl;
        if (r.condition) {
            unsigned short int what = 0;
            switch (r.c) {
                case x: what = p.x; break;
                case m: what = p.m; break;
                case a: what = p.a; break;
                case s: what = p.s; break;
            }
            if (!((r.op == lt && what < r.cmp)
                    || (r.op == gt && what > r.cmp))) continue;
        }
        if (r.res == accept) return wfres{true, accept};
        if (r.res == reject) return wfres{true, reject};
        if (r.res == jmp) return apply_workflow(p, workflows.at(r.jump), workflows);
    }
    return wfres{false};
}

ulong count_ranges(auto range, auto c, auto &p, ulong rangelen, auto wf, auto &workflows) {
    ulong ret = 0;
    ulong hererangelen = 0;
    for (auto i = range->begin(); i!=range->end(); ++i) {
        debug2 << "looking at " << c << " = " << *i << std::endl;
        if (i+1 != range->end()) {
            debug2 << "next " << c << " = " << *(i+1) << std::endl;
            hererangelen = rangelen * ((*(i+1))-(*i));
        } else {
            hererangelen = rangelen * (4001-(*i));
        }
        switch (c) {
            case x: p.x = *i; break;
            case m: p.m = *i; break;
            case a: p.a = *i; break;
            case s: p.s = *i; break;
        }
        if (c == s) {
            auto res = apply_workflow(p, wf, workflows);
            assert(res.applied);
            debug2 << "part "
                << p.x << "/" << p.m << "/" << p.a << "/" << p.s
                << " rangelen " << hererangelen << " "
                << (res.out == accept?"accept":"reject") << std::endl;
            if (res.out == accept) {
                ret += hererangelen;
            }
        } else {
            ret += count_ranges(range+1, c+1, p, hererangelen, wf, workflows);
        }
    }
    return ret;
}

int main() {
    std::string line;
    std::regex re1(R"(([a-z]+)\{(.*)\})");
    std::regex re2(R"(([a-z]+)([<>])([0-9]+),?)");
    std::smatch sm;
    std::map<std::string, uint> wfnames{};
    std::vector<workflow> workflows{};
    std::list<uint> wforder{};
    while (std::getline(std::cin, line)) {
        if (line.empty()) break;
        std::regex_match(line, sm, re1);
        std::string name = sm[1].str();
        line = sm[2].str();
        ulong split = 0;
        while ((split = line.find_first_of(','))) {
            if (split == std::string::npos) break;
            add_rule(line.substr(0, split), name, wfnames, workflows);
            line = line.substr(split+1);
        }
        add_rule(line, name, wfnames, workflows);
        wforder.push_back(wfnames.at(name));
    }
    std::regex re3(R"(\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\})");
    std::vector<part> parts{};
    while (std::getline(std::cin, line)) {
        std::regex_match(line, sm, re3);
        unsigned short int x = std::stoi(sm[1].str());
        unsigned short int m = std::stoi(sm[2].str());
        unsigned short int a = std::stoi(sm[3].str());
        unsigned short int s = std::stoi(sm[4].str());
        parts.emplace_back(part{x,m,a,s});
    }
    uint in = wfnames.at("in");
    ulong total = 0;
    for (auto &p: parts) {
        auto res = apply_workflow(p, workflows.at(in), workflows);
        assert(res.applied);
        debug1 << (res.out == accept?"accept":"reject") << std::endl;
        if (res.out == accept) total += p.rating();
    }
    std::cout << total << std::endl;
    std::array<std::vector<uint>, 4> ranges{std::vector<uint>{1},{1},{1},{1}};
    for (auto &wf: workflows) {
        for (auto &r: wf.rules) {
            if (r.condition) {
                if (r.op == lt) ranges.at(r.c).push_back(r.cmp);
                if (r.op == gt) ranges.at(r.c).push_back(r.cmp+1);
            }
        }
    }
    uint totalcheck = 1;
    for (auto &rng: ranges) {
        std::sort(rng.begin(), rng.end());
        auto end = std::unique(rng.begin(), rng.end());
        rng.resize(std::distance(rng.begin(), end));
        debug2 << rng << std::endl;
        totalcheck *= rng.size();
    }
    debug2 << "total to check: " << totalcheck << std::endl;
    auto p = part{};
    auto count = count_ranges(ranges.begin(), x, p, 1, workflows.at(in), workflows);
    std::cout << count << std::endl;
}
