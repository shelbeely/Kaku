cmake_minimum_required(VERSION 3.20)
project(kurrent-kirigami VERSION 0.3.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Qt6 and KDE Frameworks
find_package(ECM REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})

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

# Main executable (Qt/QML UI chrome)
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

# Install
install(TARGETS kurrent-kirigami-ui
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

# Desktop file
install(FILES kurrent.desktop
    DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/applications
)

# AppStream metadata
install(FILES kurrent.appdata.xml
    DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/metainfo
)
