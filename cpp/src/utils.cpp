#include "utils.h"
#include <iostream>
#include <sstream>
#include <iomanip>
#include <ctime>
#include <algorithm>

namespace MediaServer {
namespace Utils {

std::string centerText(const std::string& text, int width) {
    int padding = (width - static_cast<int>(text.length())) / 2;
    if (padding < 0) padding = 0;
    return std::string(padding, ' ') + text;
}

void drawBox(const std::string& title, const std::vector<std::string>& content) {
    int maxWidth = static_cast<int>(title.length());
    for (const auto& line : content) {
        maxWidth = std::max(maxWidth, static_cast<int>(line.length()));
    }
    maxWidth += 4; // Add padding

    // Top border
    std::cout << Colors::CYAN << Box::TOP_LEFT;
    for (int i = 0; i < maxWidth; ++i) {
        std::cout << Box::HORIZONTAL;
    }
    std::cout << Box::TOP_RIGHT << Colors::RESET << std::endl;

    // Title
    if (!title.empty()) {
        std::cout << Colors::CYAN << Box::VERTICAL << Colors::RESET;
        std::cout << Colors::BOLD << centerText(title, maxWidth) << Colors::RESET;
        std::cout << Colors::CYAN << Box::VERTICAL << Colors::RESET << std::endl;

        // Separator
        std::cout << Colors::CYAN << Box::T_RIGHT;
        for (int i = 0; i < maxWidth; ++i) {
            std::cout << Box::HORIZONTAL;
        }
        std::cout << Box::T_LEFT << Colors::RESET << std::endl;
    }

    // Content
    for (const auto& line : content) {
        std::cout << Colors::CYAN << Box::VERTICAL << Colors::RESET;
        std::cout << " " << line;
        int spaces = maxWidth - static_cast<int>(line.length()) - 1;
        std::cout << std::string(spaces, ' ');
        std::cout << Colors::CYAN << Box::VERTICAL << Colors::RESET << std::endl;
    }

    // Bottom border
    std::cout << Colors::CYAN << Box::BOTTOM_LEFT;
    for (int i = 0; i < maxWidth; ++i) {
        std::cout << Box::HORIZONTAL;
    }
    std::cout << Box::BOTTOM_RIGHT << Colors::RESET << std::endl;
}

void drawHorizontalLine(char c, int width) {
    std::cout << Colors::CYAN;
    for (int i = 0; i < width; ++i) {
        std::cout << c;
    }
    std::cout << Colors::RESET << std::endl;
}

std::string colorize(const std::string& text, const std::string& color) {
    return color + text + Colors::RESET;
}

std::string formatSize(long long bytes) {
    const char* units[] = {"B", "KB", "MB", "GB", "TB"};
    int unitIndex = 0;
    double size = static_cast<double>(bytes);

    while (size >= 1024.0 && unitIndex < 4) {
        size /= 1024.0;
        ++unitIndex;
    }

    std::ostringstream oss;
    oss << std::fixed << std::setprecision(2) << size << " " << units[unitIndex];
    return oss.str();
}

std::string getCurrentTime() {
    std::time_t now = std::time(nullptr);
    
#ifdef _WIN32
    std::tm localTimeBuf;
    localtime_s(&localTimeBuf, &now);
    std::tm* localTime = &localTimeBuf;
#else
    std::tm* localTime = std::localtime(&now); // Thread-safe in single-threaded context
#endif
    
    std::ostringstream oss;
    oss << std::put_time(localTime, "%Y-%m-%d %H:%M:%S");
    return oss.str();
}

} // namespace Utils
} // namespace MediaServer
