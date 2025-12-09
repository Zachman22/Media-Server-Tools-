#ifndef UTILS_H
#define UTILS_H

#include <string>
#include <vector>

namespace MediaServer {
namespace Utils {

// Color codes for beautiful terminal output
namespace Colors {
    const std::string RESET = "\033[0m";
    const std::string BOLD = "\033[1m";
    const std::string RED = "\033[31m";
    const std::string GREEN = "\033[32m";
    const std::string YELLOW = "\033[33m";
    const std::string BLUE = "\033[34m";
    const std::string MAGENTA = "\033[35m";
    const std::string CYAN = "\033[36m";
    const std::string WHITE = "\033[37m";
    const std::string BG_BLUE = "\033[44m";
    const std::string BG_GREEN = "\033[42m";
}

// Box drawing characters
namespace Box {
    const std::string TOP_LEFT = "╔";
    const std::string TOP_RIGHT = "╗";
    const std::string BOTTOM_LEFT = "╚";
    const std::string BOTTOM_RIGHT = "╝";
    const std::string HORIZONTAL = "═";
    const std::string VERTICAL = "║";
    const std::string T_DOWN = "╦";
    const std::string T_UP = "╩";
    const std::string T_RIGHT = "╠";
    const std::string T_LEFT = "╣";
    const std::string CROSS = "╬";
}

std::string centerText(const std::string& text, int width);
void drawBox(const std::string& title, const std::vector<std::string>& content);
void drawHorizontalLine(char c = '=', int width = 60);
std::string colorize(const std::string& text, const std::string& color);
std::string formatSize(long long bytes);
std::string getCurrentTime();

} // namespace Utils
} // namespace MediaServer

#endif // UTILS_H
