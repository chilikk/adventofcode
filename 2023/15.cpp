#include "utils.hpp"
#include <array>
#include <list>

#define debug 0&&std::clog

struct lens {
    std::string label;
    uint fl = 0;
};

std::ostream& operator<<(std::ostream& os, const std::list<lens>& box)
{
    for (auto &slot: box) os << " [" << slot.label << " " << slot.fl << "]";
    return os;
}

int main() {
    std::string input;
    std::cin >> input;
    // part 1
    unsigned char h = 0;
    ulong total1 = 0;
    for (auto &c: input) {
        if (c==',') {
            total1 += h;
            h = 0;
        } else {
            h += c;
            h *= 17;
        }
    }
    total1 += h;
    std::cout << total1 << std::endl;
    // part 2
    std::string label = "";
    std::array<std::list<lens>, 256> boxes = {};
    h = 0;
    for (auto p = input.begin(); p != input.end(); ++p) {
        auto c = *p;
        if (c=='-') {
            debug << label << c << ": box " << (int)h << ": " << boxes.at(h) << std::endl;
            boxes.at(h).remove_if([label](lens x) {return x.label == label;});
        } else if (c == '=') {
            p++;
            uint fl = *p-'0';
            debug << label << c << fl << ": box " << (int)h << ": " << boxes.at(h) << std::endl;
            bool found = false;
            for (auto &slot: boxes.at(h)) {
                if (slot.label == label) {
                    found = true;
                    slot.fl = fl;
                    break;
                }
            }
            if (!found) {
                boxes.at(h).emplace_back(lens{label, fl});
            }
        } else if (c == ',') {
            h = 0;
            label = "";
        } else {
            label += c;
            h += c;
            h *= 17;
        }
    }
    ulong total2 = 0;
    ulong boxid = 1;
    debug << std::endl;
    for (auto &box: boxes) {
        if (!box.empty()) debug << "Box " << boxid << ":" << box << std::endl;
        ulong slotid = 1;
        for (auto &slot: box) {
            total2 += boxid*slotid*slot.fl;
            ++slotid;
        }
        ++boxid;
    }
    std::cout << total2 << std::endl;
}
