#include <stdlib.h>
#include "../coordgenlibs/sketcherMinimizer.h"
#include "get_coordinates.h"

void get_coordinates(size_t n_atoms, uint8_t* atoms, size_t n_bonds, uint16_t* bonds, float* coords) {
    // create the molecule
    auto* min_mol = new sketcherMinimizerMolecule();

    // add all the atoms
    sketcherMinimizerAtom** min_atoms = (sketcherMinimizerAtom**)malloc(n_atoms*sizeof(sketcherMinimizerAtom*));
    for (size_t i = 0; i < n_atoms; i++) {
        min_atoms[i] = min_mol->addNewAtom();
        min_atoms[i]->setAtomicNumber(atoms[i]);
    }

    // add all the bonds
    // bonds are represented as 3 16-bit ints:
    // 1. index of atom 1 in bond
    // 2. index of atom 2 in bond
    // 3. bond multiplicity
    for (size_t i = 0; i < 3*n_bonds; i+=3) {
        uint16_t atom1 = bonds[i];
        uint16_t atom2 = bonds[i+1];
        uint16_t bond_order = bonds[i+2];
        auto bond = min_mol->addNewBond(min_atoms[atom1], min_atoms[atom2]);
        bond->setBondOrder(bond_order);
    }

    // perform minimization
    sketcherMinimizer minimizer;
    minimizer.initialize(min_mol);
    minimizer.runGenerateCoordinates();

    // write outputs
    for (size_t i = 0; i < n_atoms; i++) {
        auto curr_coord = min_atoms[i]->getCoordinates();
        coords[2*i] = curr_coord.x();
        coords[2*i+1] = curr_coord.y();
    }

    free(min_atoms);
}
