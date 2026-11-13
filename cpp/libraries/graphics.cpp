
#include <cstdint>

namespace BloxOS {

struct Color {
    std::uint8_t r;
    std::uint8_t g;
    std::uint8_t b;
    std::uint8_t a;
};

Color accent() {
    return {183, 255, 54, 255};
}

}
