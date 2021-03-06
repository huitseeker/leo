// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::TestTypeInference;

#[test]
fn test_empty_array() {
    let program_string = include_str!("empty_array.leo");

    let check = TestTypeInference::new(program_string);

    check.expect_error();
}

#[test]
fn test_invalid_array_access() {
    let program_string = include_str!("invalid_array_access.leo");

    let check = TestTypeInference::new(program_string);

    check.expect_error();
}

#[test]
fn test_invalid_spread() {
    let program_string = include_str!("invalid_spread.leo");

    let check = TestTypeInference::new(program_string);

    check.expect_error();
}

#[test]
fn test_index_implicit() {
    let program_string = include_str!("index_implicit.leo");

    let check = TestTypeInference::new(program_string);

    check.check()
}

#[test]
fn test_slice_implicit() {
    let program_string = include_str!("slice_implicit.leo");

    let check = TestTypeInference::new(program_string);

    check.check();
}
