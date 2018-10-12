use cpu::Cpu;
use ram::Ram;

pub type OpcodeFunction = fn(&mut Cpu, &mut Ram);
pub static OPCODES: [OpcodeFunction; 512] = [
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
    Cpu::ld_a_hli,
    Cpu::dec_hl,
    Cpu::inc_l,
    Cpu::dec_l,
    Cpu::ld_l_n,
    Cpu::cpl,
    Cpu::jr_nc_n,
    Cpu::ld_sp_nn,
    Cpu::ld_hld_a,
    Cpu::inc_sp,
    Cpu::inc_deref_hl,
    Cpu::dec_deref_hl,
    Cpu::ld_deref_hl_n,
    Cpu::scf,
    Cpu::jr_c_n,
    Cpu::add_hl_sp,
    Cpu::ld_a_hld,
    Cpu::dec_sp,
    Cpu::inc_a,
    Cpu::dec_a,
    Cpu::ld_a_n,
    Cpu::ccf,
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
    Cpu::halt,
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
    Cpu::adc_a_b,
    Cpu::adc_a_c,
    Cpu::adc_a_d,
    Cpu::adc_a_e,
    Cpu::adc_a_h,
    Cpu::adc_a_l,
    Cpu::adc_a_deref_hl,
    Cpu::adc_a_a,
    Cpu::sub_a_b,
    Cpu::sub_a_c,
    Cpu::sub_a_d,
    Cpu::sub_a_e,
    Cpu::sub_a_h,
    Cpu::sub_a_l,
    Cpu::sub_a_deref_hl,
    Cpu::sub_a_a,
    Cpu::sbc_a_b,
    Cpu::sbc_a_c,
    Cpu::sbc_a_d,
    Cpu::sbc_a_e,
    Cpu::sbc_a_h,
    Cpu::sbc_a_l,
    Cpu::sbc_a_deref_hl,
    Cpu::sbc_a_a,
    Cpu::and_a_b,
    Cpu::and_a_c,
    Cpu::and_a_d,
    Cpu::and_a_e,
    Cpu::and_a_h,
    Cpu::and_a_l,
    Cpu::and_a_deref_hl,
    Cpu::and_a_a,
    Cpu::xor_a_b,
    Cpu::xor_a_c,
    Cpu::xor_a_d,
    Cpu::xor_a_e,
    Cpu::xor_a_h,
    Cpu::xor_a_l,
    Cpu::xor_a_deref_hl,
    Cpu::xor_a_a,
    Cpu::or_a_b,
    Cpu::or_a_c,
    Cpu::or_a_d,
    Cpu::or_a_e,
    Cpu::or_a_h,
    Cpu::or_a_l,
    Cpu::or_a_deref_hl,
    Cpu::or_a_a,
    Cpu::cp_a_b,
    Cpu::cp_a_c,
    Cpu::cp_a_d,
    Cpu::cp_a_e,
    Cpu::cp_a_h,
    Cpu::cp_a_l,
    Cpu::cp_a_deref_hl,
    Cpu::cp_a_a,
    Cpu::ret_nz,
    Cpu::pop_bc,
    Cpu::jp_nz,
    Cpu::jp,
    Cpu::call_nz,
    Cpu::push_bc,
    Cpu::add_a_n,
    Cpu::rst_00h,
    Cpu::ret_z,
    Cpu::ret,
    Cpu::jp_z,
    Cpu::na, // Prefix CB, implemented in Cpu::step()
    Cpu::call_z,
    Cpu::call,
    Cpu::adc_a_n,
    Cpu::rst_08h,
    Cpu::ret_nc,
    Cpu::pop_de,
    Cpu::jp_nc,
    Cpu::na,
    Cpu::call_nc,
    Cpu::push_de,
    Cpu::sub_a_n,
    Cpu::rst_10h,
    Cpu::ret_c,
    Cpu::reti,
    Cpu::jp_c,
    Cpu::na,
    Cpu::call_c,
    Cpu::na,
    Cpu::sbc_a_n,
    Cpu::rst_18h,
    Cpu::ldh_deref_n_a,
    Cpu::pop_hl,
    Cpu::ld_addr_c_a,
    Cpu::na,
    Cpu::na,
    Cpu::push_hl,
    Cpu::and_a_n,
    Cpu::rst_20h,
    Cpu::add_sp_n,
    Cpu::jp_hl,
    Cpu::ld_nn_a,
    Cpu::na,
    Cpu::na,
    Cpu::na,
    Cpu::xor_a_n,
    Cpu::rst_28h,
    Cpu::ldh_a_deref_n,
    Cpu::pop_af,
    Cpu::ld_a_addr_c,
    Cpu::di,
    Cpu::na,
    Cpu::push_af,
    Cpu::or_a_n,
    Cpu::rst_30h,
    Cpu::ld_hl_sp_plus_n,
    Cpu::ld_sp_hl,
    Cpu::ld_a_nn,
    Cpu::ei,
    Cpu::na,
    Cpu::na,
    Cpu::cp_a_n,
    Cpu::rst_38h,
    Cpu::rlc_b,
    Cpu::rlc_c,
    Cpu::rlc_d,
    Cpu::rlc_e,
    Cpu::rlc_h,
    Cpu::rlc_l,
    Cpu::rlc_deref_hl,
    Cpu::rlc_a,
    Cpu::rrc_b,
    Cpu::rrc_c,
    Cpu::rrc_d,
    Cpu::rrc_e,
    Cpu::rrc_h,
    Cpu::rrc_l,
    Cpu::rrc_deref_hl,
    Cpu::rrc_a,
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
];
