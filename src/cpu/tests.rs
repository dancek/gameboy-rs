#[cfg(test)]
mod test {
    use ::cpu::*;
    use ::ram::Ram;

    fn init(memory: Option<&[u8]>) -> (Cpu, Ram) {
        let cpu = Cpu::new();
        let mut ram = Ram::new();
        if memory.is_some() {
            ram.set_bytes(memory.unwrap());
        }
        (cpu, ram)
    }
    fn cycles<F>(cycles: usize, closure: F)
        where F: Fn(&mut Cpu, &mut Ram)
    {
        let (mut cpu, mut ram) = init(None);
        test(&mut cpu, &mut ram, cycles, closure);
    }
    fn test<F>(cpu: &mut Cpu, ram: &mut Ram, cycles: usize, closure: F)
        where F: Fn(&mut Cpu, &mut Ram)
    {
        let prev_cycles = cpu.cycles;
        closure(cpu, ram);
        assert!(cpu.cycles == prev_cycles + cycles,
            format!("Expected cpu cycles to be {}, got {}", prev_cycles + cycles, cpu.cycles));
    }
    fn opcode(opcode: usize) -> opcodes::OpcodeFunction {
        use ::cpu::opcodes::{OPCODES, OpcodeFunction};
        let func = OPCODES[opcode];
        if func as *const OpcodeFunction as usize == Cpu::nyi as *const OpcodeFunction as usize {
            panic!(format!("Unimplemented opcode: 0x{:X}", opcode));
        }
        func
    }

    #[test]
    fn test_nop() {
        cycles(8, Cpu::nop);
    }

