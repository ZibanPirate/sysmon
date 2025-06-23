#include "screen_info.h"
#include <windows.h>

float64_t GetScaleFactorFromRegistry()
{
    HKEY hKey;
    DWORD dwType;
    DWORD dwSize = sizeof(DWORD);
    DWORD dwDpi = 96; // Default to 96 (100% scaling)

    if (RegOpenKeyEx(HKEY_CURRENT_USER, TEXT("Control Panel\\Desktop"), 0, KEY_READ, &hKey) == ERROR_SUCCESS)
    {
        RegQueryValueEx(hKey, TEXT("LogPixels"), NULL, &dwType, (LPBYTE)&dwDpi, &dwSize);
        RegCloseKey(hKey);
    }

    return static_cast<float64_t>(dwDpi) / 96.0f;
}

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

        float64_t scaleFactor = GetScaleFactorFromRegistry();

        (*pScreenInfos)->push_new_screen_info(isPrimary, scaleFactor, std::move(new_boxed_rect(rcMonitor.left, rcMonitor.top, rcMonitor.right - rcMonitor.left, rcMonitor.bottom - rcMonitor.top)), std::move(new_boxed_rect(rcWork.left, rcWork.top, rcWork.right - rcWork.left, rcWork.bottom - rcWork.top)));
    }

    return TRUE; // Continue enumeration
}

rust::Box<ScreenInfoVec> get_screen_info()
{
    auto screenInfos = new_boxed_screen_info_vec();

    EnumDisplayMonitors(NULL, NULL, MonitorEnumProc, reinterpret_cast<LPARAM>(&screenInfos));

    return screenInfos;
}
