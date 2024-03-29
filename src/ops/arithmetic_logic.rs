use crate::cpu::{Cycles, Flags};
use crate::mmu::Mapper;
use crate::ops::{rr, set_rr};

pub fn add_a_r(a: &mut u8, r: u8, f: &mut Flags) -> Cycles {
    let res = *a as u16 + r as u16;

    f.set(Flags::Z, res == 0);
    f.remove(Flags::N);
    f.set(Flags::H, (((*a & 0xf) + (r & 0xf)) & 0x10) != 0);
    f.set(Flags::C, res > 0xff);

    *a = (res & 0xff) as u8;
    Cycles(4)
}

pub fn add_a_n(a: &mut u8, n: u8, f: &mut Flags) -> Cycles {
    let res = *a as u16 + n as u16;

    f.set(Flags::Z, res == 0);
    f.remove(Flags::N);
    f.set(Flags::H, (((*a & 0xf) + (n & 0xf)) & 0x10) != 0);
    f.set(Flags::C, res > 0xff);

    *a = (res & 0xff) as u8;
    Cycles(8)
}

pub fn add_a_hlptr(a: &mut u8, h: u8, l: u8, f: &mut Flags, mapper: &Mapper) -> Cycles {
    let val = mapper.read(rr(h, l));
    let res = *a as u16 + val as u16;

    f.set(Flags::Z, res == 0);
    f.remove(Flags::N);
    f.set(Flags::H, (((*a & 0xf) + (val & 0xf)) & 0x10) != 0);
    f.set(Flags::C, res > 0xff);

    *a = (res & 0xff) as u8;
    Cycles(8)
}

pub fn adc_a_r(a: &mut u8, r: u8, f: &mut Flags) -> Cycles {
    let carry = if f.contains(Flags::C) { 1 } else { 0 };
    let res = *a as u16 + r as u16 + carry;

    f.set(Flags::Z, res == 0);
    f.remove(Flags::N);
    f.set(
        Flags::H,
        (((*a & 0xf) + (r & 0xf) + carry as u8) & 0x10) != 0,
    );
    f.set(Flags::C, res > 0xff);

    *a = (res & 0xff) as u8;
    Cycles(4)
}

pub fn adc_a_n(a: &mut u8, n: u8, f: &mut Flags) -> Cycles {
    let carry = if f.contains(Flags::C) { 1 } else { 0 };
    let res = *a as u16 + n as u16 + carry;

    f.set(Flags::Z, res == 0);
    f.remove(Flags::N);
    f.set(
        Flags::H,
        (((*a & 0xf) + (n & 0xf) + carry as u8) & 0x10) != 0,
    );
    f.set(Flags::C, res > 0xff);

    *a = (res & 0xff) as u8;
    Cycles(8)
}

pub fn adc_a_hlptr(a: &mut u8, h: u8, l: u8, f: &mut Flags, mapper: &Mapper) -> Cycles {
    let val = mapper.read(rr(h, l));
    let carry = if f.contains(Flags::C) { 1 } else { 0 };
    let res = *a as u16 + val as u16 + carry;

    f.set(Flags::Z, res == 0);
    f.remove(Flags::N);
    f.set(
        Flags::H,
        (((*a & 0xf) + (val & 0xf) + carry as u8) & 0x10) != 0,
    );
    f.set(Flags::C, res > 0xff);

    *a = (res & 0xff) as u8;
    Cycles(8)
}

pub fn sub_r(a: &mut u8, r: u8, f: &mut Flags) -> Cycles {
    let res = *a as i16 - r as i16;

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(Flags::H, (*a & 0xf) < (r & 0xf));
    f.set(Flags::C, res < 0);

    *a = (res & 0xff) as u8;
    Cycles(4)
}

pub fn sub_n(a: &mut u8, n: u8, f: &mut Flags) -> Cycles {
    let res = *a as i16 - n as i16;

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(Flags::H, (*a & 0xf) < (n & 0xf));
    f.set(Flags::C, res < 0);

    *a = (res & 0xff) as u8;
    Cycles(8)
}

pub fn sub_hlptr(a: &mut u8, h: u8, l: u8, f: &mut Flags, mapper: &Mapper) -> Cycles {
    let val = mapper.read(rr(h, l));
    let res = *a as i16 - val as i16;

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(Flags::H, (*a & 0xf) < (val & 0xf));
    f.set(Flags::C, res < 0);

    *a = (res & 0xff) as u8;
    Cycles(8)
}

pub fn sbc_a_r(a: &mut u8, r: u8, f: &mut Flags) -> Cycles {
    let carry = if f.contains(Flags::C) { 1 } else { 0 };
    let res = *a as i16 - r as i16 - carry;

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(
        Flags::H,
        ((*a & 0xf) as u16) < (((r & 0xf) as u16) + carry as u16),
    );
    f.set(Flags::C, res < 0);

    *a = (res & 0xff) as u8;
    Cycles(4)
}

