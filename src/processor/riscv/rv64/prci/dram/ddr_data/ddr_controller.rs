pub const DDR_CTL_BASE_PTR: *mut u32 = 0x100B_0000 as *mut _;
pub const DDR_CTL_LEN: usize = 265;
pub const DDR_CTL_REGISTER_0_START_MASK: u32 = 0x1;
pub const DDR_CTL_REGISTER_132: usize = 132;
pub const DDR_CTL_REGISTER_132_INT_STATUS_8: u32 = 0x1 << 8;
pub const DDR_CTL_REGISTER_136: usize = 136;
pub const DDR_CTL_REGISTER_136_DISABLE_ALL_INTERRUPTS: u32 = 0xffff_ffff;
pub const DDR_CTL_CONFIG: [u32; DDR_CTL_LEN] = [
    0x0000_0a00, // DENALI_CTL_00_DATA
    0x0000_0000, // DENALI_CTL_01_DATA
    0x0000_0000, // DENALI_CTL_02_DATA
    0x0000_0000, // DENALI_CTL_03_DATA
    0x0000_0000, // DENALI_CTL_04_DATA
    0x0000_0000, // DENALI_CTL_05_DATA
    0x0000_000a, // DENALI_CTL_06_DATA
    0x0002_d362, // DENALI_CTL_07_DATA
    0x0007_1073, // DENALI_CTL_08_DATA
    0x0a1c_0255, // DENALI_CTL_09_DATA
    0x1c1c_0400, // DENALI_CTL_10_DATA
    0x0404_c90b, // DENALI_CTL_11_DATA
    0x2b05_0405, // DENALI_CTL_12_DATA
    0x0d0c_081e, // DENALI_CTL_13_DATA
    0x0809_0914, // DENALI_CTL_14_DATA
    0x00fd_e718, // DENALI_CTL_15_DATA
    0x0018_0a05, // DENALI_CTL_16_DATA
    0x008b_130d, // DENALI_CTL_17_DATA
    0x0100_0118, // DENALI_CTL_18_DATA
    0x0d03_2001, // DENALI_CTL_19_DATA
    0x0000_0000, // DENALI_CTL_20_DATA
    0x0000_0101, // DENALI_CTL_21_DATA
    0x0000_0000, // DENALI_CTL_22_DATA
    0x0a00_0000, // DENALI_CTL_23_DATA
    0x0000_0000, // DENALI_CTL_24_DATA
    0x0145_0100, // DENALI_CTL_25_DATA
    0x0000_1c36, // DENALI_CTL_26_DATA
    0x0000_0005, // DENALI_CTL_27_DATA
    0x0017_0006, // DENALI_CTL_28_DATA
    0x014e_0400, // DENALI_CTL_29_DATA
    0x0301_0000, // DENALI_CTL_30_DATA
    0x000a_0e00, // DENALI_CTL_31_DATA
    0x0403_0200, // DENALI_CTL_32_DATA
    0x0000_031f, // DENALI_CTL_33_DATA
    0x0007_0004, // DENALI_CTL_34_DATA
    0x0000_0000, // DENALI_CTL_35_DATA
    0x0000_0000, // DENALI_CTL_36_DATA
    0x0000_0000, // DENALI_CTL_37_DATA
    0x0000_0000, // DENALI_CTL_38_DATA
    0x0000_0000, // DENALI_CTL_39_DATA
    0x0000_0000, // DENALI_CTL_40_DATA
    0x0000_0000, // DENALI_CTL_41_DATA
    0x0000_0000, // DENALI_CTL_42_DATA
    0x0000_0000, // DENALI_CTL_43_DATA
    0x0000_0000, // DENALI_CTL_44_DATA
    0x0000_0000, // DENALI_CTL_45_DATA
    0x0000_0000, // DENALI_CTL_46_DATA
    0x0000_0000, // DENALI_CTL_47_DATA
    0x0000_0000, // DENALI_CTL_48_DATA
    0x0000_0000, // DENALI_CTL_49_DATA
    0x0000_0000, // DENALI_CTL_50_DATA
    0x0000_0000, // DENALI_CTL_51_DATA
    0x0000_0000, // DENALI_CTL_52_DATA
    0x0000_0000, // DENALI_CTL_53_DATA
    0x0000_0000, // DENALI_CTL_54_DATA
    0x0000_0000, // DENALI_CTL_55_DATA
    0x0000_0000, // DENALI_CTL_56_DATA
    0x0000_0000, // DENALI_CTL_57_DATA
    0x0000_0000, // DENALI_CTL_58_DATA
    0x0000_0000, // DENALI_CTL_59_DATA
    0x0000_0424, // DENALI_CTL_60_DATA
    0x0000_0201, // DENALI_CTL_61_DATA
    0x0000_1008, // DENALI_CTL_62_DATA
    0x0000_0000, // DENALI_CTL_63_DATA
    0x0000_0200, // DENALI_CTL_64_DATA
    0x0000_0800, // DENALI_CTL_65_DATA
    0x0000_0481, // DENALI_CTL_66_DATA
    0x0000_0400, // DENALI_CTL_67_DATA
    0x0000_0424, // DENALI_CTL_68_DATA
    0x0000_0201, // DENALI_CTL_69_DATA
    0x0000_1008, // DENALI_CTL_70_DATA
    0x0000_0000, // DENALI_CTL_71_DATA
    0x0000_0200, // DENALI_CTL_72_DATA
    0x0000_0800, // DENALI_CTL_73_DATA
    0x0000_0481, // DENALI_CTL_74_DATA
    0x0000_0400, // DENALI_CTL_75_DATA
    0x0101_0000, // DENALI_CTL_76_DATA
    0x0000_0000, // DENALI_CTL_77_DATA
    0x0000_0000, // DENALI_CTL_78_DATA
    0x0000_0000, // DENALI_CTL_79_DATA
    0x0000_0000, // DENALI_CTL_80_DATA
    0x0000_0000, // DENALI_CTL_81_DATA
    0x0000_0000, // DENALI_CTL_82_DATA
    0x0000_0000, // DENALI_CTL_83_DATA
    0x0000_0000, // DENALI_CTL_84_DATA
    0x0000_0000, // DENALI_CTL_85_DATA
    0x0000_0000, // DENALI_CTL_86_DATA
    0x0000_0000, // DENALI_CTL_87_DATA
    0x0000_0000, // DENALI_CTL_88_DATA
    0x0000_0000, // DENALI_CTL_89_DATA
    0x0000_0000, // DENALI_CTL_90_DATA
    0x0000_0000, // DENALI_CTL_91_DATA
    0x0000_0000, // DENALI_CTL_92_DATA
    0x0000_0000, // DENALI_CTL_93_DATA
    0x0000_0000, // DENALI_CTL_94_DATA
    0x0000_0000, // DENALI_CTL_95_DATA
    0x0000_0000, // DENALI_CTL_96_DATA
    0x0000_0000, // DENALI_CTL_97_DATA
    0x0000_0000, // DENALI_CTL_98_DATA
    0x0000_0000, // DENALI_CTL_99_DATA
    0x0000_0000, // DENALI_CTL_100_DATA
    0x0000_0000, // DENALI_CTL_101_DATA
    0x0000_0000, // DENALI_CTL_102_DATA
    0x0000_0000, // DENALI_CTL_103_DATA
    0x0000_0000, // DENALI_CTL_104_DATA
    0x0000_0003, // DENALI_CTL_105_DATA
    0x0000_0000, // DENALI_CTL_106_DATA
    0x0000_0000, // DENALI_CTL_107_DATA
    0x0000_0000, // DENALI_CTL_108_DATA
    0x0000_0000, // DENALI_CTL_109_DATA
    0x0100_0000, // DENALI_CTL_110_DATA
    0x0004_0000, // DENALI_CTL_111_DATA
    0x0080_0200, // DENALI_CTL_112_DATA
    0x0000_0200, // DENALI_CTL_113_DATA
    0x0000_0040, // DENALI_CTL_114_DATA
    0x0100_0100, // DENALI_CTL_115_DATA
    0x0a00_0002, // DENALI_CTL_116_DATA
    0x0101_ffff, // DENALI_CTL_117_DATA
    0x0101_0101, // DENALI_CTL_118_DATA
    0x0101_0101, // DENALI_CTL_119_DATA
    0x0000_010b, // DENALI_CTL_120_DATA
    0x0000_0c03, // DENALI_CTL_121_DATA
    0x0000_0000, // DENALI_CTL_122_DATA
    0x0000_0000, // DENALI_CTL_123_DATA
    0x0000_0000, // DENALI_CTL_124_DATA
    0x0000_0000, // DENALI_CTL_125_DATA
    0x0003_0300, // DENALI_CTL_126_DATA
    0x0000_0000, // DENALI_CTL_127_DATA
    0x0001_0101, // DENALI_CTL_128_DATA
    0x0000_0000, // DENALI_CTL_129_DATA
    0x0000_0000, // DENALI_CTL_130_DATA
    0x0000_0000, // DENALI_CTL_131_DATA
    0x0000_0000, // DENALI_CTL_132_DATA
    0x0000_0000, // DENALI_CTL_133_DATA
    0x0000_0000, // DENALI_CTL_134_DATA
    0x0000_0000, // DENALI_CTL_135_DATA
    0x0000_0000, // DENALI_CTL_136_DATA
    0x0000_0000, // DENALI_CTL_137_DATA
    0x0000_0000, // DENALI_CTL_138_DATA
    0x0000_0000, // DENALI_CTL_139_DATA
    0x0000_0000, // DENALI_CTL_140_DATA
    0x0000_0000, // DENALI_CTL_141_DATA
    0x0000_0000, // DENALI_CTL_142_DATA
    0x0000_0000, // DENALI_CTL_143_DATA
    0x0000_0000, // DENALI_CTL_144_DATA
    0x0000_0000, // DENALI_CTL_145_DATA
    0x0000_0000, // DENALI_CTL_146_DATA
    0x0000_0000, // DENALI_CTL_147_DATA
    0x0000_0000, // DENALI_CTL_148_DATA
    0x0000_0000, // DENALI_CTL_149_DATA
    0x0000_0000, // DENALI_CTL_150_DATA
    0x0000_0000, // DENALI_CTL_151_DATA
    0x0000_0000, // DENALI_CTL_152_DATA
    0x0000_0000, // DENALI_CTL_153_DATA
    0x0000_0000, // DENALI_CTL_154_DATA
    0x0000_0000, // DENALI_CTL_155_DATA
    0x0000_0000, // DENALI_CTL_156_DATA
    0x0000_0000, // DENALI_CTL_157_DATA
    0x0000_0000, // DENALI_CTL_158_DATA
    0x0000_0000, // DENALI_CTL_159_DATA
    0x0000_0000, // DENALI_CTL_160_DATA
    0x0201_0102, // DENALI_CTL_161_DATA
    0x0108_070d, // DENALI_CTL_162_DATA
    0x0505_0300, // DENALI_CTL_163_DATA
    0x0400_0503, // DENALI_CTL_164_DATA
    0x0000_0000, // DENALI_CTL_165_DATA
    0x0000_0000, // DENALI_CTL_166_DATA
    0x0000_0000, // DENALI_CTL_167_DATA
    0x0000_0000, // DENALI_CTL_168_DATA
    0x280d_0000, // DENALI_CTL_169_DATA
    0x0100_0000, // DENALI_CTL_170_DATA
    0x0000_0000, // DENALI_CTL_171_DATA
    0x0003_0001, // DENALI_CTL_172_DATA
    0x0000_0000, // DENALI_CTL_173_DATA
    0x0000_0000, // DENALI_CTL_174_DATA
    0x0000_0000, // DENALI_CTL_175_DATA
    0x0000_0000, // DENALI_CTL_176_DATA
    0x0000_0000, // DENALI_CTL_177_DATA
    0x0000_0000, // DENALI_CTL_178_DATA
    0x0000_0000, // DENALI_CTL_179_DATA
    0x0000_0000, // DENALI_CTL_180_DATA
    0x0100_0000, // DENALI_CTL_181_DATA
    0x0000_0001, // DENALI_CTL_182_DATA
    0x0000_0100, // DENALI_CTL_183_DATA
    0x0001_0303, // DENALI_CTL_184_DATA
    0x6767_6701, // DENALI_CTL_185_DATA
    0x6767_6767, // DENALI_CTL_186_DATA
    0x6767_6767, // DENALI_CTL_187_DATA
    0x6767_6767, // DENALI_CTL_188_DATA
    0x6767_6767, // DENALI_CTL_189_DATA
    0x6767_6767, // DENALI_CTL_190_DATA
    0x6767_6767, // DENALI_CTL_191_DATA
    0x6767_6767, // DENALI_CTL_192_DATA
    0x6767_6767, // DENALI_CTL_193_DATA
    0x0100_0067, // DENALI_CTL_194_DATA
    0x0000_0001, // DENALI_CTL_195_DATA
    0x0000_0101, // DENALI_CTL_196_DATA
    0x0000_0000, // DENALI_CTL_197_DATA
    0x0000_0000, // DENALI_CTL_198_DATA
    0x0000_0000, // DENALI_CTL_199_DATA
    0x0000_0000, // DENALI_CTL_200_DATA
    0x0000_0000, // DENALI_CTL_201_DATA
    0x0000_0000, // DENALI_CTL_202_DATA
    0x0000_0000, // DENALI_CTL_203_DATA
    0x0000_0000, // DENALI_CTL_204_DATA
    0x0000_0000, // DENALI_CTL_205_DATA
    0x0000_0000, // DENALI_CTL_206_DATA
    0x0000_0000, // DENALI_CTL_207_DATA
    0x0000_0001, // DENALI_CTL_208_DATA
    0x0000_0000, // DENALI_CTL_209_DATA
    0x007f_ffff, // DENALI_CTL_210_DATA
    0x0000_0000, // DENALI_CTL_211_DATA
    0x007f_ffff, // DENALI_CTL_212_DATA
    0x0000_0000, // DENALI_CTL_213_DATA
    0x007f_ffff, // DENALI_CTL_214_DATA
    0x0000_0000, // DENALI_CTL_215_DATA
    0x007f_ffff, // DENALI_CTL_216_DATA
    0x0000_0000, // DENALI_CTL_217_DATA
    0x007f_ffff, // DENALI_CTL_218_DATA
    0x0000_0000, // DENALI_CTL_219_DATA
    0x007f_ffff, // DENALI_CTL_220_DATA
    0x0000_0000, // DENALI_CTL_221_DATA
    0x007f_ffff, // DENALI_CTL_222_DATA
    0x0000_0000, // DENALI_CTL_223_DATA
    0x037f_ffff, // DENALI_CTL_224_DATA
    0xffff_ffff, // DENALI_CTL_225_DATA
    0x000f_000f, // DENALI_CTL_226_DATA
    0x00ff_ff03, // DENALI_CTL_227_DATA
    0x000f_ffff, // DENALI_CTL_228_DATA
    0x0003_000f, // DENALI_CTL_229_DATA
    0xffff_ffff, // DENALI_CTL_230_DATA
    0x000f_000f, // DENALI_CTL_231_DATA
    0x00ff_ff03, // DENALI_CTL_232_DATA
    0x000f_ffff, // DENALI_CTL_233_DATA
    0x0003_000f, // DENALI_CTL_234_DATA
    0xffff_ffff, // DENALI_CTL_235_DATA
    0x000f_000f, // DENALI_CTL_236_DATA
    0x00ff_ff03, // DENALI_CTL_237_DATA
    0x000f_ffff, // DENALI_CTL_238_DATA
    0x0003_000f, // DENALI_CTL_239_DATA
    0xffff_ffff, // DENALI_CTL_240_DATA
    0x000f_000f, // DENALI_CTL_241_DATA
    0x00ff_ff03, // DENALI_CTL_242_DATA
    0x000f_ffff, // DENALI_CTL_243_DATA
    0x6407_000f, // DENALI_CTL_244_DATA
    0x0164_0001, // DENALI_CTL_245_DATA
    0x0000_0000, // DENALI_CTL_246_DATA
    0x0000_0000, // DENALI_CTL_247_DATA
    0x0000_1800, // DENALI_CTL_248_DATA
    0x0038_6c05, // DENALI_CTL_249_DATA
    0x0200_0200, // DENALI_CTL_250_DATA
    0x0200_0200, // DENALI_CTL_251_DATA
    0x0000_386c, // DENALI_CTL_252_DATA
    0x0002_3438, // DENALI_CTL_253_DATA
    0x0202_0d0f, // DENALI_CTL_254_DATA
    0x0014_0303, // DENALI_CTL_255_DATA
    0x0000_0000, // DENALI_CTL_256_DATA
    0x0000_0000, // DENALI_CTL_257_DATA
    0x0000_1403, // DENALI_CTL_258_DATA
    0x0000_0000, // DENALI_CTL_259_DATA
    0x0000_0000, // DENALI_CTL_260_DATA
    0x0000_0000, // DENALI_CTL_261_DATA
    0x0000_0000, // DENALI_CTL_262_DATA
    0x0c01_0000, // DENALI_CTL_263_DATA
    0x0000_0008, // DENALI_CTL_264_DATA
];
pub const DDR_PHYSICAL_FILTER: *mut u64 = 0x100B_8000 as *mut _;
pub const DDR_PHYSICAL_FILTER_PMP_0_INIT: u64 = 0x0f00_0008_0000_0000;
