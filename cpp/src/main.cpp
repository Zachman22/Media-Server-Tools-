/**
 * Media Server Dashboard - Beautiful C++ Application
 * Version 1.0.0
 * 
 * A stunning media server management tool with beautiful CLI interface
 */

#include <iostream>
#include <cstdlib>
#include "dashboard.h"
#include "utils.h"

int main(int argc, char* argv[]) {
    using namespace MediaServer;
    using namespace MediaServer::Utils;

    // Check for command line arguments
    if (argc > 1) {
        std::string arg = argv[1];
        if (arg == "--version" || arg == "-v") {
            std::cout << Colors::CYAN << Colors::BOLD;
            std::cout << "Media Server Dashboard v1.0.0" << Colors::RESET << std::endl;
            std::cout << "Beautiful C++ x64 Executable" << std::endl;
            return EXIT_SUCCESS;
        } else if (arg == "--help" || arg == "-h") {
            std::cout << Colors::CYAN << Colors::BOLD << "Media Server Dashboard" << Colors::RESET << std::endl;
            std::cout << "\nUsage: media-dashboard [OPTIONS]" << std::endl;
            std::cout << "\nOptions:" << std::endl;
            std::cout << "  -h, --help     Show this help message" << std::endl;
            std::cout << "  -v, --version  Show version information" << std::endl;
            std::cout << "\nA beautiful media server management tool." << std::endl;
            return EXIT_SUCCESS;
        }
    }

    try {
        Dashboard dashboard;
        dashboard.run();
    } catch (const std::exception& e) {
        std::cerr << Colors::RED << "Error: " << e.what() << Colors::RESET << std::endl;
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