pub fn sbc_a_n(a: &mut u8, n: u8, f: &mut Flags) -> Cycles {
    let carry = if f.contains(Flags::C) { 1 } else { 0 };
    let res = *a as i16 - n as i16 - carry;

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(
        Flags::H,
        ((*a & 0xf) as u16) < (((n & 0xf) as u16) + carry as u16),
    );
    f.set(Flags::C, res < 0);

    *a = (res & 0xff) as u8;
    Cycles(8)
}

pub fn sbc_a_hlptr(a: &mut u8, h: u8, l: u8, f: &mut Flags, mapper: &Mapper) -> Cycles {
    let carry = if f.contains(Flags::C) { 1 } else { 0 };
    let val = mapper.read(rr(h, l));
    let res = *a as i16 - val as i16 - carry;

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(
        Flags::H,
        ((*a & 0xf) as u16) < (((val & 0xf) as u16) + carry as u16),
    );
    f.set(Flags::C, res < 0);

    *a = (res & 0xff) as u8;
    Cycles(8)
}

pub fn and_r(a: &mut u8, r: u8, f: &mut Flags) -> Cycles {
    *a = *a & r;

    f.set(Flags::Z, *a == 0);
    f.remove(Flags::N);
    f.insert(Flags::H);
    f.remove(Flags::C);

    Cycles(4)
}

pub fn and_n(a: &mut u8, n: u8, f: &mut Flags) -> Cycles {
    *a = *a & n;

    f.set(Flags::Z, *a == 0);
    f.remove(Flags::N);
    f.insert(Flags::H);
    f.remove(Flags::C);

    Cycles(8)
}

pub fn and_hlptr(a: &mut u8, h: u8, l: u8, f: &mut Flags, mapper: &Mapper) -> Cycles {
    let val = mapper.read(rr(h, l));
    *a = *a & val;

    f.set(Flags::Z, *a == 0);
    f.remove(Flags::N);
    f.insert(Flags::H);
    f.remove(Flags::C);

    Cycles(8)
}

pub fn xor_r(a: &mut u8, r: u8, f: &mut Flags) -> Cycles {
    *a = *a ^ r;

    f.set(Flags::Z, *a == 0);
    f.remove(Flags::N);
    f.remove(Flags::H);
    f.remove(Flags::C);

    Cycles(4)
}

pub fn xor_n(a: &mut u8, n: u8, f: &mut Flags) -> Cycles {
    *a = *a ^ n;

    f.set(Flags::Z, *a == 0);
    f.remove(Flags::N);
    f.remove(Flags::H);
    f.remove(Flags::C);

    Cycles(8)
}

pub fn xor_hlptr(a: &mut u8, h: u8, l: u8, f: &mut Flags, mapper: &Mapper) -> Cycles {
    let val = mapper.read(rr(h, l));
    *a = *a ^ val;

    f.set(Flags::Z, *a == 0);
    f.remove(Flags::N);
    f.remove(Flags::H);
    f.remove(Flags::C);

    Cycles(8)
}

pub fn or_r(a: &mut u8, r: u8, f: &mut Flags) -> Cycles {
    *a = *a | r;

    f.set(Flags::Z, *a == 0);
    f.remove(Flags::N);
    f.remove(Flags::H);
    f.remove(Flags::C);

    Cycles(4)
}

pub fn or_n(a: &mut u8, n: u8, f: &mut Flags) -> Cycles {
    *a = *a | n;

    f.set(Flags::Z, *a == 0);
    f.remove(Flags::N);
    f.remove(Flags::H);
    f.remove(Flags::C);

    Cycles(8)
}

pub fn or_hlptr(a: &mut u8, h: u8, l: u8, f: &mut Flags, mapper: &Mapper) -> Cycles {
    let val = mapper.read(rr(h, l));
    *a = *a | val;

    f.set(Flags::Z, *a == 0);
    f.remove(Flags::N);
    f.remove(Flags::H);
    f.remove(Flags::C);

    Cycles(8)
}

pub fn cp_r(a: u8, r: u8, f: &mut Flags) -> Cycles {
    let res = a as i16 - r as i16;

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(Flags::H, (a & 0xf) < (r & 0xf));
    f.set(Flags::C, res < 0);

    Cycles(4)
}

pub fn cp_n(a: u8, n: u8, f: &mut Flags) -> Cycles {
    let res = a as i16 - n as i16;

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(Flags::H, (a & 0xf) < (n & 0xf));
    f.set(Flags::C, res < 0);

    Cycles(8)
}

pub fn cp_hlptr(a: u8, h: u8, l: u8, f: &mut Flags, mapper: &Mapper) -> Cycles {
    let val = mapper.read(rr(h, l));
    let res = a as i16 - val as i16;

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(Flags::H, (a & 0xf) < (val & 0xf));
    f.set(Flags::C, res < 0);

    Cycles(8)
}

pub fn inc_r(r: &mut u8, f: &mut Flags) -> Cycles {
    let res = if *r == 0xff { 0 } else { *r + 1 };

    f.set(Flags::Z, res == 0);
    f.remove(Flags::N);
    f.set(Flags::H, (((*r & 0xf) + 1) & 0x10) != 0);

    *r = res;
    Cycles(4)
}

