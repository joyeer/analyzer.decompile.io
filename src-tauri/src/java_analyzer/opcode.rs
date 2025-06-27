
pub const OP_NOP: u8 = 0x00;
pub const OP_ACONST_NULL: u8 = 0x01;
pub const OP_ICONST_M1: u8 = 0x02;
pub const OP_ICONST_0: u8 = 0x03;
pub const OP_ICONST_1: u8 = 0x04;
pub const OP_ICONST_2: u8 = 0x05;
pub const OP_ICONST_3: u8 = 0x06;
pub const OP_ICONST_4: u8 = 0x07;
pub const OP_ICONST_5: u8 = 0x08;
pub const OP_LCONST_0: u8 = 0x09;
pub const OP_LCONST_1: u8 = 0x0A;
pub const OP_FCONST_0: u8 = 0x0B;
pub const OP_FCONST_1: u8 = 0x0C;
pub const OP_FCONST_2: u8 = 0x0D;
pub const OP_DCONST_0: u8 = 0x0E;
pub const OP_DCONST_1: u8 = 0x0F;
pub const OP_BIPUSH: u8 = 0x10;
pub const OP_SIPUSH: u8 = 0x11;
pub const OP_LDC: u8 = 0x12;
pub const OP_LDC_W: u8 = 0x13;
pub const OP_LDC2_W: u8 = 0x14;
pub const OP_ILOAD: u8 = 0x15;
pub const OP_LLOAD: u8 = 0x16;
pub const OP_FLOAD: u8 = 0x17;
pub const OP_DLOAD: u8 = 0x18;
pub const OP_ALOAD: u8 = 0x19;
pub const OP_ILOAD_0: u8 = 0x1A;
pub const OP_ILOAD_1: u8 = 0x1B;
pub const OP_ILOAD_2: u8 = 0x1C;
pub const OP_ILOAD_3: u8 = 0x1D;
pub const OP_LLOAD_0: u8 = 0x1E;
pub const OP_LLOAD_1: u8 = 0x1F;
pub const OP_LLOAD_2: u8 = 0x20;
pub const OP_LLOAD_3: u8 = 0x21;
pub const OP_FLOAD_0: u8 = 0x22;
pub const OP_FLOAD_1: u8 = 0x23;
pub const OP_FLOAD_2: u8 = 0x24;
pub const OP_FLOAD_3: u8 = 0x25;
pub const OP_DLOAD_0: u8 = 0x26;
pub const OP_DLOAD_1: u8 = 0x27;
pub const OP_DLOAD_2: u8 = 0x28;
pub const OP_DLOAD_3: u8 = 0x29;
pub const OP_ALOAD_0: u8 = 0x2A;
pub const OP_ALOAD_1: u8 = 0x2B;
pub const OP_ALOAD_2: u8 = 0x2C;
pub const OP_ALOAD_3: u8 = 0x2D;
pub const OP_IALOAD: u8 = 0x2E;
pub const OP_LALOAD: u8 = 0x2F;
pub const OP_FALOAD: u8 = 0x30;
pub const OP_DALOAD: u8 = 0x31;
pub const OP_AALOAD: u8 = 0x32;
pub const OP_BALOAD: u8 = 0x33;
pub const OP_CALOAD: u8 = 0x34;
pub const OP_SALOAD: u8 = 0x35;
pub const OP_ISTORE: u8 = 0x36;
pub const OP_LSTORE: u8 = 0x37;
pub const OP_FSTORE: u8 = 0x38;
pub const OP_DSTORE: u8 = 0x39;
pub const OP_ASTORE: u8 = 0x3A;
pub const OP_ISTORE_0: u8 = 0x3B;
pub const OP_ISTORE_1: u8 = 0x3C;
pub const OP_ISTORE_2: u8 = 0x3D;
pub const OP_ISTORE_3: u8 = 0x3E;
pub const OP_LSTORE_0: u8 = 0x3F;
pub const OP_LSTORE_1: u8 = 0x40;
pub const OP_LSTORE_2: u8 = 0x41;
pub const OP_LSTORE_3: u8 = 0x42;
pub const OP_FSTORE_0: u8 = 0x43;
pub const OP_FSTORE_1: u8 = 0x44;
pub const OP_FSTORE_2: u8 = 0x45;
pub const OP_FSTORE_3: u8 = 0x46;
pub const OP_DSTORE_0: u8 = 0x47;
pub const OP_DSTORE_1: u8 = 0x48;
pub const OP_DSTORE_2: u8 = 0x49;
pub const OP_DSTORE_3: u8 = 0x4A;
pub const OP_ASTORE_0: u8 = 0x4B;
pub const OP_ASTORE_1: u8 = 0x4C;
pub const OP_ASTORE_2: u8 = 0x4D;
pub const OP_ASTORE_3: u8 = 0x4E;
pub const OP_IASTORE: u8 = 0x4F;
pub const OP_LASTORE: u8 = 0x50;
pub const OP_FASTORE: u8 = 0x51;
pub const OP_DASTORE: u8 = 0x52;
pub const OP_AASTORE: u8 = 0x53;
pub const OP_BASTORE: u8 = 0x54;
pub const OP_CASTORE: u8 = 0x55;
pub const OP_SASTORE: u8 = 0x56;
pub const OP_POP: u8 = 0x57;
pub const OP_POP2: u8 = 0x58;
pub const OP_DUP: u8 = 0x59;
pub const OP_DUP_X1: u8 = 0x5A;
pub const OP_DUP_X2: u8 = 0x5B;
pub const OP_DUP2: u8 = 0x5C;
pub const OP_DUP2_X1: u8 = 0x5D;
pub const OP_DUP2_X2: u8 = 0x5E;
pub const OP_SWAP: u8 = 0x5F;
pub const OP_IADD: u8 = 0x60;
pub const OP_LADD: u8 = 0x61;
pub const OP_FADD: u8 = 0x62;
pub const OP_DADD: u8 = 0x63;
pub const OP_ISUB: u8 = 0x64;
pub const OP_LSUB: u8 = 0x65;
pub const OP_FSUB: u8 = 0x66;
pub const OP_DSUB: u8 = 0x67;
pub const OP_IMUL: u8 = 0x68;
pub const OP_LMUL: u8 = 0x69;
pub const OP_FMUL: u8 = 0x6A;
pub const OP_DMUL: u8 = 0x6B;
pub const OP_IDIV: u8 = 0x6C;
pub const OP_LDIV: u8 = 0x6D;
pub const OP_FDIV: u8 = 0x6E;
pub const OP_DDIV: u8 = 0x6F;
pub const OP_IREM: u8 = 0x70;
pub const OP_LREM: u8 = 0x71;
pub const OP_FREM: u8 = 0x72;
pub const OP_DREM: u8 = 0x73;
pub const OP_INEG: u8 = 0x74;
pub const OP_LNEG: u8 = 0x75;
pub const OP_FNEG: u8 = 0x76;
pub const OP_DNEG: u8 = 0x77;
pub const OP_ISHL: u8 = 0x78;
pub const OP_LSHL: u8 = 0x79;
pub const OP_ISHR: u8 = 0x7A;
pub const OP_LSHR: u8 = 0x7B;
pub const OP_IUSHR: u8 = 0x7C;
pub const OP_LUSHR: u8 = 0x7D;
pub const OP_IAND: u8 = 0x7E;
pub const OP_LAND: u8 = 0x7F;
pub const OP_IOR: u8 = 0x80;
pub const OP_LOR: u8 = 0x81;
pub const OP_IXOR: u8 = 0x82;
pub const OP_LXOR: u8 = 0x83;
pub const OP_IINC: u8 = 0x84;
pub const OP_I2L: u8 = 0x85;
pub const OP_I2F: u8 = 0x86;
pub const OP_I2D: u8 = 0x87;
pub const OP_L2I: u8 = 0x88;
pub const OP_L2F: u8 = 0x89;
pub const OP_L2D: u8 = 0x8A;
pub const OP_F2I: u8 = 0x8B;
pub const OP_F2L: u8 = 0x8C;
pub const OP_F2D: u8 = 0x8D;
pub const OP_D2I: u8 = 0x8E;
pub const OP_D2L: u8 = 0x8F;
pub const OP_D2F: u8 = 0x90;
pub const OP_I2B: u8 = 0x91;
pub const OP_I2C: u8 = 0x92;
pub const OP_I2S: u8 = 0x93;
pub const OP_LCMP: u8 = 0x94;
pub const OP_FCMPL: u8 = 0x95;
pub const OP_FCMPG: u8 = 0x96;
pub const OP_DCMPL: u8 = 0x97;
pub const OP_DCMPG: u8 = 0x98;
pub const OP_IFEQ: u8 = 0x99;
pub const OP_IFNE: u8 = 0x9A;
pub const OP_IFLT: u8 = 0x9B;
pub const OP_IFGE: u8 = 0x9C;
pub const OP_IFGT: u8 = 0x9D;
pub const OP_IFLE: u8 = 0x9E;
pub const OP_IF_ICMPEQ: u8 = 0x9F;
pub const OP_IF_ICMPNE: u8 = 0xA0;
pub const OP_IF_ICMPLT: u8 = 0xA1;
pub const OP_IF_ICMPGE: u8 = 0xA2;
pub const OP_IF_ICMPGT: u8 = 0xA3;
pub const OP_IF_ICMPLE: u8 = 0xA4;
pub const OP_IF_ACMPEQ: u8 = 0xA5;
pub const OP_IF_ACMPNE: u8 = 0xA6;
pub const OP_GETSTATIC: u8 = 0xB2;
pub const OP_PUTSTATIC: u8 = 0xB3;
pub const OP_GETFIELD: u8 = 0xB4;
pub const OP_PUTFIELD: u8 = 0xB5;
pub const OP_INVOKEVIRTUAL: u8 = 0xB6;
pub const OP_INVOKESPECIAL: u8 = 0xB7;
pub const OP_INVOKESTATIC: u8 = 0xB8;
pub const OP_INVOKEINTERFACE: u8 = 0xB9;
pub const OP_INVOKEDYNAMIC: u8 = 0xBA;
pub const OP_NEW: u8 = 0xBB;
pub const OP_NEWARRAY: u8 = 0xBC;
pub const OP_ANEWARRAY: u8 = 0xBD;
pub const OP_ARRAYLENGTH: u8 = 0xBE;
pub const OP_ATHROW: u8 = 0xBF;
pub const OP_CHECKCAST: u8 = 0xC0;
pub const OP_INSTANCEOF: u8 = 0xC1;
pub const OP_MONITORENTER: u8 = 0xC2;
pub const OP_MONITOREXIT: u8 = 0xC3;
pub const OP_GOTO: u8 = 0xA7;
pub const OP_JSR: u8 = 0xA8;
pub const OP_RET: u8 = 0xA9;
pub const OP_TABLESWITCH: u8 = 0xAA;
pub const OP_LOOKUPSWITCH: u8 = 0xAB;
pub const OP_IRETURN: u8 = 0xAC;
pub const OP_LRETURN: u8 = 0xAD;
pub const OP_FRETURN: u8 = 0xAE;
pub const OP_DRETURN: u8 = 0xAF;
pub const OP_ARETURN: u8 = 0xB0;
pub const OP_RETURN: u8 = 0xB1;
pub const OP_WIDE: u8 = 0xC4;
pub const OP_MULTIANEWARRAY: u8 = 0xC5;
pub const OP_IFNULL: u8 = 0xC6;
pub const OP_IFNONNULL: u8 = 0xC7;
pub const OP_GOTO_W: u8 = 0xC8;
pub const OP_JSR_W: u8 = 0xC9;
pub const OP_BREAKPOINT: u8 = 0xCA;
pub const OP_IMPDEP1: u8 = 0xFE;
pub const OP_IMPDEP2: u8 = 0xFF;

pub enum ArrayType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
}

#[derive(Debug, Default)]
pub struct Instruction {
    pub opcode: u8,
    pub offset: u32,
    pub value: i32,
    pub value2: i32,
    pub pairs: Vec<(u16, u16)>,
}

impl Instruction {
    pub fn new(opcode: u8, offset: u32) -> Self {
        Self {
            opcode,
            offset,
            ..Default::default()
        }
    }

    pub fn new2(opcode: u8, offset: u32, value: i32) -> Self {
        Self {
            opcode,
            offset,
            value,
            ..Default::default()
        }
    }

    pub fn new3(opcode: u8, offset: u32, value: i32, value2: i32) -> Self {
        Self {
            opcode,
            offset,
            value,
            value2,
            ..Default::default()
        }
    }
}
