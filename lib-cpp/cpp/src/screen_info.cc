#include "lib.h"
#include <windows.h>
#include <cstdint> // Add this for standard fixed-width types
#include <string>

// Define float64_t if not already defined
#ifndef float64_t
typedef double float64_t;
#endif

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

    MONITORINFOEX monitorInfo;
    monitorInfo.cbSize = sizeof(MONITORINFOEX);

    if (GetMonitorInfo(hMonitor, &monitorInfo))
    {
        RECT rcMonitor = monitorInfo.rcMonitor;
        RECT rcWork = monitorInfo.rcWork;

        bool isPrimary = (monitorInfo.dwFlags & MONITORINFOF_PRIMARY) != 0;

        float64_t scaleFactor = GetScaleFactorFromRegistry();

        // Convert szDevice to std::string (UTF-8)
        std::string monitorId;
#ifdef UNICODE
        // szDevice is wchar_t[32]
        int len = WideCharToMultiByte(CP_UTF8, 0, monitorInfo.szDevice, -1, nullptr, 0, nullptr, nullptr);
        if (len > 0)
        {
            std::string temp(len - 1, 0); // exclude null terminator
            WideCharToMultiByte(CP_UTF8, 0, monitorInfo.szDevice, -1, &temp[0], len, nullptr, nullptr);
            monitorId = std::move(temp);
        }
#else
        // szDevice is char[32]
        monitorId = monitorInfo.szDevice;
#endif

        (*pScreenInfos)->push_new_screen_info(monitorId, isPrimary, scaleFactor, std::move(new_boxed_rect(rcMonitor.left, rcMonitor.top, rcMonitor.right - rcMonitor.left, rcMonitor.bottom - rcMonitor.top)), std::move(new_boxed_rect(rcWork.left, rcWork.top, rcWork.right - rcWork.left, rcWork.bottom - rcWork.top)));
    }

    return TRUE; // Continue enumeration
}

rust::Box<ScreenInfoVec> get_screen_info()
{
    auto screenInfos = new_boxed_screen_info_vec();

    EnumDisplayMonitors(NULL, NULL, MonitorEnumProc, reinterpret_cast<LPARAM>(&screenInfos));

    return screenInfos;
}
