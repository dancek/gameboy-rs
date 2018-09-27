use cpu::Cpu;
use ram::Ram;

pub type OpcodeFunction = fn(&mut Cpu, &mut Ram);
pub static OPCODES: &'static [OpcodeFunction] = &[
    Cpu::nop,
    Cpu::ld_bc_nn,
    Cpu::ld_bc_a,
    Cpu::inc_bc,
    Cpu::inc_b,
    Cpu::dec_b,
    Cpu::ld_b_n,
    Cpu::rlca,
    Cpu::ld_deref_a16_sp,
    Cpu::add_hl_bc,
    Cpu::ld_a_deref_bc,
    Cpu::dec_bc,
    Cpu::inc_c,
    Cpu::dec_c,
    Cpu::ld_c_n,
    Cpu::rrca,
    Cpu::stop,
    Cpu::ld_de_nn,
    Cpu::ld_de_a,
    Cpu::inc_de,
    Cpu::inc_d,
    Cpu::dec_d,
    Cpu::ld_d_n,
    Cpu::rla,
    Cpu::jr_n,
    Cpu::add_hl_de,
    Cpu::ld_a_deref_de,
    Cpu::dec_de,
    Cpu::inc_e,
    Cpu::dec_e,
    Cpu::ld_e_n,
    Cpu::rra,
    Cpu::jr_nz_n,
    Cpu::ld_hl_nn,
    Cpu::ld_hli_a,
    Cpu::inc_hl,
    Cpu::inc_h,
    Cpu::dec_h,
    Cpu::ld_h_n,
    Cpu::daa,
    Cpu::jr_z_n,
    Cpu::add_hl_hl,
    Cpu::nyi,
    Cpu::dec_hl,
    Cpu::inc_l,
    Cpu::dec_l,
    Cpu::ld_l_n,
    Cpu::nyi,
    Cpu::jr_nc_n,
    Cpu::ld_sp_nn,
    Cpu::ld_hld_a,
    Cpu::inc_sp,
    Cpu::inc_deref_hl,
    Cpu::dec_deref_hl,
    Cpu::ld_hl_n,
    Cpu::nyi,
    Cpu::jr_c_n,
    Cpu::add_hl_sp,
    Cpu::nyi,
    Cpu::dec_sp,
    Cpu::inc_a,
    Cpu::dec_a,
    Cpu::ld_a_n,
    Cpu::nyi,
    Cpu::ld_b_b,
    Cpu::ld_b_c,
    Cpu::ld_b_d,
    Cpu::ld_b_e,
    Cpu::ld_b_h,
    Cpu::ld_b_l,
    Cpu::ld_b_deref_hl,
    Cpu::ld_b_a,
    Cpu::ld_c_b,
    Cpu::ld_c_c,
    Cpu::ld_c_d,
    Cpu::ld_c_e,
    Cpu::ld_c_h,
    Cpu::ld_c_l,
    Cpu::ld_c_deref_hl,
    Cpu::ld_c_a,
    Cpu::ld_d_b,
    Cpu::ld_d_c,
    Cpu::ld_d_d,
    Cpu::ld_d_e,
    Cpu::ld_d_h,
    Cpu::ld_d_l,
    Cpu::ld_d_deref_hl,
    Cpu::ld_d_a,
    Cpu::ld_e_b,
    Cpu::ld_e_c,
    Cpu::ld_e_d,
    Cpu::ld_e_e,
    Cpu::ld_e_h,
    Cpu::ld_e_l,
    Cpu::ld_e_deref_hl,
    Cpu::ld_e_a,
    Cpu::ld_h_b,
    Cpu::ld_h_c,
    Cpu::ld_h_d,
    Cpu::ld_h_e,
    Cpu::ld_h_h,
    Cpu::ld_h_l,
    Cpu::ld_h_deref_hl,
    Cpu::ld_h_a,
    Cpu::ld_l_b,
    Cpu::ld_l_c,
    Cpu::ld_l_d,
    Cpu::ld_l_e,
    Cpu::ld_l_h,
    Cpu::ld_l_l,
    Cpu::ld_l_deref_hl,
    Cpu::ld_l_a,
    Cpu::ld_hl_b,
    Cpu::ld_hl_c,
    Cpu::ld_hl_d,
    Cpu::ld_hl_e,
    Cpu::ld_hl_h,
    Cpu::ld_hl_l,
    Cpu::nyi,
    Cpu::ld_hl_a,
    Cpu::ld_a_b,
    Cpu::ld_a_c,
    Cpu::ld_a_d,
    Cpu::ld_a_e,
    Cpu::ld_a_h,
    Cpu::ld_a_l,
    Cpu::ld_a_deref_hl,
    Cpu::ld_a_a,
    Cpu::add_a_b,
    Cpu::add_a_c,
    Cpu::add_a_d,
    Cpu::add_a_e,
    Cpu::add_a_h,
    Cpu::add_a_l,
    Cpu::add_a_deref_hl,
    Cpu::add_a_a,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::ld_addr_c_a,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::ld_nn_a,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::ld_a_addr_c,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::ld_a_nn,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
    Cpu::nyi,
];
