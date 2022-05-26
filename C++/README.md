# C++ Implementation

## Building

Ensure you either have visual studio (Windows) or g++ (Linux) installed.  Ensure that it can compile C++17.

To build use the folowing command in the base folder of this project:

    scons -j128

To clean the build files use the following command:

    scons -c

## Running

When running the C++ executable ensure that you add executable permissions:

    chmod +x build/djikstras_search_Linux

Then to run the executable:

    ./builds/djikstras_search_Linux