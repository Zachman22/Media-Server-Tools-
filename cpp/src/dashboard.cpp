#include "dashboard.h"
#include "utils.h"
#include <iostream>
#include <iomanip>
#include <thread>
#include <chrono>

namespace MediaServer {

Dashboard::Dashboard() : isRunning_(true) {
    // Initialize statistics
    stats_["total_media"] = 1542;
    stats_["movies"] = 348;
    stats_["tv_shows"] = 89;
    stats_["music"] = 1105;
    stats_["active_streams"] = 3;
    stats_["total_users"] = 12;
}

Dashboard::~Dashboard() = default;

void Dashboard::clearScreen() const {
#ifdef _WIN32
    system("cls");
#else
    system("clear");
#endif
}

void Dashboard::displayBeautifulBanner() const {
    using namespace Utils;
    
    std::cout << Colors::CYAN << Colors::BOLD;
    std::cout << "\n";
    std::cout << "  ███╗   ███╗███████╗██████╗ ██╗ █████╗     ███████╗███████╗██████╗ ██╗   ██╗███████╗██████╗ \n";
    std::cout << "  ████╗ ████║██╔════╝██╔══██╗██║██╔══██╗    ██╔════╝██╔════╝██╔══██╗██║   ██║██╔════╝██╔══██╗\n";
    std::cout << "  ██╔████╔██║█████╗  ██║  ██║██║███████║    ███████╗█████╗  ██████╔╝██║   ██║█████╗  ██████╔╝\n";
    std::cout << "  ██║╚██╔╝██║██╔══╝  ██║  ██║██║██╔══██║    ╚════██║██╔══╝  ██╔══██╗╚██╗ ██╔╝██╔══╝  ██╔══██╗\n";
    std::cout << "  ██║ ╚═╝ ██║███████╗██████╔╝██║██║  ██║    ███████║███████╗██║  ██║ ╚████╔╝ ███████╗██║  ██║\n";
    std::cout << "  ╚═╝     ╚═╝╚══════╝╚═════╝ ╚═╝╚═╝  ╚═╝    ╚══════╝╚══════╝╚═╝  ╚═╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝\n";
    std::cout << Colors::RESET;
    std::cout << "\n";
    std::cout << centerText(colorize("✨ Beautiful C++ Dashboard ✨", Colors::YELLOW), 100) << "\n";
    std::cout << centerText("Version 1.0.0 | x64 Architecture", 100) << "\n";
    std::cout << "\n";
}

void Dashboard::displayWelcome() const {
    using namespace Utils;
    
    clearScreen();
    displayBeautifulBanner();
    
    std::vector<std::string> welcome = {
        colorize("🎬 Welcome to Media Server Dashboard!", Colors::GREEN),
        "",
        "Your beautiful media management solution",
        "Built with C++ for maximum performance",
        "",
        colorize("⏰ " + getCurrentTime(), Colors::CYAN)
    };
    
    drawBox("WELCOME", welcome);
    std::cout << "\n";
}

void Dashboard::displayMainMenu() const {
    using namespace Utils;
    
    std::cout << Colors::BOLD << "\n╔══════════════════════════════════════════════════════╗\n";
    std::cout << "║              " << colorize("MAIN MENU", Colors::YELLOW) << "                        ║\n";
    std::cout << "╠══════════════════════════════════════════════════════╣\n" << Colors::RESET;
    
    std::cout << Colors::CYAN << "║" << Colors::RESET;
    std::cout << "  " << colorize("1.", Colors::GREEN) << " 📊 View Dashboard Statistics              ";
    std::cout << Colors::CYAN << "║" << Colors::RESET << "\n";
    
    std::cout << Colors::CYAN << "║" << Colors::RESET;
    std::cout << "  " << colorize("2.", Colors::GREEN) << " 🎬 Browse Media Library                   ";
    std::cout << Colors::CYAN << "║" << Colors::RESET << "\n";
    
    std::cout << Colors::CYAN << "║" << Colors::RESET;
    std::cout << "  " << colorize("3.", Colors::GREEN) << " 🌐 Active Connections                     ";
    std::cout << Colors::CYAN << "║" << Colors::RESET << "\n";
    
    std::cout << Colors::CYAN << "║" << Colors::RESET;
    std::cout << "  " << colorize("4.", Colors::GREEN) << " 💻 System Information                     ";
    std::cout << Colors::CYAN << "║" << Colors::RESET << "\n";
    
    std::cout << Colors::CYAN << "║" << Colors::RESET;
    std::cout << "  " << colorize("5.", Colors::GREEN) << " ⚙️  Server Status                          ";
    std::cout << Colors::CYAN << "║" << Colors::RESET << "\n";
    
    std::cout << Colors::CYAN << "║" << Colors::RESET;
    std::cout << "  " << colorize("0.", Colors::RED) << " 🚪 Exit                                   ";
    std::cout << Colors::CYAN << "║" << Colors::RESET << "\n";
    
    std::cout << Colors::BOLD << "╚══════════════════════════════════════════════════════╝\n" << Colors::RESET;
    
    std::cout << "\n" << colorize("Enter your choice: ", Colors::YELLOW);
}

void Dashboard::displayStats() const {
    using namespace Utils;
    
    std::cout << "\n";
    std::vector<std::string> stats = {
        colorize("📊 MEDIA STATISTICS", Colors::BOLD + Colors::CYAN),
        "",
        colorize("🎬 Movies:      ", Colors::GREEN) + std::to_string(stats_.at("movies")),
        colorize("📺 TV Shows:    ", Colors::GREEN) + std::to_string(stats_.at("tv_shows")),
        colorize("🎵 Music:       ", Colors::GREEN) + std::to_string(stats_.at("music")),
        colorize("📦 Total Items: ", Colors::YELLOW) + std::to_string(stats_.at("total_media")),
        "",
        colorize("👥 Total Users:     ", Colors::BLUE) + std::to_string(stats_.at("total_users")),
        colorize("▶️  Active Streams:  ", Colors::MAGENTA) + std::to_string(stats_.at("active_streams")),
        "",
        colorize("💾 Storage Used:    ", Colors::CYAN) + formatSize(8475893284LL),
        colorize("💿 Storage Free:    ", Colors::GREEN) + formatSize(42853927364LL)
    };
    
    drawBox("DASHBOARD STATISTICS", stats);
}

void Dashboard::displayServerStatus() const {
    using namespace Utils;
    
    std::cout << "\n";
    std::vector<std::string> status = {
        colorize("🟢 Server Status: ", Colors::GREEN) + colorize("ONLINE", Colors::BOLD + Colors::GREEN),
        "",
        colorize("🔄 Uptime:    ", Colors::CYAN) + "15 days, 7 hours, 23 minutes",
        colorize("🌡️  CPU:       ", Colors::YELLOW) + "23% (8 cores)",
        colorize("💾 RAM:       ", Colors::BLUE) + "4.2 GB / 16.0 GB (26%)",
        colorize("🌐 Network:   ", Colors::MAGENTA) + "↓ 12.5 MB/s  ↑ 3.2 MB/s",
        "",
        colorize("✅ All services operational", Colors::GREEN)
    };
    
    drawBox("SERVER STATUS", status);
}

void Dashboard::showMediaLibrary() const {
    using namespace Utils;
    
    std::cout << "\n";
    std::vector<std::string> library = {
        colorize("🎬 MEDIA LIBRARY", Colors::BOLD + Colors::CYAN),
        "",
        "📁 Recent Additions:",
        "  • The Matrix Resurrections (2021)",
        "  • Dune: Part Two (2024)",
        "  • Breaking Bad - S05E16",
        "",
        "🎵 Top Albums:",
        "  • Pink Floyd - Dark Side of the Moon",
        "  • The Beatles - Abbey Road",
        "  • Queen - A Night at the Opera",
        "",
        colorize("✨ Browse full library for more", Colors::YELLOW)
    };
    
    drawBox("MEDIA LIBRARY", library);
}

void Dashboard::showActiveConnections() const {
    using namespace Utils;
    
    std::cout << "\n";
    std::vector<std::string> connections = {
        colorize("🌐 ACTIVE CONNECTIONS", Colors::BOLD + Colors::CYAN),
        "",
        colorize("User: John Doe", Colors::GREEN),
        "  Device: Chrome (Desktop)",
        "  Watching: Breaking Bad S01E03",
        "",
        colorize("User: Jane Smith", Colors::GREEN),
        "  Device: iPad Pro",
        "  Listening: Pink Floyd - Comfortably Numb",
        "",
        colorize("User: Bob Wilson", Colors::GREEN),
        "  Device: Smart TV",
        "  Watching: The Matrix",
        "",
        colorize("Total: 3 active streams", Colors::YELLOW)
    };
    
    drawBox("ACTIVE CONNECTIONS", connections);
}

void Dashboard::showSystemInfo() const {
    using namespace Utils;
    
    std::cout << "\n";
    std::vector<std::string> sysinfo = {
        colorize("💻 SYSTEM INFORMATION", Colors::BOLD + Colors::CYAN),
        "",
        colorize("OS:           ", Colors::GREEN) + "Linux x64 / Windows x64",
        colorize("CPU:          ", Colors::GREEN) + "Intel Core i7-9700K @ 3.6GHz",
        colorize("RAM:          ", Colors::GREEN) + "16 GB DDR4",
        colorize("Storage:      ", Colors::GREEN) + "512 GB NVMe SSD",
        "",
        colorize("Application:  ", Colors::YELLOW) + "Media Dashboard v1.0.0",
        colorize("Architecture: ", Colors::YELLOW) + "x64",
        colorize("Compiler:     ", Colors::YELLOW) + "GCC/MSVC C++17",
        "",
        colorize("✨ Beautiful C++ Executable", Colors::MAGENTA)
    };
    
    drawBox("SYSTEM INFORMATION", sysinfo);
}

void Dashboard::handleUserInput(int choice) {
    using namespace Utils;
    
    switch (choice) {
        case 1:
            displayStats();
            break;
        case 2:
            showMediaLibrary();
            break;
        case 3:
            showActiveConnections();
            break;
        case 4:
            showSystemInfo();
            break;
        case 5:
            displayServerStatus();
            break;
        case 0:
            std::cout << "\n" << colorize("👋 Thank you for using Media Server Dashboard!", Colors::CYAN) << "\n";
            std::cout << colorize("Goodbye! ✨", Colors::YELLOW) << "\n\n";
            isRunning_ = false;
            break;
        default:
            std::cout << "\n" << colorize("❌ Invalid choice. Please try again.", Colors::RED) << "\n";
            break;
    }
    
    if (isRunning_ && choice != 0) {
        std::cout << "\n" << colorize("Press Enter to continue...", Colors::CYAN);
        std::cin.ignore();
        std::cin.get();
    }
}

void Dashboard::run() {
    displayWelcome();
    
    std::this_thread::sleep_for(std::chrono::seconds(2));
    
    while (isRunning_) {
        clearScreen();
        displayBeautifulBanner();
        displayMainMenu();
        
        int choice;
        std::cin >> choice;
        
        if (std::cin.fail()) {
            std::cin.clear();
            std::cin.ignore(10000, '\n');
            choice = -1;
        }
        
        handleUserInput(choice);
    }
}

} // namespace MediaServer
