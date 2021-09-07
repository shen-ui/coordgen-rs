#ifndef __WRAPPER_H__
#define __WRAPPER_H__

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

extern void get_coordinates(size_t n_atoms, uint8_t* atoms, size_t n_bonds, uint16_t* bonds, float* coords);

#ifdef __cplusplus
}
#endif

#endif // ifndef __WRAPPER_H__
