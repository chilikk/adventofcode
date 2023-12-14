#include "utils.hpp"
#include <regex>
#include <atomic>
#include <thread>
#include "assert.h"

using std::clog;
using std::cout;
using std::endl;

struct seq {
    char type = '.';
    uint len = 0;
};

std::ostream& operator<<(std::ostream& os, const seq& s) {
    os << s.len << s.type;
    return os;
}

std::vector<seq> str2seq(const std::string str) {
    std::vector<seq> ret = {};
    seq cur = {};
    for (const char& c: str) {
        if (cur.type == c && c != '?') {
            cur.len++;
        } else {
            if (cur.len > 0) ret.push_back(cur);
            cur = seq{c, 1};
        }
    }
    if (cur.len > 0) ret.push_back(cur);
    return ret;
}

ulong combs(char prevtype, auto cur, const auto &vsend, auto ni, const auto &numsend, auto ti, auto sti, char assume) {
    auto next = cur;
    while (cur != vsend) {
        if (next == cur) next++;
        auto curtype = cur->type;
        if (assume != ' ') {
            assert(curtype == '?');
            curtype = assume;
            assume = ' ';
        }
        if (curtype == '#') {
            uint curlen = cur->len;
            while (cur != next) {
                if (ni == numsend || curlen > *ni) {
                    return 0;
                }
                if (curlen == *ni && (next == vsend || next->type != '#')) {
                    ni++;
                    ti++;
                    prevtype = '#';
                    std::advance(sti, std::distance(cur, next));
                    cur = next;
                    if (ni == numsend && next != vsend) {
                        next++;
                        while (next != vsend) {
                            if (next->type == '#') {
                                return 0;
                            }
                            next++;
                        }
                        return 1;
                    }
                    if (next != vsend && *sti < *ti) {
                        return 0;
                    }
                } else if (curlen < *ni && (next == vsend || next->type == '.')) {
                    return 0;
                } else {
                    curlen += next->len;
                    next++;
                }
            }
        } else if (curtype == '?') {
            ulong total = 0;
            if (prevtype == '#') {
                assume = '.';
            } else {
                for (auto c: {'.', '#'}) {
                    total += combs(prevtype, cur, vsend, ni, numsend, ti, sti, c);
                }
                return total;
            }
        } else {
            prevtype = curtype;
            std::advance(sti, std::distance(cur, next));
            cur = next;
        }
    }
    if (cur == vsend) {
        return ni == numsend;
    }
    assert(false);
}

ulong split(auto vsbegin, const auto &vsend, const auto &numbegin, const auto &numsend, auto ti, auto sti) {
    while (vsbegin->type == '.') {
        vsbegin++;
        sti++;
    }
    auto cur = vsbegin;
    auto len = 0;
    auto stibegin = sti;
    while (cur->type != '.' && cur != vsend) {
        len += cur->len;
        cur++;
        sti++;
    }
    if (cur == vsend) {
        return combs('.', vsbegin, vsend, numbegin, numsend, ti, stibegin, ' ');
    }
    auto numptr = numbegin;
    auto tibegin = ti;
    auto acc = -1;
    ulong total = 0;
    while (numptr != numsend) {
        if (acc <= len && *ti <= *sti) {
            auto half = combs('.', vsbegin, cur, numbegin, numptr, tibegin, stibegin, ' ');
            if (half) {
                total += half * split(cur, vsend, numptr, numsend, ti, sti);
            }
        }
        acc += 1 + *numptr;
        numptr++;
        ti++;
    }
    if (acc <= len) {
        auto half = combs('.', cur, vsend, numsend, numsend, ti, sti, ' ');
        total += half * combs('.', vsbegin, cur, numbegin, numsend, tibegin, stibegin, ' ');
    }
    return total;
}

ulong solve(const auto &seq, const auto &nums) {
    auto numiter = nums.begin();
    auto numiterend = nums.end();
    auto seqiter = seq.begin();
    auto seqiterend = seq.end();
    std::vector<uint> tails(nums);
    uint prev = 0;
    for (auto t = tails.rbegin(); t != tails.rend(); t++) {
        *t += prev + 1;
        prev = *t;
    }
    std::vector<uint> stails = {};
    stails.resize(seq.size());
    prev = 0;
    auto s = seq.rbegin();
    auto st = stails.rbegin();
    for (; s != seq.rend(); s++, st++) {
        *st = prev + s->len;
        prev = *st;
    }
    auto tailsiter = tails.begin();
    auto stailsiter = stails.begin();
    return split(seqiter, seqiterend, numiter, numiterend, tailsiter, stailsiter);
}

void do_line(std::string line, auto id, auto &total1, auto &total2) {
    clog << "start job " << id << endl;
    std::vector<uint> nums;
    std::regex re(R"(([?.#]+) ([0-9,]+))");
    std::smatch m;
    std::regex_match(line, m, re);
    std::string spec = m[1].str();
    std::string intspec = m[2].str();
    nums = utils::ints<uint>(intspec);
    auto seq = str2seq(spec);
    auto res1 = solve(seq, nums);
    total1 += res1;
    clog << "job " << id << " part 1 : " << res1 << " combinations" << endl;
    seq = str2seq(spec + "?" +  spec + "?" + spec + "?" + spec + "?" + spec);
    nums = utils::ints<uint>(intspec + "," + intspec + "," + intspec + "," + intspec + "," + intspec);
    auto res2 = solve(seq, nums);
    clog << "job " << id << " part 2 : " << res2 << " combinations" << endl;
    total2 += res2;
}

struct jobspec {
    std::vector<std::string> jobs = {};
    std::atomic_uint jobctr = 0;
    std::atomic_ullong total1 = 0;
    std::atomic_ullong total2 = 0;
};

void do_thread(jobspec *js) {
    uint jobid = 0;
    while ((jobid = js->jobctr++) < js->jobs.size()) {
        do_line(js->jobs[jobid], jobid, js->total1, js->total2);
    }
}

int main() {
    std::string line;
    std::vector<std::thread> threads = {};
    jobspec js = {};
    while (std::getline(std::cin, line)) {
        js.jobs.push_back(line);
    }
    threads.resize(std::thread::hardware_concurrency());
    for (auto &i: threads) {
        i = std::thread(do_thread, &js);
    }
    for (auto &t: threads) t.join();
    cout << js.total1 << endl << js.total2 << endl;
}
