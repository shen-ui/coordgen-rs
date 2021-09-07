#include <stdlib.h>
#include <stdio.h>
#include "get_coordinates.h"

// This is a duplicate of `coordgenlibs/example_dir/example.cpp` but using
// the C wrapper written specifically for this rust library instead.
int main(int argc, char** argv) {
    assert(argc == 4);
    int* atoms = (int*)malloc(2*sizeof(int));
    atoms[0] = 7;
    atoms[1] = 6;
    int* bonds = (int*)malloc(3*sizeof(int));
    bonds[0] = 0;
    bonds[1] = 1;
    bonds[2] = 1;

    float* coords = get_coordinates(2, atoms, 1, bonds);
    for (size_t i = 0; i < 4; i++) {
        printf("%f\n", coords[i]);
    }
    free_coordinates(coords);
    free(atoms);
    free(bonds);
}
