#include "utils.hpp"

#define STEPS1 64
#define STEPS2 26501365UL // = 202300 * 131 + 65
#define STEPS2BREAK 591

#define debug 0 && std::clog

typedef utils::map<char> cmap;

struct step {
    cmap::iterator<cmap> it;
    uint step;
};

inline unsigned long long ul(cmap::iterator<cmap> &it) {
    return ((unsigned long long)((1<<16)+it.x) << 32) + (ulong)((1<<16)+it.y);
}

int main() {
    ulong res1 = 0, res2 = 0;
    cmap m = utils::read_map();
    auto start = m.begin(cmap::RIGHT);
    bool done = false;
    while (!done) {
        auto it = start;
        assert(!it.wrap);
        while (!it.out_of_bounds()) {
            if (*it == 'S') {
                start = it;
                done = true;
                break;
            }
            ++it;
        }
        if (!done) start.next();
    }
    start.wrap = true;
    std::queue<step> q{{step{start, 0}}};
    std::set<unsigned long long> visited = {ul(start)};
    uint curstep = 0;
    while (!q.empty()) {
        auto s = q.front();
        q.pop();
        if (s.step > curstep) {
            auto noblock = (ulong)(s.step-(s.step%2))*(s.step-(s.step%2));
            debug << curstep << ": " << res1 << ", " << (ulong)noblock-res2 << std::endl;
            if (curstep % 262 == 65 || curstep % 131 == 67) {
                std::cout << curstep << ": " << noblock-res2 << std::endl;
            }
            if (curstep == STEPS2BREAK) break;
            curstep = s.step;
        }
        if (s.step % 2 == 0) {
            if (s.step <= STEPS1) {
                *(s.it) = 'O';
                res1++;
            };
        }
        if (s.step % 2 == 1) {
            res2++;
        }
        for (auto &dir: {cmap::UP, cmap::RIGHT, cmap::DOWN, cmap::LEFT}) {
            s.it.set_direction(dir);
            ++s.it;
            assert(s.it.wrap);
            if (!s.it.out_of_bounds() && *(s.it) != '#' && !visited.contains(ul(s.it))) {
                visited.insert(ul(s.it));
                q.push(step{s.it, s.step+1});
            }
            --s.it;
        }
    }
    //debug << m << std::endl;
    //
    // the second part is solved by data analysis outside of this program
    // note that if there were no stones on the map
    // then the number of positions we could reach
    // (for an odd number of steps N) is (N+1)^2
    // i.e. for 26501365 that would be 26501366^2 = 702322399865956
    //
    // Now we only need to count the positions blocked by the stones
    // i.e. the amount this number will be decreased by
    //
    // NOTE that the plane is organized in rombs. there are two different
    // types of rombs = rombs with different stone patterns
    //
    // But each romb can be passed in odd or even positions, so that makes
    // for 4 different sets of blocked positions (marked A, B, C, D below)
    //
    // The rombs are positioned as follows
    //
    //          B
    //         D C
    //        B A B
    //       D C D C
    //      B A B A B
    //     D C D C D C
    //    B A B A B A B
    //     D C D C D C
    //      B A B A B
    //       D C D C
    //        B A B
    //         D C
    //          B
    //
    // NOTE that the size of the map is 131, so the size of one romb is 131
    // Because we start from the middle of the romb we add 65 or 67 steps that
    // cover the initial romb.
    //
    // Conveniently, 26501365 = 202300 * 131 + 65.
    //
    // However, N*131 + 67 works better to start with (there's a quirk for 65).
    //
    // From steps = 67 debug printout we know that
    // the number of blocked positions in A is 612 for this input.
    //
    // At 198 steps (131 + 67) the number of blocked positions is 5250
    // At 329 steps (2*131+67) the number of blocked positions is 14902
    // At 460 steps (3*131+67) the number of blocked positions is 28816
    // At 591 steps (4*131+67) the number of blocked positions is 48120
    //
    // Now solve the system of equations:
    // A = 612
    // A + 4*B + 2*C + 2*D = 5250
    // 9*A + 4*B + 6*C + 6*D = 14902
    // 9*A - A + 4*B - 4*B + 6*(C+D) - 2*(C+D) = 14902-5250
    // (C+D) = (14902-5250-8*612)/4 = 1189
    // B = (5250 - A - 2*(C+D))/4 = (5250-612-2*1189)/4 = 565
    //
    // Verify at 460 steps:
    // 9*A + 16*B + 12*(C+D) = 9*612 + 16*565 + 12*1189 = 28816
    // Verify at 591 steps:
    // 25*A + 16*B + 20*(C+D) = 25*612 + 16*565 + 20*1189 = 48120
    // 
    // Generic formula for the number of blocked positions
    // at (N*131+67) steps when N is even:
    // ((N+1)^2)*A + (N^2)*B + N*(N+1)*(C+D)
    //
    // However, we need (N*131+65). By comparing 327 steps with 329
    // and 589 with 591 we notice that there's a quirk in the corner of one of
    // the rombs which blocks an additional of N positions. So the formula
    // for (N*131+65) for even N is:
    // ((N+1)^2)*A + (N^2)*B + N*(N+1)*(C+D) + N
    //
    // Which makes the formula for the number of positions that can be reached
    // (the target value):
    // (N*131+65+1)^2 - (((N+1)^2)*A + (N^2)*B + N*(N+1)*(C+D) + N)
    // where N = (26501365 - 65)/131 = 202300
    ulong n = (STEPS2 - 65)/131;
    assert(n == 202300);
    res2 = (STEPS2+1)*(STEPS2+1) - ((n+1)*(n+1)*612 + n*n*565 + n*(n+1)*1189 + n);
    std::cout << res1 << std::endl << res2 << std::endl;
}
