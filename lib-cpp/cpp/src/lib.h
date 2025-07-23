#ifndef LIB_H
#define LIB_H

#include "crate-root/src/lib.rs.h"
#include "rust/cxx.h"

rust::Box<ScreenInfoVec> get_screen_info();

void start_observing_screen_info();

rust::Box<CNetworkInfo> get_network_info();

#endif
