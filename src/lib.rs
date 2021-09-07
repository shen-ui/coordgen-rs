#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::{
    collections::{HashMap, HashSet},
    fmt,
};

/// A pre-condition violation that may cause undefined behavior when generating coordinates.
///
/// These pre-conditions correspond to those documented in [`gen_coords_unchecked`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    /// An atom index in a bond exceeded the maximum possible value
    AtomIdx {
        /// The invalid atom index that was found
        provided: u16,
        /// The index of the invalid bond
        bond_idx: usize,
        /// The index of the atom within the bond (0 or 1)
        atom: u8,
        /// The maximum valid atom index
        max: usize,
    },
    /// One of the bond multiplicities was not `∈ {1,2,3}`
    BondMult {
        /// The invalid bond multiplicity that was found
        provided: u16,
        /// The index of the invalid bond
        bond_idx: usize,
    },
    /// One of the atoms contained an atomic number for an atom that doesn't exist
    AtomicNum {
        /// The invalid atomic number that was found
        provided: u8,
        /// The index of the invalid atom
        atom_idx: usize,
    },
    /// The provided bonds contained a parallel bond
    ParallelBonds {
        /// The first parallel bond found
        bond1: usize,
        /// The second parallel bond found
        bond2: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AtomIdx {
                provided,
                bond_idx,
                atom,
                max,
            } => {
                write!(f, "bond {} contained atomic index {} for coincident atom {}, but only {} atoms exist", bond_idx, provided, atom, max)
            }
            Self::BondMult { provided, bond_idx } => {
                write!(
                    f,
                    "bond {} contained multiplicity {}, but it must be 1, 2, or 3",
                    bond_idx, provided
                )
            }
            Self::AtomicNum { provided, atom_idx } => {
                write!(
                    f,
                    "atom {} had an atomic number {}, no such atom currently exists",
                    atom_idx, provided
                )
            }
            Self::ParallelBonds { bond1, bond2 } => {
                write!(
                    f,
                    "bond {} and bond {} connect the same two atoms",
                    bond1, bond2
                )
            }
        }
    }
}

impl std::error::Error for Error {}

extern {
    fn get_coordinates(
        n_atoms: usize,
        atoms: *const u8,
        n_bonds: usize,
        bonds: *const u16,
        coords: *mut f32,
    );
}

/// Computes 2D coordinates for a molecule given atomic numbers and bond multiplicities.
///
/// `atoms` is a slice of bytes representing the atomic numbers of constituent atoms.
///
/// `bonds` is a slice of 3-membered arrays where elements are:
/// 1. The index of the first coincident atom in `atoms`
/// 2. The index of the second coincident atom in `atoms`
/// 3. The bond multiplicity where 1 is a single bond, 2 is a double bond, and 3 is a triple bond
///
/// Returns a vector of `(x, y)` coordinates co-indexed with the `atoms` parameter (i.e.
/// `gen_coord_unchecked()[0]` is the coordinates for `atoms[0]`).
///
/// # Safety
/// This performs no checks at all for validity in release builds.
/// When debug asserts are enabled this will panic when:
/// - Any of the atom indices in the `bonds` slice exceeds the `atoms.len()`
/// - Any bond in `bonds` would cause a parallel undirected edge in the molecular graph
/// - Any of the bond multiplicities in the `bonds` slice aren't `∈ 1..=3`
/// - Any of the members of `atoms` aren't `∈ 1..=118` (i.e. they aren't valid atomic numbers)
///
/// # Panics
/// Panics when `debug_assertions` are enabled if any of the safety pre-conditions are violated.
/// This makes this function safe, since it will panic if anything that would cause undefined
/// behavior across the FFI boundary exists.
pub unsafe fn gen_coords_unchecked(atoms: &[u8], bonds: &[[u16; 3]]) -> Vec<(f32, f32)> {
    let n_atoms = atoms.len();
    let n_bonds = bonds.len();

    // debug asserts for conditions from doc comment, in the order they appear
    debug_assert!(bonds
        .iter()
        .all(|[atom1, atom2, _]| (*atom1 as usize) < n_atoms && (*atom2 as usize) < n_atoms));
    debug_assert_eq!(
        bonds
            .iter()
            .map(|[atom1, atom2, _]| {
                let mut atoms = [*atom1, *atom2];
                atoms.sort_unstable();
                atoms
            })
            .collect::<HashSet<[u16; 2]>>()
            .len(),
        bonds.len()
    );
    debug_assert!(bonds.iter().all(|[_, _, mult]| (1..=3).contains(mult)));
    debug_assert!(atoms
        .iter()
        .all(|atomic_num| (1..=118).contains(atomic_num)));

    // this is sufficient
    let atoms = atoms.as_ptr();

    // now we need to convert the bonds
    let bonds = bonds.iter().copied().flatten().collect::<Vec<u16>>();
    let bonds = bonds.as_ptr();

    // first allocate a raw pointer to use over the FFI boundary
    // we get it from a vec because then we can turn the result back into a vec
    let mut raw_coords = Vec::<f32>::with_capacity(2 * n_atoms);

    // the unsafe operations all here
    get_coordinates(n_atoms, atoms, n_bonds, bonds, raw_coords.as_mut_ptr());
    raw_coords.set_len(2 * n_atoms);

    // convert to coordinate tuples
    // FIXME: use a pointer case on a repr(C) type to avoid double allocations
    let mut coords = Vec::with_capacity(n_atoms);
    for idx in 0..n_atoms {
        let coord = (raw_coords[2 * idx], raw_coords[2 * idx + 1]);
        coords.push(coord);
    }

    coords
}

