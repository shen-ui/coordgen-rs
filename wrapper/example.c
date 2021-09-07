#include <stdlib.h>
#include <stdio.h>
#include "get_coordinates.h"

// This is a duplicate of `coordgenlibs/example_dir/example.cpp` but using
// the C wrapper written specifically for this rust library instead.
int main() {
    uint8_t* atoms = (uint8_t*)malloc(2*sizeof(uint8_t));
    atoms[0] = 7;
    atoms[1] = 6;
    uint16_t* bonds = (uint16_t*)malloc(3*sizeof(uint16_t));
    bonds[0] = 0;
    bonds[1] = 1;
    bonds[2] = 1;

    float* coords = (float*)malloc(4*sizeof(float));
    get_coordinates(2, atoms, 1, bonds, coords);
    for (size_t i = 0; i < 4; i++) {
        printf("%f\n", coords[i]);
    }
    free(coords);
    free(atoms);
    free(bonds);
}
