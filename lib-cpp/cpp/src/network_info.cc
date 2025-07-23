#include "lib.h"

#include <winsock2.h>
#include <ws2tcpip.h>

#pragma comment(lib, "Iphlpapi.lib")
#include <iphlpapi.h>

rust::Box<CNetworkInfo> get_network_info()
{
    ULONG64 total_sent(0);
    ULONG64 total_received(0);

    PMIB_IF_TABLE2 if_table = nullptr;
    if (GetIfTable2(&if_table) != NO_ERROR)
    {
        throw std::runtime_error("GetIfTable2 failed");
    }

    for (ULONG i = 0; i < if_table->NumEntries; ++i)
    {
        const auto &entry = if_table->Table[i];
        total_received += entry.InOctets;
        total_sent += entry.OutOctets;
    }

    auto networkInfo = new_boxed_network_info(total_sent, total_received);
    return networkInfo;
}
