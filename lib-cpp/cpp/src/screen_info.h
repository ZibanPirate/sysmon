#ifndef SCREEN_INFO_H
#define SCREEN_INFO_H

#include "crate-root/src/lib.rs.h"
#include "rust/cxx.h"

rust::Box<ScreenInfoVec> get_screen_info();

#endif
