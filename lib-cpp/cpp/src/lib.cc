#include "lib.h"

rust::Box<ScreenInfoVec> get_screen_info()
{
    auto screenInfos = new_boxed_screen_info_vec();

    screenInfos->push_new_screen_info(true, std::move(new_boxed_rect(0, 0, 1920, 1080)), std::move(new_boxed_rect(50, 50, 1820, 980)));
    screenInfos->push_new_screen_info(true, std::move(new_boxed_rect(0, 0, 1920, 1080)), std::move(new_boxed_rect(50, 50, 1820, 980)));

    return screenInfos;
}
