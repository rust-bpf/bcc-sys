pub const BPF_REG_0: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_0;
pub const BPF_REG_1: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_1;
pub const BPF_REG_2: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_2;
pub const BPF_REG_3: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_3;
pub const BPF_REG_4: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_4;
pub const BPF_REG_5: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_5;
pub const BPF_REG_6: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_6;
pub const BPF_REG_7: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_7;
pub const BPF_REG_8: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_8;
pub const BPF_REG_9: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_9;
pub const BPF_REG_10: _bindgen_ty_1 = _bindgen_ty_1::BPF_REG_10;
pub const __MAX_BPF_REG: _bindgen_ty_1 = _bindgen_ty_1::__MAX_BPF_REG;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_1 {
    BPF_REG_0 = 0,
    BPF_REG_1 = 1,
    BPF_REG_2 = 2,
    BPF_REG_3 = 3,
    BPF_REG_4 = 4,
    BPF_REG_5 = 5,
    BPF_REG_6 = 6,
    BPF_REG_7 = 7,
    BPF_REG_8 = 8,
    BPF_REG_9 = 9,
    BPF_REG_10 = 10,
    __MAX_BPF_REG = 11,
}
pub const BPF_SOCK_OPS_VOID: _bindgen_ty_2 = _bindgen_ty_2::BPF_SOCK_OPS_VOID;
pub const BPF_SOCK_OPS_TIMEOUT_INIT: _bindgen_ty_2 =
    _bindgen_ty_2::BPF_SOCK_OPS_TIMEOUT_INIT;
pub const BPF_SOCK_OPS_RWND_INIT: _bindgen_ty_2 =
    _bindgen_ty_2::BPF_SOCK_OPS_RWND_INIT;
pub const BPF_SOCK_OPS_TCP_CONNECT_CB: _bindgen_ty_2 =
    _bindgen_ty_2::BPF_SOCK_OPS_TCP_CONNECT_CB;
pub const BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: _bindgen_ty_2 =
    _bindgen_ty_2::BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB;
pub const BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: _bindgen_ty_2 =
    _bindgen_ty_2::BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB;
pub const BPF_SOCK_OPS_NEEDS_ECN: _bindgen_ty_2 =
    _bindgen_ty_2::BPF_SOCK_OPS_NEEDS_ECN;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_2 {
    BPF_SOCK_OPS_VOID = 0,
    BPF_SOCK_OPS_TIMEOUT_INIT = 1,
    BPF_SOCK_OPS_RWND_INIT = 2,
    BPF_SOCK_OPS_TCP_CONNECT_CB = 3,
    BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB = 4,
    BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB = 5,
    BPF_SOCK_OPS_NEEDS_ECN = 6,
}
