#include "lib.h"
#include <windows.h>

BOOL CALLBACK MonitorEnumProc(HMONITOR hMonitor, HDC hdcMonitor, LPRECT lprcMonitor, LPARAM dwData)
{
    rust::Box<ScreenInfoVec> *pScreenInfos = reinterpret_cast<rust::Box<ScreenInfoVec> *>(dwData);

    MONITORINFO monitorInfo;
    monitorInfo.cbSize = sizeof(MONITORINFO);

    if (GetMonitorInfo(hMonitor, &monitorInfo))
    {
        RECT rcMonitor = monitorInfo.rcMonitor;
        RECT rcWork = monitorInfo.rcWork;

        bool isPrimary = (monitorInfo.dwFlags & MONITORINFOF_PRIMARY) != 0;

        (*pScreenInfos)->push_new_screen_info(isPrimary, std::move(new_boxed_rect(rcMonitor.left, rcMonitor.top, rcMonitor.right - rcMonitor.left, rcMonitor.bottom - rcMonitor.top)), std::move(new_boxed_rect(rcWork.left, rcWork.top, rcWork.right - rcWork.left, rcWork.bottom - rcWork.top)));
    }

    return TRUE; // Continue enumeration
}

rust::Box<ScreenInfoVec> get_screen_info()
{
    auto screenInfos = new_boxed_screen_info_vec();

    // Enumerate all display monitors
    EnumDisplayMonitors(NULL, NULL, MonitorEnumProc, reinterpret_cast<LPARAM>(&screenInfos));

    return screenInfos;
}
