// Copyright (C) 2023 Richard Hughes <richard@hughsie.com>
// SPDX-License-Identifier: LGPL-2.1+

#[derive(Getters)]
struct FuStructEfiImageDataDirEntry {
    addr: u32le,
    size: u32le,
}
