#include <iostream>
#include <string_view>

#include "bloxos/platform.h"

namespace {
constexpr std::string_view version = "0.1.0";

void print_help()
{
    std::cout
        << "BloxOS Control Utility " << version << "\n\n"
        << "Usage: bloxctl <command>\n\n"
        << "Commands:\n"
        << "  --version   Show version\n"
        << "  status      Show host platform information\n"
        << "  help        Show this help\n";
}

void print_status()
{
    std::cout
        << "BloxOS development environment\n"
        << "Version:       " << version << '\n'
        << "Host platform: " << bloxos_platform_name() << '\n'
        << "Architecture:  " << bloxos_architecture_name() << '\n'
        << "Status:        running as a host-side prototype\n";
}
}

int main(int argc, char *argv[])
{
    if (argc < 2) {
        print_help();
        return 0;
    }

    const std::string_view command(argv[1]);

    if (command == "--version" || command == "-v") {
        std::cout << "bloxctl " << version << '\n';
        return 0;
    }

    if (command == "status") {
        print_status();
        return 0;
    }

    if (command == "help" || command == "--help" || command == "-h") {
        print_help();
        return 0;
    }

    std::cerr << "Unknown command: " << command
              << "\nRun 'bloxctl help' for usage.\n";
    return 2;
}