/// A safe wrapper around [`gen_coords_unchecked`].
///
/// Instead of silently possible performing UB this function checks all pre-conditions and returns
/// an [`Error`] if they aren't satisfied. This is at the cost of extra checks.
pub fn gen_coords(atoms: &[u8], bonds: &[[u16; 3]]) -> Result<Vec<(f32, f32)>, Error> {
    let mut seen_bonds = HashMap::<[u16; 2], usize>::new();
    for (bond_idx, [atom1, atom2, mult]) in bonds.iter().enumerate() {
        // check for parallel bonds
        let mut bond = [*atom1, *atom2];
        bond.sort_unstable();
        if let Some(bond1) = seen_bonds.insert(bond, bond_idx) {
            return Err(Error::ParallelBonds {
                bond1,
                bond2: bond_idx,
            });
        }

        let max = atoms.len();
        let mut invalid_atom = false;
        let mut provided = *atom1;
        let mut atom = 0;

        // first atom in bond is invalid
        if *atom1 as usize >= max {
            invalid_atom = true;
        }
        // second atom in bond is invalid
        else if *atom2 as usize >= max {
            invalid_atom = true;
            provided = *atom2;
            atom = 1;
        }
        // bond multiplicity is invalid
        else if !(1..=3).contains(mult) {
            return Err(Error::BondMult {
                provided: *mult,
                bond_idx,
            });
        }

        // if any of the atom indices were invalid return the error
        if invalid_atom {
            return Err(Error::AtomIdx {
                provided,
                bond_idx,
                atom,
                max,
            });
        }
    }

    // check the atomic numbers
    for (atom_idx, &provided) in atoms.iter().enumerate() {
        if !(1..=118).contains(&provided) {
            return Err(Error::AtomicNum { provided, atom_idx });
        }
    }

    // SAFETY: we've checked all the pre-conditions, so the unchecked function is actually safe
    Ok(unsafe { gen_coords_unchecked(atoms, bonds) })
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{gen_coords_unchecked, get_coordinates};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct PubchemAtoms {
        element: Vec<u8>,
    }

    #[derive(Deserialize)]
    struct PubchemBonds {
        aid1: Vec<u16>,
        aid2: Vec<u16>,
        order: Vec<u16>,
    }

    #[derive(Deserialize)]
    struct PubchemRecord {
        atoms: PubchemAtoms,
        bonds: PubchemBonds,
    }

    #[allow(non_snake_case)]
    #[derive(Deserialize)]
    struct PubchemResponse {
        PC_Compounds: Vec<PubchemRecord>,
    }

    #[test]
    fn raw_ffi() {
        // this is the example form coordgenlibs/example_dir/example.cpp
        let atoms = vec![7u8, 6];
        let atoms: *const u8 = atoms.as_ptr();
        let bonds = vec![0u16, 1, 1];
        let bonds: *const u16 = bonds.as_ptr();

        let mut coords = Vec::with_capacity(4);
        let coords_ptr = coords.as_mut_ptr();
        unsafe {
            get_coordinates(2usize, atoms, 1usize, bonds, coords_ptr);
            coords.set_len(4);
        };

        assert_eq!(coords, vec![-50.0, 0.0, 0.0, 0.0]);
    }

    fn pubchem_consistency(cid: usize) {
        let compound = reqwest::blocking::get(format!(
            "http://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/{}/JSON",
            cid
        ))
        .unwrap()
        .json::<PubchemResponse>()
        .unwrap();
        let atoms = &compound.PC_Compounds[0].atoms.element;
        let bonds = compound.PC_Compounds[0]
            .bonds
            .aid1
            .iter()
            .zip(compound.PC_Compounds[0].bonds.aid2.iter())
            .zip(compound.PC_Compounds[0].bonds.order.iter())
            .map(|((atom1, atom2), order)| [*atom1 - 1, *atom2 - 1, *order])
            .collect::<Vec<[u16; 3]>>();
        let mut c_coords: Vec<f32> = Vec::with_capacity(2 * atoms.len());
        let flat_bonds = bonds.clone().into_iter().flatten().collect::<Vec<u16>>();
        let rust_coords = unsafe { gen_coords_unchecked(&atoms, &bonds) };
        unsafe {
            get_coordinates(
                atoms.len(),
                atoms.as_ptr(),
                bonds.len(),
                flat_bonds.as_ptr(),
                c_coords.as_mut_ptr(),
            );
            c_coords.set_len(2 * atoms.len());
        }

        for idx in 0..atoms.len() {
            assert_eq!(c_coords[2 * idx], rust_coords[idx].0);
            assert_eq!(c_coords[2 * idx + 1], rust_coords[idx].1);
        }

        // don't get blocked by pubchem
        std::thread::sleep(Duration::from_secs(1));
    }

    // test consistency between FFI wrapper and the rust wrapper
    #[test]
    fn consistency() {
        // aspirin
        pubchem_consistency(2244);
        // arsenic acid
        pubchem_consistency(234);
        // 2-Amino-1-phenylethanol
        pubchem_consistency(1000);
        // N-[[5-(2,2-Dimethyl-1,3-dioxolan-4-yl)-2,2-dimethyl-1,3-dioxolan-4-yl]methyl]-3,4-dimethylaniline
        pubchem_consistency(234578);
        // 3-(2-Bromophenoxy)-4-oxo-4H-chromen-7-yl 3,4,5-trimethoxybenzoate
        pubchem_consistency(992342);
    }
}
