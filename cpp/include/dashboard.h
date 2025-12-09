#ifndef DASHBOARD_H
#define DASHBOARD_H

#include <string>
#include <vector>
#include <map>

namespace MediaServer {

class Dashboard {
public:
    Dashboard();
    ~Dashboard();

    void displayWelcome() const;
    void displayMainMenu() const;
    void displayStats() const;
    void displayServerStatus() const;
    void displayBeautifulBanner() const;
    
    void run();

private:
    std::map<std::string, int> stats_;
    bool isRunning_;
    
    void handleUserInput(int choice);
    void showMediaLibrary() const;
    void showActiveConnections() const;
    void showSystemInfo() const;
    void clearScreen() const;
};

} // namespace MediaServer

#endif // DASHBOARD_H
