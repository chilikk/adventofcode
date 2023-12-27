#include <cfloat>
#include <cmath>
#include <iostream>
#include <list>
#include <regex>
#include <vector>
#include <gmpxx.h>

#define debug 0 && std::clog

//#define CMIN 7LL
//#define CMAX 27LL
#define CMIN 200000000000000LL
#define CMAX 400000000000000LL

struct hail {
    long x, y, z, dx, dy, dz;
};

std::ostream& operator<<(std::ostream& os, const hail &hail)
{
    debug << hail.x << "(" << hail.dx << "):"
          << hail.y << "(" << hail.dy << "):"
          << hail.z << "(" << hail.dz << ")";
    return os;
}

template <class T>
std::ostream& operator<<(std::ostream& os, const std::vector<T>& vs)
{
    bool first = true;
    for (auto& s: vs) {
        if (!first) os << " "; else first = false;
        os << s;
    }
    return os;
}

bool crossed1(hail &h1, hail &h2) {
    debug << h1 << " x " << h2;
    // h1.x + a*h1.dx = h2.x + b*h2.dx
    // h1.y + a*h1.dy = h2.y + b*h2.dy
    //
    // b = (a*h1.dx + h1.x - h2.x)/h2.dx
    // b = (a*h1.dy + h1.y - h2.y)/h2.dy
    //
    // (a*h1.dx + h1.x - h2.x)/h2.dx = (a*h1.dy + h1.y - h2.y)/h2.dy
    // a*h1.dx*h2.dy + h1.x*h2.dy - h2.x*h2.dy = a*h1.dy*h2.dx + h1.y*h2.dx - h2.y * h2.dx
    // a = (h1.y*h2.dx - h2.y*h2.dx + h2.x*h2.dy - h1.x*h2.dy)/(h1.dx*h2.dy - h1.dy*h2.dx)
    //
    // a = (b*h2.dx + h2.x - h1.x)/h1.dx
    // a = (b*h2.dy + h2.y - h1.y)/h1.dy
    //
    // (b*h2.dx + h2.x - h1.x)/h1.dx = (b*h2.dy + h2.y - h1.y)/h1.dy
    // b*h2.dx*h1.dy + h2.x*h1.dy - h1.x*h1.dy = b*h2.dy*h1.dx + h2.y*h1.dx - h1.y*h1.dx
    // b*(h2.dx*h1.dy - h2.dy*h1.dx) = h2.y*h1.dx - h1.y*h1.dx + h1.x*h1.dy - h2.x*h1.dy
    // b = (h2.y*h1.dx - h1.y*h1.dx + h1.x*h1.dy - h2.x*h1.dy)/(h2.dx*h1.dy - h2.dy*h1.dx)

    // check b >= 0 and not parallel
    long num = (h2.y-h1.y)*h1.dx + (h1.x-h2.x)*h1.dy;
    long denom = h2.dx*h1.dy - h2.dy*h1.dx;
    if (num != 0 && num <=> 0 != denom <=> 0) {
        if (denom == 0) {
            debug << " run parallel!" << std::endl;
        } else {
            debug << " in the past for the second" << std::endl;
        }
        return false;
    }
    // check a >= 0 and not parallel
    num = (h1.y-h2.y)*h2.dx + (h2.x-h1.x)*h2.dy;
    denom = -denom;
    if (num != 0 && num <=> 0 != denom <=> 0) {
        debug << " in the past for the first" << std::endl;
        return false;
    }
    // check that intersection point is within the test area for both x and y
    //
    // a = nom/denom
    // h1.x + a*h1.dx >= CMIN
    // h1.x + a*h1.dx <= CMAX
    // h1.y + a*h1.dy >= CMIN
    // h1.y + a*h1.dy <= CMAX
    //
    // num*h1.dx >= (CMIN-h1.x)*denom
    // num*h1.dx <= (CMAX-h1.x)*denom
    // num*h1.dy >= (CMIN-h1.y)*denom
    // num*h1.dy <= (CMAX-h1.y)*denom
    // this causes long overflow
    //bool xwithin = (num*h1.dx >= denom*(CMIN-h1.x)) &&
    //               (num*h1.dx <= denom*(CMAX-h1.x));
    bool xwithin = (num/denom*h1.dx + (num%denom*h1.dx)/denom >= (CMIN-h1.x)) &&
                   (num/denom*h1.dx + (num%denom*h1.dx)/denom + (num%denom*h1.dx%denom==0?0:1) <= (CMAX-h1.x));
    if (!xwithin) {
        debug << " cross outside the test area for x";
    }
    // this causes long overflow
    //bool ywithin = (num*h1.dy >= denom*(CMIN-h1.y)) &&
    //               (num*h1.dy <= denom*(CMAX-h1.y));
    bool ywithin = (num/denom*h1.dy + (num%denom*h1.dy)/denom >= (CMIN-h1.y)) &&
                   (num/denom*h1.dy + (num%denom*h1.dy)/denom + (num%denom*h1.dy%denom==0?0:1) <= (CMAX-h1.y));
    if (!ywithin) {
        debug << " cross outside the test area for y";
    }
    if (xwithin && ywithin) {
        debug << " cross inside the test area";
    }
    debug <<  " at "
            << static_cast<double>(h1.x) + static_cast<double>(num*h1.dx)/static_cast<double>(denom) << ":"
            << static_cast<double>(h1.y) + static_cast<double>(num*h1.dy)/static_cast<double>(denom);
    debug << std::endl;
    return xwithin && ywithin;
}

