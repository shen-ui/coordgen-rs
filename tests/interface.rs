use coordgen::*;

#[test]
fn atomic_num_errors() {
    // 0 isn't a valid atomic num
    assert_eq!(
        gen_coords(&[0, 1], &[[0, 1, 1]]),
        Err(Error::AtomicNum {
            provided: 0,
            atom_idx: 0
        })
    );
    // neither is 200
    assert_eq!(
        gen_coords(&[200, 100], &[[0, 1, 1]]),
        Err(Error::AtomicNum {
            provided: 200,
            atom_idx: 0
        })
    );
    // make sure it catches things at a higher index
    assert_eq!(
        gen_coords(&[1, 1, 200], &[[0, 1, 1]]),
        Err(Error::AtomicNum {
            provided: 200,
            atom_idx: 2
        })
    );
    // make sure it errors on the first invalid thing found
    assert_eq!(
        gen_coords(&[1, 150, 200], &[[0, 1, 1]]),
        Err(Error::AtomicNum {
            provided: 150,
            atom_idx: 1
        })
    );
}

#[test]
fn atomic_idx_errors() {
    assert_eq!(
        gen_coords(&[1u8], &[[10u16, 0, 1]]),
        Err(Error::AtomIdx {
            provided: 10,
            max: 1,
            atom: 0,
            bond_idx: 0
        })
    );
    assert_eq!(
        gen_coords(&[1u8], &[[0u16, 10, 1]]),
        Err(Error::AtomIdx {
            provided: 10,
            max: 1,
            atom: 1,
            bond_idx: 0
        })
    );
    assert_eq!(
        gen_coords(&[1u8], &[[1u16, 0, 1]]),
        Err(Error::AtomIdx {
            provided: 1,
            max: 1,
            atom: 0,
            bond_idx: 0
        })
    );
    assert_eq!(
        gen_coords(&[1u8], &[[0u16, 1, 1]]),
        Err(Error::AtomIdx {
            provided: 1,
            max: 1,
            atom: 1,
            bond_idx: 0
        })
    );

    assert_eq!(
        gen_coords(&[1u8, 2], &[[1, 0, 1], [3, 0, 1]]),
        Err(Error::AtomIdx {
            provided: 3,
            max: 2,
            atom: 0,
            bond_idx: 1
        })
    );
    assert_eq!(
        gen_coords(&[1u8, 2], &[[1, 0, 1], [0, 3, 1]]),
        Err(Error::AtomIdx {
            provided: 3,
            max: 2,
            atom: 1,
            bond_idx: 1
        })
    );
    // make sure error from first invalid one found
    assert_eq!(
        gen_coords(&[1u8, 2], &[[1, 3, 1], [0, 3, 1]]),
        Err(Error::AtomIdx {
            provided: 3,
            max: 2,
            atom: 1,
            bond_idx: 0
        })
    );
}

#[test]
fn bond_mult_errors() {
    assert_eq!(
        gen_coords(&[1u8, 1], &[[1, 0, 0]]),
        Err(Error::BondMult {
            provided: 0,
            bond_idx: 0
        })
    );
    assert_eq!(
        gen_coords(&[1u8, 1, 2], &[[0, 1, 1], [1, 2, 0]]),
        Err(Error::BondMult {
            provided: 0,
            bond_idx: 1
        })
    );
    // make sure error from first invalid bond found
    assert_eq!(
        gen_coords(&[1u8, 1, 2], &[[0, 1, 4], [1, 2, 0]]),
        Err(Error::BondMult {
            provided: 4,
            bond_idx: 0
        })
    );
}

