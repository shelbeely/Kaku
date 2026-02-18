cmake_minimum_required(VERSION 3.20)
project(kurrent-kirigami VERSION 0.3.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# KDE Extra CMake Modules
find_package(ECM REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})

include(KDEInstallDirs)
include(KDECMakeSettings)
include(KDECompilerSettings NO_POLICY_SCOPE)
include(ECMInstallIcons)
include(ECMSetupVersion)

ecm_setup_version(${PROJECT_VERSION}
    VARIABLE_PREFIX KURRENT
    VERSION_HEADER "${CMAKE_CURRENT_BINARY_DIR}/kurrent-version.h"
)

# Find Qt6 and KDE Frameworks
find_package(Qt6 REQUIRED COMPONENTS
    Core
    Quick
    Widgets
    Qml
)

find_package(KF6 REQUIRED COMPONENTS
    Kirigami
    I18n
    CoreAddons
    Config
)

# QML/Qt resources
qt_add_resources(QML_RESOURCES
    qml/qml.qrc
)

# Main executable
add_executable(kurrent-kirigami-ui
    src/main.cpp
    src/terminalbridge.cpp
    src/terminalbridge.h
    ${QML_RESOURCES}
)

target_link_libraries(kurrent-kirigami-ui
    Qt6::Core
    Qt6::Quick
    Qt6::Widgets
    Qt6::Qml
    KF6::Kirigami
    KF6::I18n
    KF6::CoreAddons
    KF6::Config
)

# Install binary
install(TARGETS kurrent-kirigami-ui
    DESTINATION ${KDE_INSTALL_BINDIR}
)

# Install XDG desktop entry
install(FILES org.kde.kurrent.desktop
    DESTINATION ${KDE_INSTALL_APPDIR}
)

# Install AppStream metainfo
install(FILES org.kde.kurrent.metainfo.xml
    DESTINATION ${KDE_INSTALL_METAINFODIR}
)

# Install scalable application icon
ecm_install_icons(
    ICONS icons/org.kde.kurrent.svg
    DESTINATION ${KDE_INSTALL_ICONDIR}
    THEME hicolor
)