pub fn inc_hlptr(h: u8, l: u8, f: &mut Flags, mapper: &mut Mapper) -> Cycles {
    let val = mapper.read(rr(h, l));
    let res = if val == 0xff { 0 } else { val + 1 };

    f.set(Flags::Z, res == 0);
    f.remove(Flags::N);
    f.set(Flags::H, (((val & 0xf) + 1) & 0x10) != 0);

    mapper.write(rr(h, l), res);
    Cycles(12)
}

pub fn dec_r(r: &mut u8, f: &mut Flags) -> Cycles {
    let res = if *r == 0 { 0xff } else { *r - 1 };

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(Flags::H, (*r & 0xf) == 0);

    *r = res;
    Cycles(4)
}

pub fn dec_hlptr(h: u8, l: u8, f: &mut Flags, mapper: &mut Mapper) -> Cycles {
    let val = mapper.read(rr(h, l));
    let res = if val == 0 { 0xff } else { val - 1 };

    f.set(Flags::Z, res == 0);
    f.insert(Flags::N);
    f.set(Flags::H, (val & 0xf) == 0);

    mapper.write(rr(h, l), res);
    Cycles(12)
}

pub fn daa(a: &mut u8, f: &mut Flags) -> Cycles {
    let mut res = *a as i16;

    if f.contains(Flags::N) {
        if f.contains(Flags::C) {
            res -= 0x60
        }
        if f.contains(Flags::H) {
            res -= 0x6
        }
    } else {
        if f.contains(Flags::C) || *a > 0x99 {
            res += 0x60;
            f.insert(Flags::C)
        }
        if f.contains(Flags::H) || (res & 0xf) > 0x9 {
            res += 0x6
        }
    }

    f.set(Flags::Z, res == 0);
    f.remove(Flags::H);

    *a = (res & 0xff) as u8;
    Cycles(4)
}

pub fn cpl(a: &mut u8, f: &mut Flags) -> Cycles {
    *a = *a ^ 0xff;

    f.insert(Flags::N);
    f.insert(Flags::H);

    Cycles(4)
}

pub fn add_hl_rr(h: &mut u8, l: &mut u8, r1: u8, r2: u8, f: &mut Flags) -> Cycles {
    let hl = rr(*h, *l);
    let val = rr(r1, r2);
    let res = hl as u32 + val as u32;
    set_rr(h, l, (res & 0xffff) as u16);

    f.remove(Flags::N);
    f.set(Flags::H, ((res & 0x0f) as u16) + (val & 0x0f) > 0x0f);
    f.set(Flags::C, res & 0xff != 0);

    Cycles(8)
}

pub fn add_hl_sp(h: &mut u8, l: &mut u8, sp: u16, f: &mut Flags) -> Cycles {
    let hl = rr(*h, *l);
    let res = hl as u32 + sp as u32;
    set_rr(h, l, (res & 0xffff) as u16);

    f.remove(Flags::N);
    f.set(Flags::H, (res & 0xf) as u16 + (sp & 0xf) > 0xf);
    f.set(Flags::C, res & 0xff != 0);

    Cycles(8)
}

pub fn inc_rr(r1: &mut u8, r2: &mut u8) -> Cycles {
    let val = rr(*r1, *r2);
    if val < (u16::MAX - 1) {
        set_rr(r1, r2, val + 1);
    } else {
        set_rr(r1, r2, 0);
    }

    Cycles(8)
}

pub fn inc_sp(sp: &mut u16) -> Cycles {
    *sp += 1;

    Cycles(8)
}

pub fn dec_rr(r1: &mut u8, r2: &mut u8) -> Cycles {
    let val = rr(*r1, *r2);
    set_rr(r1, r2, val - 1);

    Cycles(8)
}

pub fn dec_sp(sp: &mut u16) -> Cycles {
    *sp -= 1;

    Cycles(8)
}

pub fn add_sp_dd(sp: &mut u16, dd: i8, f: &mut Flags) -> Cycles {
    let res = *sp as i32 + dd as i32;
    *sp = (res & 0xffff) as u16;

    f.remove(Flags::Z);
    f.remove(Flags::N);
    f.set(Flags::H, (res & 0xf) as u8 + (dd & 0xf) as u8 > 0xf);
    f.set(Flags::C, res as u32 & 0xffff0000 != 0);

    Cycles(16)
}

pub fn ld_hl_sp_dd(h: &mut u8, l: &mut u8, sp: u16, dd: i8, f: &mut Flags) -> Cycles {
    let res = sp as i32 + dd as i32;
    set_rr(h, l, (res & 0xffff) as u16);

    f.remove(Flags::Z);
    f.remove(Flags::N);
    f.set(Flags::H, (res & 0xf) as u8 + (dd & 0xf) as u8 > 0xf);
    f.set(Flags::C, res as u32 & 0xffff0000 != 0);

    Cycles(12)
}