#[test]
fn parallel_bond_errors() {
    // simple cases
    // make sure this doesn't depend on the order of atomic indices in bonds
    // also make sure the bond multiplicity doesn't affect things
    assert_eq!(
        gen_coords(&[1u8, 1], &[[1, 0, 1], [1, 0, 1]]),
        Err(Error::ParallelBonds { bond1: 0, bond2: 1 })
    );
    assert_eq!(
        gen_coords(&[1u8, 1], &[[1, 0, 1], [1, 0, 2]]),
        Err(Error::ParallelBonds { bond1: 0, bond2: 1 })
    );
    assert_eq!(
        gen_coords(&[1u8, 1], &[[1, 0, 1], [0, 1, 1]]),
        Err(Error::ParallelBonds { bond1: 0, bond2: 1 })
    );
    assert_eq!(
        gen_coords(&[1u8, 1], &[[1, 0, 1], [0, 1, 2]]),
        Err(Error::ParallelBonds { bond1: 0, bond2: 1 })
    );
    assert_eq!(
        gen_coords(&[1u8, 1], &[[0, 1, 1], [1, 0, 1]]),
        Err(Error::ParallelBonds { bond1: 0, bond2: 1 })
    );
    assert_eq!(
        gen_coords(&[1u8, 1], &[[0, 1, 1], [1, 0, 2]]),
        Err(Error::ParallelBonds { bond1: 0, bond2: 1 })
    );

    // make sure indices are correct
    assert_eq!(
        gen_coords(
            &[1u8, 1, 1, 1],
            &[[0, 1, 1], [1, 2, 2], [2, 3, 1], [1, 0, 2]]
        ),
        Err(Error::ParallelBonds { bond1: 0, bond2: 3 })
    );
    assert_eq!(
        gen_coords(
            &[1u8, 1, 1, 1],
            &[[0, 1, 1], [1, 2, 2], [2, 3, 1], [2, 1, 2]]
        ),
        Err(Error::ParallelBonds { bond1: 1, bond2: 3 })
    );
}

#[test]
fn error_order() {
    // errors should happen in this order:
    // 1. parallel bonds as soon as one is found
    // 2. atom index, preferring atom 0 over 1
    // 3. bond multiplicity
    // 4. atomic number
    let mut atoms = [1u8, 1, 1, 1];
    let mut bonds = [[0u16, 1, 1], [1, 2, 2], [2, 3, 3]];

    assert!(gen_coords(&atoms, &bonds).is_ok());

    // bond 0 and 1 are parallel
    bonds[1][1] = 0;
    // bond 2 has two invalid atom indices and invalid bond mult
    bonds[2][0] = 100;
    bonds[2][1] = 150;
    bonds[2][2] = 500;
    // atom 1 has a invalid atomic number
    atoms[1] = 150;

    // first parallel bond errors
    assert_eq!(
        gen_coords(&atoms, &bonds),
        Err(Error::ParallelBonds { bond1: 0, bond2: 1 })
    );

    // if we fix it then first atomic index one errors
    bonds[1][1] = 2;
    assert_eq!(
        gen_coords(&atoms, &bonds),
        Err(Error::AtomIdx {
            provided: 100,
            atom: 0,
            bond_idx: 2,
            max: 4
        })
    );
    // if we fix it then second atomic index one errors
    bonds[2][0] = 2;
    assert_eq!(
        gen_coords(&atoms, &bonds),
        Err(Error::AtomIdx {
            provided: 150,
            atom: 1,
            bond_idx: 2,
            max: 4
        })
    );
    // if we fix it then bond multiplicity fails
    bonds[2][1] = 3;
    assert_eq!(
        gen_coords(&atoms, &bonds),
        Err(Error::BondMult {
            provided: 500,
            bond_idx: 2
        })
    );
    // if when we fix that the atomic number fails
    bonds[2][2] = 2;
    assert_eq!(
        gen_coords(&atoms, &bonds),
        Err(Error::AtomicNum {
            provided: 150,
            atom_idx: 1
        })
    );
    // finally we go back to being ok
    atoms[1] = 10;
    assert!(gen_coords(&atoms, &bonds).is_ok());
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn unchecked_panics_atomic_num() {
    unsafe {
        gen_coords_unchecked(&[0, 1], &[[0, 1, 1]]);
    }
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn unchecked_panics_atomic_idx() {
    unsafe {
        gen_coords_unchecked(&[1u8], &[[10u16, 0, 1]]);
    }
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn unchecked_panics_bond_mult() {
    unsafe {
        gen_coords_unchecked(&[1u8, 1], &[[1, 0, 0]]);
    }
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn unchecked_panics_parallel_bond() {
    unsafe {
        gen_coords_unchecked(&[1u8, 1], &[[1, 0, 1], [1, 0, 1]]);
    }
}
