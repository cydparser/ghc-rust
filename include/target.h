#pragma once

#include <stdint.h>

#if UINTPTR_MAX == 0xFFFFFFFFFFFFFFFFu
#define SIZEOF_VOID_P 8
#define SIZEOF_VOID_P_8
#elif UINTPTR_MAX == 0xFFFFFFFFu
#define SIZEOF_VOID_P 4
#define SIZEOF_VOID_P_4
#else
#error GHC untested on this architecture: sizeof(void *) != 4 or 8
#endif