template<typename T> struct invalid{};
template<> struct invalid<double> { double i = NAN; };
template<> struct invalid<long> { long i = LONG_MAX; };
template<> struct invalid<mpz_class> { mpz_class i = mpz_class(LONG_MAX); };
template<typename T> bool is_invalid(T &x) { return x == invalid<T>().i; }
template<> bool is_invalid(double &x) { return std::isnan(x); }
template<typename T> bool is0(T &x) { return x == 0; }
template<> bool is0(double &x) { return -10*DBL_EPSILON < x && x < 10*DBL_EPSILON; }

// given the system of equations
//   k11*x1 + k12*x2 + k13*x3 + c1 = 0
//   k21*x1 + k22*x2 + k23*x3 + c2 = 0
//   k31*x1 + k32*x2 + k33*x3 + c3 = 0
// the input vectors are
//   {c1, k11, k12, k13}
//   {c2, k21, k22, k23}
//   {c3, k31, k32, k33}
// the output vector is the values of
//   {x1, x2, x3}
template<class T>
std::vector<T> solve(std::list<std::vector<T>> &eqs) {
    debug << "solve:" << std::endl;
    for (auto eq: eqs) {
        debug << eq << std::endl;
    }
    std::list<std::vector<T>> eqs2{};
    for (auto eq1 = eqs.begin(); eq1 != eqs.end(); ++eq1) {
        if (eq1->size() == 1) {
            if (!is0(eq1->front())) {
                debug << "no solution" << std::endl;
                return std::vector<T>{invalid<T>().i};
            }
            continue;
        }
        if (is0(eq1->back())) {
            eqs2.push_back(*eq1);
            eqs2.back().resize(eq1->size()-1);
            continue;
        }
        auto eq2 = eq1;
        for (++eq2; eq2 != eqs.end(); ++eq2) {
            if (is0(eq2->back())) {
                eqs2.push_back(*eq2);
                eqs2.back().resize(eq2->size()-1);
                continue;
            }
            std::vector<T> neweq{};
            for (uint i=0; i<eq1->size()-1; ++i) {
                neweq.push_back(eq2->back()*eq1->at(i) - eq1->back()*eq2->at(i));
            }
            eqs2.push_back(neweq);
        }
        break;
    }
    std::vector<T> solutions{};
    if (eqs.size() == 1 && eqs.front().size() == 2) {
        solutions.push_back(-eqs.front().front()/eqs.front().back());
    } else if (eqs2.size() > 0) {
        solutions = solve(eqs2);
        if (solutions.size() > 0 && is_invalid(solutions.front())) {
            debug << "no solution" << std::endl;
            return solutions;
        }
        bool solved = false;
        for (auto eq: eqs) {
            T acc = eq.at(0);
            for (auto v = solutions.begin(); v != solutions.end(); ++v) {
                acc += *v*eq.at(1+std::distance(solutions.begin(), v));
            }
            if (solved) {
                if (!is0(acc)) {
                    debug << "no solution" << std::endl;
                    return std::vector<T>{invalid<T>().i};
                }
            } else if (!is0(eq.back())) {
                solutions.push_back(-acc/eq.back());
                debug << "solution: " << solutions << std::endl;
                solved = true;
            }
        }
    }
    return solutions;
}

