#include "utils.hpp"
#include <list>
#include <map>
#include <regex>
#include <queue>
#include <set>

#define debug 0 && std::clog

enum modtype { placeholder, bcast, flip, conj };
struct mod {
    std::string name;
    modtype t = placeholder;
    std::list<uint> in{};
    std::list<uint> out{};
    std::bitset<64> state{};
};

std::ostream& operator<<(std::ostream& os, const mod m) {
    if (m.t == flip) os << "%";
    else if (m.t == conj) os << "&";
    os << m.name;
    return os;
}

enum sigval { low, high };
struct signal {
    uint from, to;
    sigval value;
};

uint get(const std::string &name, auto &modules, auto &modnames) {
    if (!modnames.contains(name)) {
        modnames[name] = modules.size();
        modules.push_back(mod{name});
    }
    debug << "get " << name << " = " << modnames.at(name) << std::endl;
    return modnames.at(name);
}

std::string name(modtype mt) {
    switch (mt) {
        case placeholder: return "placeholder";
        case bcast: return "broadcaster";
        case flip: return "flip-flop";
        case conj: return "conjunctor";
    }
}

void process(auto &sigq, auto &modules, auto &counts, auto &round, auto &periods, auto &mon_type) {
    while (!sigq.empty()) {
        auto sig = sigq.front();
        debug << modules.at(sig.from) << " -"
            << (sig.value==low?"low":"high") << "> "
            << modules.at(sig.to) << std::endl;
        if (periods.contains(sig.from) && sig.value==mon_type) {
            debug << "monitor at step " << round
                << ": signal " << modules.at(sig.from) << " -"
                << (sig.value==low?"low":"high") << "-> "
                << modules.at(sig.to) << std::endl;
            if (periods.at(sig.from) == 0) {
                periods[sig.from] = round;
                debug << "set period for " << modules.at(sig.from)
                    << " to " << periods.at(sig.from) << std::endl;
            } else {
                assert(periods.at(sig.from) == round - periods.at(sig.from));
            }
        }
        sigq.pop();
        counts.at(sig.value)++;
        auto &dest = modules.at(sig.to);
        switch (dest.t) {
            case placeholder:
                continue;
            case bcast:
                for (auto &to: dest.out) {
                    sigq.push(signal{sig.to, to, sig.value});
                }
                break;
            case flip:
                if (sig.value == high) continue;
                dest.state.flip(0);
                for (auto &to: dest.out) {
                    sigq.push(signal{sig.to, to, (dest.state[0]?high:low)});
                }
                break;
            case conj:
                dest.state[sig.from] = sig.value==low;
                for (auto &to: dest.out) {
                    sigq.push(signal{sig.to, to, (dest.state.any()?high:low)});
                }
        }
    }
}

void print_graph(auto &modules) {
    std::cout << "digraph 20 {" << std::endl;
    for (auto &m: modules) {
        for (auto &out: m.out) {
            std::cout << "\"" << (m.t==flip?"\\":"") << m
                << "\" -> \""
                << (modules.at(out).t==flip?"\\":"")
                << modules.at(out) << "\"" << std::endl;
        }
    }
    std::cout << "}" << std::endl;
    exit(0);
}

int main() {
    std::string line;
    std::vector<mod> modules{};
    std::map<std::string,uint> modnames{};
    std::regex re(R"(([%&]?)([a-z]+) -> (.+))");
    std::smatch sm;
    uint broadcaster = 0;
    while (std::getline(std::cin, line)) {
        std::regex_match(line, sm, re);
        uint cur = get(sm[2].str(), modules, modnames);
        mod m{sm[2].str()};
        if (sm[2].str() == "broadcaster") {
            broadcaster = cur;
            m.t = bcast;
        }
        if (sm[1].str() == "%")      m.t = flip;
        else if (sm[1].str() == "&") m.t = conj;
        std::string outs = sm[3].str();
        ulong split = outs.find_first_of(',');
        uint out = 0;
        while (split != std::string::npos) {
            out = get(outs.substr(0, split), modules, modnames);
            m.out.push_back(out);
            modules.at(out).in.push_back(cur);
            modules.at(out).state.set(cur);
            outs = outs.substr(split+2);
            split = outs.find_first_of(',');
        }
        out = get(outs, modules, modnames);
        m.out.push_back(out);
        modules.at(out).in.push_back(cur);
        modules.at(out).state.set(cur);
        m.in = modules.at(cur).in;
        m.state = modules.at(cur).state;
        modules.at(cur) = m;
        debug << m.name << " " << name(m.t) << " state " << m.state << " out " << m.out << std::endl;
    }
    for (auto &m: modules) {
        if (m.t != conj) {
            m.state.reset();
        }
    }
    debug << std::endl;
    std::queue<signal> sigq{};
    std::array<ulong,2> total1{0, 0};
    ulong cnt2 = 0;
    // uncomment print_graph & collect the graph with
    // ./20 < 20.txt | dot -Tpng -o20.png
    //print_graph(modules);
    // do some custom processing based on studying the graph
    std::map<uint,uint> periods2{std::pair{modnames.at("rx"), 0}};
    for (auto &i: {0, 1}) {
        assert(periods2.size() == 1);
        auto ref = periods2.begin() -> first;
        assert(modules.at(ref).t == conj || modules.at(ref).name == "rx");
        periods2.clear();
        for (auto &in: modules.at(ref).in) {
            periods2.insert(std::pair{in, 0});
        }
    }
    assert(periods2.size() > 1);
    sigval monitor_type = high;
    bool done = false;
    while (!done) {
        sigq.push(signal{broadcaster, broadcaster, low});
        cnt2++;
        process(sigq, modules, total1, cnt2, periods2, monitor_type);
        debug << std::endl;
        if (cnt2 == 1000) {
            std::cout << total1[0]*total1[1] << std::endl;
        }
        done = true;
        for (auto &v: periods2) {
            if (v.second == 0) done = false;
        }
    }
    cnt2 = 1;
    for (auto &v: periods2) {
        cnt2 *= v.second;
    }
    std::cout << cnt2 << std::endl;
}
