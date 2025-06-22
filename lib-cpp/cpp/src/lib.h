#ifndef LIB_H
#define LIB_H

#pragma once
#include "crate-root/src/lib.rs.h"
#include "rust/cxx.h"

rust::Box<ScreenInfoVec> get_screen_info();

#endif