int main() {
    std::list<hail> hails{};
    std::string line;
    std::regex re(R"((\d+), *(\d+), *(\d+) *@ *(-?\d+), *(-?\d+), *(-?\d+))");
    std::smatch sm;
    while (std::getline(std::cin, line)) {
        std::regex_match(line, sm, re);
        hails.push_back(hail{
                std::stoll(sm[1].str()),
                std::stoll(sm[2].str()),
                std::stoll(sm[3].str()),
                std::stoll(sm[4].str()),
                std::stoll(sm[5].str()),
                std::stoll(sm[6].str())
                });
    }
    //part 1
    ulong count = 0;
    for (auto h1 = hails.begin(); h1 != hails.end(); ++h1) {
        for (auto h2 = h1; h2 != hails.end(); ++h2) {
            if (h1 == h2) continue;
            if (crossed1(*h1, *h2)) count++;
        }
    }
    std::cout << count << std::endl;

    // rock: x, y, z, dx, dy, dz
    // hail: xi, yi, zi, dxi, dyi, dzi
    // x + ti*dx = xi + ti*dxi
    // y + ti*dy = yi + ti*dyi
    // z + ti*dz = zi + ti*dzi
    //
    // ti = (x-xi)/(dxi-dx)
    // ti = (y-yi)/(dyi-dy)
    // ti = (z-zi)/(dzi-dz)
    //
    // (x-xi)*(dyi-dy) = (y-yi)*(dxi-dx)
    // (x-xi)*(dzi-dz) = (z-zi)*(dxi-dx)
    // (y-yi)*(dzi-dz) = (z-zi)*(dyi-dy)
    //
    // (1) x*(dyi - dy) + y*(dx - dxi) + xi*(dy - dyi) + yi*(dxi - dx) = 0
    // (2) x*(dzi - dz) + z*(dx - dxi) + xi*(dz - dzi) + zi*(dxi - dx) = 0
    // (3) y*(dzi - dz) + z*(dy - dyi) + yi*(dz - dzi) + zi*(dyi - dy) = 0
    //
    // find two hails with dx1 = dx2, take (1) for both of them
    // x*(dy1 - dy) + y*(dx - dx1) + x1*(dy - dy1) + y1*(dx1 - dx) = 0
    // x*(dy2 - dy) + y*(dx - dx2) + x2*(dy - dy2) + y2*(dx2 - dx) = 0
    //
    // subtract
    // x*(dy1-dy2) + x1*(dy-dy1) - x2*(dy-dy2) + (y1-y2)*(dx1-dx) = 0
    // x*(dy1-dy2) + dx*(y2-y1) + dy*(x1-x2) + (x2*dy2-x1*dy1+y1*dx1-y2*dx2) = 0
    //
    // symmetrically for (2)
    // x*(dz1-dz2) + dx*(z2-z1) + dz*(x1-x2) + (x2*dz2-x1*dz1+z1*dx1-z2*dx2) = 0
    //
    // find two (possibly other) hails with dy3 = dy4, take (1) for both of them
    // x*(dy4 - dy) + y*(dx - dx4) + x4*(dy - dy4) + y4*(dx4 - dx) = 0
    // x*(dy3 - dy) + y*(dx - dx3) + x3*(dy - dy3) + y3*(dx3 - dx) = 0
    //
    // y*(dx3-dx4) + dy*(x4-x3) + dx*(y3-y4) + (x3*dy3-x4*dy4+dx4*y4-dx3*y3) = 0
    //
    // symmetrically for (3)
    // y*(dz3-dz4) + dy*(z4-z3) + dz*(y3-y4) + (z3*dy3-z4*dy4+dz4*y4-dz3*y3) = 0
    //
    // same for dz5 = dz6, (2) and (3)
    // z*(dx5-dx6) + dx*(z5-z6) + dz*(x6-x5) + (x5*dz5-x6*dz6+dx6*z6-z5*dx5) = 0
    // z*(dy5-dy6) + dy*(z5-z6) + dz*(y6-y5) + (y5*dz5-y6*dz6+dy6*z6-z5*dy5) = 0

    hail h1=hail{}, h2=hail{}, h3=hail{}, h4=hail{}, h5=hail{}, h6=hail{};
    bool found_h1_h2 = false, found_h3_h4 = false, found_h5_h6 = false;
    for (auto h = hails.begin(); !found_h1_h2 && h != hails.end(); ++h) {
        h1 = *h;
        auto hh = h;
        for (++hh; !found_h1_h2 && hh != hails.end(); ++hh) {
            if (hh->dx == h1.dx) {
                h2 = *hh;
                found_h1_h2 = true;
            }
        }
    }
    debug << "h1.x == h2.x" << std::endl << "h1: " <<  h1 << std::endl << "h2: " << h2 << std::endl;
    for (auto h = hails.begin(); !found_h3_h4 && h != hails.end(); ++h) {
        h3 = *h;
        auto hh = h;
        for (++hh; !found_h3_h4 && hh != hails.end(); ++hh) {
            if (hh->dy == h3.dy) {
                h4 = *hh;
                found_h3_h4 = true;
            }
        }
    }
    debug << "h3.y == h4.y" << std::endl << "h3: " <<  h3 << std::endl << "h4: " << h4 << std::endl;
    for (auto h = hails.begin(); !found_h5_h6 && h != hails.end(); ++h) {
        h5 = *h;
        auto hh = h;
        for (++hh; !found_h5_h6 && hh != hails.end(); ++hh) {
            if (hh->dz == h5.dz) {
                h6 = *hh;
                found_h5_h6 = true;
            }
        }
    }
    debug << "h5.z == h6.z" << std::endl << "h5: " <<  h5 << std::endl << "h6: " << h6 << std::endl;

    typedef mpz_class solve_t;
    std::vector<solve_t> solution{};

    //test solve
    //std::list<std::vector<double>> eqs0(
    //        {{-1, 1, 1, 0},
    //         {-2, 1, 0, 1},
    //         {-3, 0, 1, 1},
    //         {-3, 1, 1, 1}});
    // expect 0 1 2
    //std::cout << solve(eqs0) << std::endl;
    //exit(0);

    std::list<std::vector<solve_t>> eqs{
        // x*(dy1-dy2) + dx*(y2-y1) + dy*(x1-x2) + (x2*dy2-x1*dy1+y1*dx1-y2*dx2) = 0
        {static_cast<solve_t>(h2.x*h2.dy-h1.x*h1.dy+h1.y*h1.dx-h2.y*h2.dx),
         static_cast<solve_t>(h1.dy-h2.dy),
         0,
         0,
         static_cast<solve_t>(h2.y-h1.y),
         static_cast<solve_t>(h1.x-h2.x),
         0
        },
         // x*(dz1-dz2) + dx*(z2-z1) + dz*(x1-x2) + (x2*dz2-x1*dz1+z1*dx1-z2*dx2) = 0
        {static_cast<solve_t>(h2.x*h2.dz-h1.x*h1.dz+h1.z*h1.dx-h2.z*h2.dx),
         static_cast<solve_t>(h1.dz-h2.dz),
         0,
         0,
         static_cast<solve_t>(h2.z-h1.z),
         0,
         static_cast<solve_t>(h1.x-h2.x)
        },
        // y*(dx3-dx4) + dy*(x4-x3) + dx*(y3-y4) + (x3*dy3-x4*dy4+dx4*y4-dx3*y3) = 0
        {static_cast<solve_t>(h3.x*h3.dy-h4.x*h4.dy+h4.y*h4.dx-h3.dx*h3.y),
         0,
         static_cast<solve_t>(h3.dx-h4.dx),
         0,
         static_cast<solve_t>(h3.y-h4.y),
         static_cast<solve_t>(h4.x-h3.x),
         0
        },
        // y*(dz3-dz4) + dy*(z4-z3) + dz*(y3-y4) + (z3*dy3-z4*dy4+dz4*y4-dz3*y3) = 0
        {static_cast<solve_t>(h3.z*h3.dy-h4.z*h4.dy+h4.y*h4.dz-h3.dz*h3.y),
         0,
         static_cast<solve_t>(h3.dz-h4.dz),
         0,
         0,
         static_cast<solve_t>(h4.z-h3.z),
         static_cast<solve_t>(h3.y-h4.y)
        },
        // z*(dx5-dx6) + dx*(z5-z6) + dz*(x6-x5) + (x5*dz5-x6*dz6+dx6*z6-z5*dx5) = 0
        {static_cast<solve_t>(h5.x*h5.dz-h6.x*h6.dz+h6.z*h6.dx-h5.z*h5.dx),
         0,
         0,
         static_cast<solve_t>(h5.dx-h6.dx),
         static_cast<solve_t>(h5.z-h6.z),
         0,
         static_cast<solve_t>(h6.x-h5.x)
        },
        // z*(dy5-dy6) + dy*(z5-z6) + dz*(y6-y5) + (y5*dz5-y6*dz6+dy6*z6-z5*dy5) = 0
        {static_cast<solve_t>(h5.y*h5.dz-h6.y*h6.dz+h6.z*h6.dy-h5.z*h5.dy),
         0,
         0,
         static_cast<solve_t>(h5.dy-h6.dy),
         0,
         static_cast<solve_t>(h5.z-h6.z),
         static_cast<solve_t>(h6.y-h5.y)
        }
    };
    solution = solve(eqs);
    std::clog << solution << std::endl;
    std::cout << (solution[0]+solution[1]+solution[2]) << std::endl;
}
