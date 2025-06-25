#include "lib.h"
#include <windows.h>
#include <atomic>

// atomic singleton for network info
std::atomic<uint32_t> download(10000);
std::atomic<uint32_t> upload(200);

rust::Box<CNetworkInfo> get_network_info()
{
    // keep incrementing the singleton by 10000 and 200
    download.fetch_add(10000);
    upload.fetch_add(200);

    // mock for now
    auto networkInfo = new_boxed_network_info(download.load(), upload.load());
    return networkInfo;
}
