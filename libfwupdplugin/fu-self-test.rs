// Copyright (C) 2023 Richard Hughes <richard@hughsie.com>
// SPDX-License-Identifier: LGPL-2.1+

#[repr(u8)]
enum FuSelfTestRevision {
    None = 0x0,
    All	= 0xF_F,
}

#[derive(New, Validate, Parse, ToString)]
struct FuStructSelfTest {
    signature: u32be == 0x1234_5678,
    length: u32le = $struct_size, // bytes
    revision: FuSelfTestRevision,
    owner: Guid,
    oem_id: [char; 6] == "ABCDEF",
    oem_table_id: [char; 8],
    oem_revision: u32le,
    asl_compiler_id: [u8; 4] =	0xDF,
    asl_compiler_revision: u32le,
}

#[derive(New, Validate, Parse, ToString)]
struct FuStructSelfTestWrapped {
    less: u8,
    base: FuStructSelfTest,
    more: u8,
}
