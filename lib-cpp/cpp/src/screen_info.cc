#include "lib.h"
#include <windows.h>
#include <cstdint> // Add this for standard fixed-width types
#include <string>
#include <shellscalingapi.h>

// Define float64_t if not already defined
#ifndef float64_t
typedef double float64_t;
#endif

// --- todo-zm: move this to a seperate file
typedef DPI_AWARENESS_CONTEXT(WINAPI *PFN_SetThreadDpiAwarenessContext)(DPI_AWARENESS_CONTEXT);

static PFN_SetThreadDpiAwarenessContext pSetThreadDpiAwarenessContext = NULL;

static void InitDpiAwarenessFunc(void)
{
    if (!pSetThreadDpiAwarenessContext)
    {
        HMODULE hUser32 = GetModuleHandleW(L"user32.dll");
        if (hUser32)
        {
            pSetThreadDpiAwarenessContext =
                (PFN_SetThreadDpiAwarenessContext)
                    GetProcAddress(hUser32, "SetThreadDpiAwarenessContext");
        }
    }
}

static DPI_AWARENESS_CONTEXT SafeSetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT ctx)
{
    InitDpiAwarenessFunc();
    return pSetThreadDpiAwarenessContext ? pSetThreadDpiAwarenessContext(ctx) : NULL;
}
// ---

BOOL CALLBACK MonitorEnumProc(HMONITOR hMonitor, HDC hdcMonitor, LPRECT lprcMonitor, LPARAM dwData)
{
    // only when running cargo test, to mimic Tauri's DPI awareness
    // BOOL results = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);

    rust::Box<ScreenInfoVec> *pScreenInfos = reinterpret_cast<rust::Box<ScreenInfoVec> *>(dwData);

    MONITORINFOEX monitorInfo;
    monitorInfo.cbSize = sizeof(MONITORINFOEX);

    // temporarily be DPI-unaware
    DPI_AWARENESS_CONTEXT originalContext = SafeSetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_UNAWARE);

    if (GetMonitorInfo(hMonitor, &monitorInfo))
    {
        RECT rcMonitor = monitorInfo.rcMonitor;
        RECT rcWork = monitorInfo.rcWork;

        bool isPrimary = (monitorInfo.dwFlags & MONITORINFOF_PRIMARY) != 0;

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

        (*pScreenInfos)->push_new_screen_info(monitorId, isPrimary, std::move(new_boxed_rect(rcMonitor.left, rcMonitor.top, rcMonitor.right - rcMonitor.left, rcMonitor.bottom - rcMonitor.top)), std::move(new_boxed_rect(rcWork.left, rcWork.top, rcWork.right - rcWork.left, rcWork.bottom - rcWork.top)));
    }

    // Restore original DPI awareness context
    SafeSetThreadDpiAwarenessContext(originalContext);

    return TRUE; // Continue enumeration
}

rust::Box<ScreenInfoVec> get_screen_info()
{
    auto screenInfos = new_boxed_screen_info_vec();

    EnumDisplayMonitors(NULL, NULL, MonitorEnumProc, reinterpret_cast<LPARAM>(&screenInfos));

    return screenInfos;
}
