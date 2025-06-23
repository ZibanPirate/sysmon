#include "lib.h"
#include <atomic>
#include <string>
#include <Windows.h>

static LRESULT CALLBACK WindowProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
    CppMessage message = CppMessage::ScreenInfoChanged;
    message_from_cpp(message);

    return DefWindowProc(hwnd, msg, wParam, lParam);
}

static inline HWND hWnd = nullptr;

DWORD WINAPI MessageLoopThread(LPVOID lpParam)
{
    const wchar_t *CLASS_NAME = L"HiddenWindowClass";

    // Register window class
    WNDCLASSEXW wc = {};
    wc.cbSize = sizeof(WNDCLASSEX);
    wc.lpfnWndProc = WindowProc;
    wc.hInstance = GetModuleHandle(nullptr);
    wc.lpszClassName = CLASS_NAME;
    RegisterClassExW(&wc);

    // Create hidden window
    hWnd = CreateWindowExW(
        0, CLASS_NAME, L"", 0, 0, 0, 0, 0,
        NULL, nullptr, wc.hInstance, nullptr);

    // Message loop
    MSG msg;
    while (GetMessage(&msg, nullptr, 0, 0))
    {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    // Cleanup
    DestroyWindow(hWnd);
    UnregisterClassW(CLASS_NAME, wc.hInstance);
    hWnd = nullptr;
    return 0;
}

static std::atomic<bool> isObserving = false;
static DWORD threadId = 0;

/**
 * Observe screen info changes by creating a hidden window and a message loop in a separate thread.
 */
void start_observing_screen_info()
{
    if (!isObserving.exchange(true))
    {
        CreateThread(
            nullptr, 0, MessageLoopThread, nullptr, 0, &threadId);
    }
}