    #[test]
    fn test_add_a_r() {
        macro_rules! test_add_a_r {
            ($r:ident, $func:expr) => {{
                let (mut cpu, mut ram) = init(None);
                cpu.a = 0x10;
                cpu.$r = 0x1;
                let expected = cpu.a.wrapping_add(cpu.$r);
                test(&mut cpu, &mut ram, 4, $func);
                assert!(cpu.a == expected, format!("add a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.a));
                assert!(!cpu.f.z());
                assert!(!cpu.f.n());
                assert!(!cpu.f.h());
                assert!(!cpu.f.c());

                let (mut cpu, mut ram) = init(None);
                cpu.a = 0x10;
                cpu.$r = 0x10;
                let expected = cpu.a.wrapping_add(cpu.$r);
                test(&mut cpu, &mut ram, 4, $func);
                assert!(cpu.a == expected, format!("add a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.a));
                assert!(!cpu.f.z());
                assert!(!cpu.f.n());
                assert!(cpu.f.h());
                assert!(!cpu.f.c());

                let (mut cpu, mut ram) = init(None);
                cpu.a = 0xF0;
                cpu.$r = 0x11;
                let expected = cpu.a.wrapping_add(cpu.$r);
                test(&mut cpu, &mut ram, 4, $func);
                assert!(cpu.a == expected, format!("add a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.a));
                assert!(!cpu.f.z());
                assert!(!cpu.f.n());
                assert!(cpu.f.h());
                assert!(cpu.f.c());
            }}
        }
        fn test_add_a_a() {
            let (mut cpu, mut ram) = init(None);
            cpu.a = 0x4;
            let expected = cpu.a.wrapping_add(cpu.a);
            test(&mut cpu, &mut ram, 4, opcode(0x87));
            assert!(cpu.a == expected, format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            let (mut cpu, mut ram) = init(None);
            cpu.a = 0x8;
            let expected = cpu.a.wrapping_add(cpu.a);
            test(&mut cpu, &mut ram, 4, opcode(0x87));
            assert!(cpu.a == expected, format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());

            let (mut cpu, mut ram) = init(None);
            cpu.a = 0xF0;
            let expected = cpu.a.wrapping_add(cpu.a);
            test(&mut cpu, &mut ram, 4, opcode(0x87));
            assert!(cpu.a == expected, format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(cpu.f.c());
        }

        test_add_a_a();
        test_add_a_r!(b, opcode(0x80));
        test_add_a_r!(c, opcode(0x81));
        test_add_a_r!(d, opcode(0x82));
        test_add_a_r!(e, opcode(0x83));
        test_add_a_r!(h, opcode(0x84));
        test_add_a_r!(l, opcode(0x85));
    }

    #[test]
    fn test_dec_rr() {
        macro_rules! test_dec_rr(
            ($r1:ident, $r2:ident, $func:expr) => {{
                let (mut cpu, mut ram) = init(None);
                cpu.$r1 = 0x0;
                cpu.$r2 = 0x0;
                test(&mut cpu, &mut ram, 8, $func);
                assert!(cpu.$r1 == 0xFF, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0xFF, cpu.$r1));
                assert!(cpu.$r2 == 0xFF, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0xFF, cpu.$r2));

                cpu.$r1 = 0xFF;
                cpu.$r2 = 0x1;
                test(&mut cpu, &mut ram, 8, $func);
                assert!(cpu.$r1 == 0xFF, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0xFF, cpu.$r1));
                assert!(cpu.$r2 == 0x0, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x0, cpu.$r2));

                cpu.$r1 = 0x1;
                cpu.$r2 = 0x6;
                test(&mut cpu, &mut ram, 8, $func);
                assert!(cpu.$r1 == 0x1, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x1, cpu.$r1));
                assert!(cpu.$r2 == 0x5, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x5, cpu.$r2));
            }}
        );
        test_dec_rr!(b, c, opcode(0x0B));
        test_dec_rr!(d, e, opcode(0x1B));
        test_dec_rr!(h, l, opcode(0x2B));
    }

    #[test]
    fn test_inc_rr() {
        macro_rules! test_inc_rr(
            ($r1:ident, $r2:ident, $func:expr) => {{
                let (mut cpu, mut ram) = init(None);
                cpu.$r1 = 0x0;
                cpu.$r2 = 0x0;
                test(&mut cpu, &mut ram, 8, $func);
                assert!(cpu.$r1 == 0x00, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x00, cpu.$r1));
                assert!(cpu.$r2 == 0x01, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x01, cpu.$r2));

                cpu.$r1 = 0x2;
                cpu.$r2 = 0xFF;
                test(&mut cpu, &mut ram, 8, $func);
                assert!(cpu.$r1 == 0x3, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x3, cpu.$r1));
                assert!(cpu.$r2 == 0x0, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x0, cpu.$r2));

                cpu.$r1 = 0xFF;
                cpu.$r2 = 0xFF;
                test(&mut cpu, &mut ram, 8, $func);
                assert!(cpu.$r1 == 0x0, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x0, cpu.$r1));
                assert!(cpu.$r2 == 0x0, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x0, cpu.$r2));
            }}
        );
        test_inc_rr!(b, c, opcode(0x03));
        test_inc_rr!(d, e, opcode(0x13));
        test_inc_rr!(h, l, opcode(0x23));
    }

    #[test]
    fn test_dec_sp() {
        let (mut cpu, mut ram) = init(None);
        cpu.sp = 1;
        test(&mut cpu, &mut ram, 8, opcode(0x3B));
        assert!(cpu.sp == 0x0);
        test(&mut cpu, &mut ram, 8, opcode(0x3B));
        assert!(cpu.sp == 0xFFFF);
        test(&mut cpu, &mut ram, 8, opcode(0x3B));
        assert!(cpu.sp == 0xFFFE);
    }

    #[test]
    fn test_inc_sp() {
        let (mut cpu, mut ram) = init(None);
        cpu.sp = 0xFFFE;
        test(&mut cpu, &mut ram, 8, opcode(0x33));
        assert!(cpu.sp == 0xFFFF);
        test(&mut cpu, &mut ram, 8, opcode(0x33));
        assert!(cpu.sp == 0x0);
        test(&mut cpu, &mut ram, 8, opcode(0x33));
        assert!(cpu.sp == 0x1);
    }

    #[test]
    fn test_ld_rr_nn() {
        macro_rules! test_ld_rr_nn(
            ($reg1:ident, $reg2:ident, $func: expr) => {{
                let (mut cpu, mut ram) = init(Some(&[0,0,1,2]));
                cpu.pc = 2;
                test(&mut cpu, &mut ram, 12, $func);
                assert!(cpu.$reg1 == ram[3], format!("Expected {}, got {}", ram[1], cpu.$reg1));
                assert!(cpu.$reg2 == ram[2], format!("Expected {}, got {}", ram[0], cpu.$reg2));
                assert!(cpu.pc == 4, format!("Expected pc=4, got pc={}", cpu.pc));
            }}
        );

        test_ld_rr_nn!(b, c, opcode(0x1));
        test_ld_rr_nn!(d, e, opcode(0x11));
        test_ld_rr_nn!(h, l, opcode(0x21));

        let (mut cpu, mut ram) = init(Some(&[0,0,1,2]));
        cpu.pc = 2;
        test(&mut cpu, &mut ram, 12, opcode(0x31));
        assert!(cpu.sp == 513);
        assert!(cpu.pc == 4, format!("Expected pc=4, got pc={}", cpu.pc));
    }

    #[test]
    fn test_ld_r1_r2() {
        macro_rules! test_ld_r1_r2(
            ($r1:ident, $r2:ident, $func:expr) => {{
                let (mut cpu, mut ram) = init(None);
                cpu.$r2 = 123;
                test(&mut cpu, &mut ram, 4, $func);
                assert!(cpu.$r1 == cpu.$r2,
                        format!("ld {}, {}: Expected {}, got {}", stringify!($r1), stringify!($r2), cpu.$r2, cpu.$r1));
            }}
        );
        macro_rules! test_ld_r_hl(
            ($r:ident, $func: expr) => {{
                let (mut cpu, mut ram) = init(None);
                cpu.h = 0x11;
                cpu.l = 0x22;
                ram[0x1122] = 0x33;
                test(&mut cpu, &mut ram, 8, $func);
                assert!(cpu.$r == 0x33, format!("ld {}, (hl): Expected {}, got {}", stringify!($r), 0x33, cpu.$r));
            }}
        );
        test_ld_r1_r2!(a, a, opcode(0x7F));
        test_ld_r1_r2!(a, b, opcode(0x78));
        test_ld_r1_r2!(a, c, opcode(0x79));
        test_ld_r1_r2!(a, d, opcode(0x7A));
        test_ld_r1_r2!(a, e, opcode(0x7B));
        test_ld_r1_r2!(a, h, opcode(0x7C));
        test_ld_r1_r2!(a, l, opcode(0x7D));
        test_ld_r_hl!(a, opcode(0x7E));

        test_ld_r1_r2!(b, a, opcode(0x47));
        test_ld_r1_r2!(b, b, opcode(0x40));
        test_ld_r1_r2!(b, c, opcode(0x41));
        test_ld_r1_r2!(b, d, opcode(0x42));
        test_ld_r1_r2!(b, e, opcode(0x43));
        test_ld_r1_r2!(b, h, opcode(0x44));
        test_ld_r1_r2!(b, l, opcode(0x45));
        test_ld_r_hl!(b, opcode(0x46));

        test_ld_r1_r2!(c, a, opcode(0x4F));
        test_ld_r1_r2!(c, b, opcode(0x48));
        test_ld_r1_r2!(c, c, opcode(0x49));
        test_ld_r1_r2!(c, d, opcode(0x4A));
        test_ld_r1_r2!(c, e, opcode(0x4B));
        test_ld_r1_r2!(c, h, opcode(0x4C));
        test_ld_r1_r2!(c, l, opcode(0x4D));
        test_ld_r_hl!(c, opcode(0x4E));

        test_ld_r1_r2!(d, a, opcode(0x57));
        test_ld_r1_r2!(d, b, opcode(0x50));
        test_ld_r1_r2!(d, c, opcode(0x51));
        test_ld_r1_r2!(d, d, opcode(0x52));
        test_ld_r1_r2!(d, e, opcode(0x53));
        test_ld_r1_r2!(d, h, opcode(0x54));
        test_ld_r1_r2!(d, l, opcode(0x55));
        test_ld_r_hl!(d, opcode(0x56));

        test_ld_r1_r2!(e, a, opcode(0x5F));
        test_ld_r1_r2!(e, b, opcode(0x58));
        test_ld_r1_r2!(e, c, opcode(0x59));
        test_ld_r1_r2!(e, d, opcode(0x5A));
        test_ld_r1_r2!(e, e, opcode(0x5B));
        test_ld_r1_r2!(e, h, opcode(0x5C));
        test_ld_r1_r2!(e, l, opcode(0x5D));
        test_ld_r_hl!(e, opcode(0x5E));

        test_ld_r1_r2!(h, a, opcode(0x67));
        test_ld_r1_r2!(h, b, opcode(0x60));
        test_ld_r1_r2!(h, c, opcode(0x61));
        test_ld_r1_r2!(h, d, opcode(0x62));
        test_ld_r1_r2!(h, e, opcode(0x63));
        test_ld_r1_r2!(h, h, opcode(0x64));
        test_ld_r1_r2!(h, l, opcode(0x65));
        test_ld_r_hl!(h, opcode(0x66));

        test_ld_r1_r2!(l, a, opcode(0x6F));
        test_ld_r1_r2!(l, b, opcode(0x68));
        test_ld_r1_r2!(l, c, opcode(0x69));
        test_ld_r1_r2!(l, d, opcode(0x6A));
        test_ld_r1_r2!(l, e, opcode(0x6B));
        test_ld_r1_r2!(l, h, opcode(0x6C));
        test_ld_r1_r2!(l, l, opcode(0x6D));
        test_ld_r_hl!(l, opcode(0x6E));
    }

    #[test]
    fn test_ld_rr_r() {
        macro_rules! test_ld_rr_r(
            ($rr_1:ident, $rr_2:ident, $rr:ident, $r:ident, $func: expr) => {{
                let (mut cpu, mut ram) = init(None);
                cpu.$r = 123;
                cpu.$rr_1 = 0x11;
                cpu.$rr_2 = 0x22;
                test(&mut cpu, &mut ram, 8, $func);
                assert!(cpu.$rr() == 0x1122);
                let value = ram[cpu.$rr() as usize];
                assert!(value == 123,
                        format!("ld ({}), {}: Expected {}, got {}", stringify!($rr), stringify!($r), 123, value));
            }}
        );
        test_ld_rr_r!(h, l, hl, a, opcode(0x77));
        test_ld_rr_r!(h, l, hl, b, opcode(0x70));
        test_ld_rr_r!(h, l, hl, c, opcode(0x71));
        test_ld_rr_r!(h, l, hl, d, opcode(0x72));
        test_ld_rr_r!(h, l, hl, e, opcode(0x73));

        // ld_hl_h
        let (mut cpu, mut ram) = init(None);
        cpu.h = 0x11;
        cpu.l = 0x22;
        test(&mut cpu, &mut ram, 8, opcode(0x74));
        let value = ram[cpu.hl() as usize];
        assert!(value == 0x11,
                format!("ld (hl), h: Expected {}, got {}", 0x11, value));

        // ld_hl_l
        let (mut cpu, mut ram) = init(None);
        cpu.h = 0x11;
        cpu.l = 0x22;
        test(&mut cpu, &mut ram, 8, opcode(0x75));
        let value = ram[cpu.hl() as usize];
        assert!(value == 0x22,
                format!("ld (hl), l: Expected {}, got {}", 0x22, value));

        // ld_hl_n
        let (mut cpu, mut ram) = init(Some(&[123]));
        cpu.h = 0x11;
        cpu.l = 0x22;
        test(&mut cpu, &mut ram, 12, opcode(0x36));
        let value = ram[cpu.hl() as usize];
        assert!(value == 123,
                format!("ld (hl), n: Expected {}, got {}", 123, value));

        test_ld_rr_r!(b, c, bc, a, opcode(0x02));
        test_ld_rr_r!(d, e, de, a, opcode(0x12));
    }

    #[test]
    fn test_ld_r_n() {
        macro_rules! test_ld_r_n {
            ($r:ident, $func:expr) => {{
                let (mut cpu, mut ram) = init(Some(&[0,0,123]));
                cpu.pc=2;
                test(&mut cpu, &mut ram, 8, $func);
                assert!(cpu.$r == 123,
                    format!("ld {}, n: Expected {}, got {}", stringify!($r), 123, cpu.$r));
            }}
        };
        test_ld_r_n!(a, opcode(0x3E));
        test_ld_r_n!(b, opcode(0x06));
        test_ld_r_n!(c, opcode(0x0E));
        test_ld_r_n!(d, opcode(0x16));
        test_ld_r_n!(e, opcode(0x1E));
        test_ld_r_n!(h, opcode(0x26));
        test_ld_r_n!(l, opcode(0x2E));
    }

    #[test]
    fn test_ld_nn_a() {
        let (mut cpu, mut ram) = init(Some(&[0x0, 0x0, 0x22, 0x11]));
        cpu.pc = 2;
        cpu.a = 123;
        test(&mut cpu, &mut ram, 16, opcode(0xEA));
        assert!(ram[0x1122] == 123,
            format!("ld (nn), a: Expected {}, got {}", 123, ram[0x1122]));
    }

    #[test]
    fn test_ld_a_bc() {
        let (mut cpu, mut ram) = init(None);
        ram[0x1122] = 123;
        cpu.b=0x11;
        cpu.c=0x22;
        test(&mut cpu, &mut ram, 8, opcode(0x0A));
        assert!(cpu.a == 123,
            format!("ld a, (bc): Expected {}, got {}", 123, cpu.a));
    }

    #[test]
    fn test_ld_a_de() {
        let (mut cpu, mut ram) = init(None);
        ram[0x1122] = 123;
        cpu.d=0x11;
        cpu.e=0x22;
        test(&mut cpu, &mut ram, 8, opcode(0x1A));
        assert!(cpu.a == 123,
            format!("ld a, (de): Expected {}, got {}", 123, cpu.a));
    }
    #[test]
    fn test_ld_a_nn() {
        let (mut cpu, mut ram) = init(Some(&[0,0,0x22,0x11]));
        cpu.pc = 2;
        ram[0x1122] = 123;
        test(&mut cpu, &mut ram, 16, opcode(0xFA));
        assert!(cpu.a == 123,
            format!("ld a, (nn): Expected 123, got {}", cpu.a));
    }
    #[test]
    fn test_ld_a_addr_c() {
        let (mut cpu, mut ram) = init(None);
        cpu.c = 0x05;
        ram[0xFF05] = 123;
        test(&mut cpu, &mut ram, 8, opcode(0xF2));
        assert!(cpu.a == 123,
            format!("ld a, (c): Expected 123, got {}", cpu.a));
    }
    #[test]
    fn test_ld_addr_c_a() {
        let (mut cpu, mut ram) = init(None);
        cpu.a = 123;
        cpu.c = 0x05;
        test(&mut cpu, &mut ram, 8, opcode(0xE2));
        assert!(ram[0xFF05] == 123,
            format!("ld (c), a: Expected 123, got {}", ram[0xFF05]));
    }
}