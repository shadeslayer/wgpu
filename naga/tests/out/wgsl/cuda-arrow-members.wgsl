struct Leaf_cuda_record_0_ {
    value: i32,
}

struct int3_ {
    x: i32,
    y: i32,
    z: i32,
}

struct Record_cuda_record_1_ {
    nested: Leaf_cuda_record_0_,
    point: int3_,
    samples: array<i32, 3>,
}

@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

@compute @workgroup_size(1, 1, 1)
fn arrow_members() {
    var records: array<Record_cuda_record_1_, 3>;
    var local: u32;
    var local_1: i32;
    var local_2: u32;
    var local_3: u32;
    var local_4: u32;
    var local_5: u32;
    var local_6: u32;
    var local_7: u32;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var local_11: u32;
    var local_12: u32;
    var local_13: u32;
    var local_14: u32;
    var local_15: u32;
    var local_16: u32;
    var local_17: u32;
    var local_18: i32;
    var local_19: u32;
    var local_20: u32;
    var local_21: u32;
    var local_22: i32;
    var index: i32;
    var local_23: u32;
    var local_24: u32;
    var local_25: u32;
    var local_26: i32;
    var local_27: u32;
    var local_28: u32;
    var choose: bool;
    var local_29: u32;
    var local_30: u32;
    var local_31: u32;
    var local_32: u32;
    var local_33: u32;
    var local_34: u32;
    var local_35: u32;
    var local_36: u32;
    var frozen: Record_cuda_record_1_;
    var pair: vec2<i32>;
    var mutable_pair: vec2<i32>;
    var select_: bool;
    var local_37: u32;
    var local_38: u32;
    var local_39: u32;
    var local_40: i32;
    var local_41: u32;
    var local_42: u32;
    var local_43: u32;
    var local_44: u32;
    var local_45: u32;
    var local_46: u32;
    var local_47: i32;
    var local_48: u32;
    var local_49: i32;
    var local_50: u32;
    var local_51: i32;
    var local_52: u32;
    var local_53: i32;
    var local_54: u32;
    var local_55: i32;
    var local_56: u32;
    var local_57: i32;
    var local_58: u32;
    var local_59: i32;
    var local_60: u32;
    var local_61: i32;
    var local_62: u32;
    var local_63: i32;
    var local_64: u32;
    var local_65: u32;
    var local_66: i32;
    var local_67: i32;
    var local_68: i32;
    var local_69: i32;
    var local_70: i32;
    var local_71: i32;
    var local_72: u32;
    var local_73: i32;
    var local_74: u32;
    var local_75: i32;
    var local_76: u32;
    var local_77: i32;
    var local_78: u32;
    var local_79: i32;
    var local_80: u32;
    var local_81: i32;
    var local_82: u32;
    var local_83: i32;
    var local_84: u32;
    var local_85: i32;
    var local_86: u32;
    var local_87: i32;
    var local_88: u32;
    var local_89: i32;
    var local_90: i32;
    var local_91: i32;
    var local_92: u32;
    var local_93: i32;
    var local_94: u32;
    var local_95: i32;
    var local_96: u32;
    var local_97: i32;
    var local_98: u32;
    var local_99: i32;
    var local_100: u32;
    var local_101: u32;
    var local_102: i32;
    var local_103: i32;
    var local_104: i32;
    var local_105: u32;
    var local_106: u32;
    var local_107: i32;
    var local_108: i32;
    var local_109: i32;
    var local_110: u32;
    var local_111: u32;
    var local_112: i32;
    var local_113: u32;
    var local_114: i32;
    var local_115: u32;
    var local_116: u32;
    var local_117: i32;
    var local_118: u32;
    var local_119: i32;
    var local_120: u32;
    var local_121: i32;
    var local_122: u32;
    var local_123: u32;
    var local_124: u32;
    var local_125: i32;
    var local_126: i32;
    var local_127: u32;
    var local_128: i32;
    var local_129: u32;
    var local_130: i32;
    var local_131: u32;
    var local_132: i32;
    var local_133: u32;
    var local_134: i32;
    var local_135: u32;
    var local_136: i32;
    var local_137: u32;
    var local_138: i32;
    var local_139: u32;
    var local_140: u32;
    var local_141: u32;
    var local_142: i32;
    var local_143: u32;
    var local_144: i32;
    var local_145: u32;
    var local_146: u32;
    var local_147: u32;
    var local_148: u32;
    var local_149: u32;
    var local_150: i32;
    var local_151: u32;
    var local_152: i32;
    var local_153: i32;
    var local_154: i32;
    var local_155: i32;
    var local_156: i32;
    var local_157: u32;
    var local_158: u32;
    var local_159: u32;
    var local_160: i32;
    var local_161: u32;
    var local_162: i32;
    var local_163: u32;
    var local_164: i32;
    var local_165: u32;
    var local_166: i32;

    records[0].nested.value = 2i;
    records[0].point.x = 3i;
    records[0].point.y = 4i;
    records[0].point.z = 5i;
    records[0].samples[0] = 6i;
    records[0].samples[1] = 7i;
    records[0].samples[2] = 8i;
    records[1].nested.value = 11i;
    records[1].point.x = 13i;
    records[1].point.y = 17i;
    records[1].point.z = 19i;
    records[1].samples[0] = 23i;
    records[1].samples[1] = 29i;
    records[1].samples[2] = 31i;
    records[2].nested.value = 37i;
    records[2].point.x = 41i;
    records[2].point.y = 43i;
    records[2].point.z = 47i;
    records[2].samples[0] = 53i;
    records[2].samples[1] = 59i;
    records[2].samples[2] = 61i;
    local_46 = u32(0u);
    let _e59 = local_46;
    let _e63 = records[_e59].nested.value;
    local_47 = _e63;
    let _e64 = local_47;
    out[0i] = u32(_e64);
    local_48 = u32(0u);
    let _e73 = local_48;
    let _e77 = records[_e73].nested.value;
    local_49 = _e77;
    let _e78 = local_49;
    let _e81 = local_48;
    records[_e81].nested.value = (_e78 + 3i);
    local_50 = u32(0u);
    let _e89 = local_50;
    let _e93 = records[_e89].nested.value;
    local_51 = _e93;
    let _e94 = local_51;
    out[1i] = u32(_e94);
    local = u32(1i);
    let _e101 = local;
    local_52 = u32(_e101);
    let _e105 = local_52;
    let _e109 = records[_e105].point.y;
    local_53 = _e109;
    let _e110 = local_53;
    let _e113 = local_52;
    records[_e113].point.y = (_e110 + 1i);
    local_1 = _e110;
    let _e118 = local_1;
    out[2i] = u32(_e118);
    local_2 = u32(1i);
    let _e125 = local_2;
    local_54 = u32(_e125);
    let _e129 = local_54;
    let _e133 = records[_e129].point.y;
    local_55 = _e133;
    let _e134 = local_55;
    out[3i] = u32(_e134);
    local_3 = u32(1i);
    let _e141 = local_3;
    local_56 = u32(_e141);
    let _e145 = local_56;
    let _e149 = records[_e145].point.z;
    local_57 = _e149;
    let _e150 = local_57;
    let _e153 = local_56;
    records[_e153].point.z = (_e150 + 1i);
    let _e157 = local_3;
    local_58 = u32(_e157);
    let _e161 = local_58;
    let _e165 = records[_e161].point.z;
    local_59 = _e165;
    let _e166 = local_59;
    out[4i] = u32(_e166);
    local_4 = u32(1i);
    local_5 = u32(1i);
    let _e176 = local_4;
    local_60 = u32(_e176);
    let _e180 = local_60;
    let _e183 = local_5;
    let _e185 = records[_e180].samples[_e183];
    local_61 = _e185;
    let _e186 = local_61;
    let _e189 = local_60;
    let _e192 = local_5;
    records[_e189].samples[_e192] = (_e186 * 2i);
    local_6 = u32(1i);
    local_7 = u32(1i);
    let _e200 = local_6;
    local_62 = u32(_e200);
    let _e204 = local_62;
    let _e207 = local_7;
    let _e209 = records[_e204].samples[_e207];
    local_63 = _e209;
    let _e210 = local_63;
    out[5i] = u32(_e210);
    local_8 = u32(1i);
    local_64 = u32();
    local_65 = u32();
    local_66 = i32();
    local_67 = i32();
    local_68 = i32();
    local_69 = i32();
    local_70 = i32();
    local_71 = i32();
    local_64 = u32(0i);
    local_72 = u32(0u);
    let _e240 = local_72;
    let _e243 = local_64;
    let _e245 = records[_e240].samples[_e243];
    local_73 = _e245;
    let _e246 = local_73;
    local_66 = _e246;
    let _e248 = local_66;
    let _e249 = local_8;
    local_74 = u32(_e249);
    let _e253 = local_74;
    let _e257 = records[_e253].nested.value;
    local_75 = _e257;
    let _e258 = local_75;
    local_67 = _e258;
    let _e260 = local_67;
    let _e262 = local_74;
    records[_e262].nested.value = (_e260 + _e248);
    local_76 = u32(0u);
    let _e270 = local_76;
    let _e274 = records[_e270].point.z;
    local_77 = _e274;
    let _e275 = local_77;
    local_68 = _e275;
    let _e277 = local_68;
    let _e278 = local_8;
    local_78 = u32(_e278);
    let _e282 = local_78;
    let _e286 = records[_e282].point.y;
    local_79 = _e286;
    let _e287 = local_79;
    local_69 = _e287;
    let _e289 = local_69;
    let _e291 = local_78;
    records[_e291].point.y = (_e289 + _e277);
    let _e295 = local_8;
    local_80 = u32(_e295);
    let _e299 = local_80;
    let _e303 = records[_e299].nested.value;
    local_81 = _e303;
    let _e304 = local_81;
    local_70 = _e304;
    let _e306 = local_70;
    local_65 = u32(1i);
    let _e310 = local_8;
    local_82 = u32(_e310);
    let _e314 = local_82;
    let _e317 = local_65;
    let _e319 = records[_e314].samples[_e317];
    local_83 = _e319;
    let _e320 = local_83;
    local_71 = _e320;
    let _e322 = local_71;
    let _e324 = local_82;
    let _e327 = local_65;
    records[_e324].samples[_e327] = (_e322 + _e306);
    local_9 = u32(1i);
    let _e332 = local_9;
    local_84 = u32(_e332);
    let _e336 = local_84;
    let _e340 = records[_e336].nested.value;
    local_85 = _e340;
    let _e341 = local_85;
    out[6i] = u32(_e341);
    local_10 = u32(1i);
    let _e348 = local_10;
    local_86 = u32(_e348);
    let _e352 = local_86;
    let _e356 = records[_e352].point.y;
    local_87 = _e356;
    let _e357 = local_87;
    out[7i] = u32(_e357);
    local_11 = u32(1i);
    local_12 = u32(1i);
    let _e367 = local_11;
    local_88 = u32(_e367);
    let _e371 = local_88;
    let _e374 = local_12;
    let _e376 = records[_e371].samples[_e374];
    local_89 = _e376;
    let _e377 = local_89;
    out[8i] = u32(_e377);
    local_13 = u32(1i);
    local_90 = i32();
    local_91 = i32();
    let _e388 = local_13;
    local_92 = u32(_e388);
    let _e392 = local_92;
    let _e396 = records[_e392].nested.value;
    local_93 = _e396;
    let _e397 = local_93;
    local_90 = _e397;
    let _e399 = local_90;
    let _e402 = local_92;
    records[_e402].nested.value = (_e399 + 3i);
    let _e406 = local_13;
    local_94 = u32(_e406);
    let _e410 = local_94;
    let _e414 = records[_e410].point.x;
    local_95 = _e414;
    let _e415 = local_95;
    local_91 = _e415;
    let _e417 = local_91;
    let _e420 = local_94;
    records[_e420].point.x = (_e417 + 3i);
    local_14 = u32(1i);
    let _e427 = local_14;
    local_96 = u32(_e427);
    let _e431 = local_96;
    let _e435 = records[_e431].nested.value;
    local_97 = _e435;
    let _e436 = local_97;
    out[9i] = u32(_e436);
    local_15 = u32(1i);
    let _e443 = local_15;
    local_98 = u32(_e443);
    let _e447 = local_98;
    let _e451 = records[_e447].point.x;
    local_99 = _e451;
    let _e452 = local_99;
    out[10i] = u32(_e452);
    local_100 = u32(0u);
    local_101 = u32();
    local_102 = i32();
    local_103 = i32();
    local_104 = i32();
    local_105 = u32(0u);
    let _e471 = local_105;
    records[_e471].nested.value = 5i;
    local_106 = u32(0u);
    let _e479 = local_106;
    let _e483 = records[_e479].nested.value;
    local_107 = _e483;
    let _e484 = local_107;
    local_102 = _e484;
    let _e486 = local_102;
    let _e488 = local_100;
    let _e492 = records[_e488].nested.value;
    local_108 = _e492;
    let _e493 = local_108;
    local_103 = _e493;
    let _e495 = local_103;
    let _e497 = local_100;
    records[_e497].nested.value = (_e495 + _e486);
    let _e502 = local_100;
    let _e506 = records[_e502].nested.value;
    local_109 = _e506;
    let _e507 = local_109;
    local_104 = _e507;
    let _e509 = local_104;
    local_101 = u32(0i);
    local_110 = u32(0u);
    let _e516 = local_110;
    let _e519 = local_101;
    records[_e516].samples[_e519] = _e509;
    local_111 = u32(0u);
    let _e525 = local_111;
    let _e529 = records[_e525].nested.value;
    local_112 = _e529;
    let _e530 = local_112;
    out[11i] = u32(_e530);
    local_16 = u32(0i);
    local_113 = u32(0u);
    let _e541 = local_113;
    let _e544 = local_16;
    let _e546 = records[_e541].samples[_e544];
    local_114 = _e546;
    let _e547 = local_114;
    out[12i] = u32(_e547);
    local_17 = u32(3i);
    local_115 = u32();
    local_116 = u32();
    local_117 = i32();
    local_115 = u32(1i);
    local_116 = u32(2i);
    let _e567 = local_115;
    let _e570 = local_17;
    local_118 = u32(((0u - _e567) + _e570));
    let _e575 = local_118;
    let _e578 = local_116;
    let _e580 = records[_e575].samples[_e578];
    local_119 = _e580;
    let _e581 = local_119;
    local_117 = _e581;
    let _e583 = local_117;
    local_18 = _e583;
    let _e584 = local_18;
    out[13i] = u32(_e584);
    local_19 = u32(3i);
    local_20 = u32(2i);
    let _e594 = local_20;
    let _e597 = local_19;
    local_120 = u32(((0u - _e594) + _e597));
    let _e602 = local_120;
    let _e606 = records[_e602].nested.value;
    local_121 = _e606;
    let _e607 = local_121;
    out[14i] = u32(_e607);
    local_21 = u32(1i);
    let _e614 = local_21;
    local_122 = u32(_e614);
    local_123 = u32();
    local_124 = u32();
    local_125 = i32();
    local_126 = i32();
    local_123 = u32(0i);
    local_124 = u32(1i);
    let _e632 = local_123;
    local_127 = u32(_e632);
    let _e636 = local_122;
    let _e639 = local_127;
    let _e641 = records[_e636].samples[_e639];
    local_128 = _e641;
    let _e642 = local_128;
    local_125 = _e642;
    let _e644 = local_125;
    let _e645 = local_124;
    local_129 = u32(_e645);
    let _e649 = local_122;
    let _e652 = local_129;
    let _e654 = records[_e649].samples[_e652];
    local_130 = _e654;
    let _e655 = local_130;
    local_126 = _e655;
    let _e657 = local_126;
    local_22 = (_e644 + _e657);
    let _e659 = local_22;
    out[15i] = u32(_e659);
    index = 0i;
    let _e665 = index;
    index = (_e665 + 1i);
    local_23 = u32(_e665);
    let _e670 = local_23;
    local_131 = u32(_e670);
    let _e674 = local_131;
    let _e678 = records[_e674].point.x;
    local_132 = _e678;
    let _e679 = local_132;
    let _e682 = local_131;
    records[_e682].point.x = (_e679 + 2i);
    local_133 = u32(0u);
    let _e690 = local_133;
    let _e694 = records[_e690].point.x;
    local_134 = _e694;
    let _e695 = local_134;
    out[16i] = u32(_e695);
    let _e699 = index;
    out[17i] = u32(_e699);
    let _e703 = index;
    index = (_e703 + 1i);
    local_24 = u32(_e703);
    local_25 = u32(0i);
    let _e711 = local_24;
    local_135 = u32(_e711);
    let _e715 = local_135;
    let _e718 = local_25;
    let _e720 = records[_e715].samples[_e718];
    local_136 = _e720;
    let _e721 = local_136;
    let _e724 = local_135;
    let _e727 = local_25;
    records[_e724].samples[_e727] = (_e721 + 1i);
    local_26 = _e721;
    let _e730 = local_26;
    out[18i] = u32(_e730);
    local_27 = u32(1i);
    local_28 = u32(0i);
    let _e740 = local_27;
    local_137 = u32(_e740);
    let _e744 = local_137;
    let _e747 = local_28;
    let _e749 = records[_e744].samples[_e747];
    local_138 = _e749;
    let _e750 = local_138;
    out[19i] = u32(_e750);
    let _e754 = index;
    out[20i] = u32(_e754);
    choose = true;
    let _e760 = choose;
    if _e760 {
        local_29 = u32(1i);
        local_31 = 1u;
    } else {
        local_30 = u32(2i);
        local_31 = 0u;
    }
    let _e774 = local_31;
    if (_e774 != 0u) {
        let _e777 = local_29;
        local_139 = u32(_e777);
        local_141 = 107u;
    } else {
        let _e780 = local_30;
        local_140 = u32(_e780);
        local_141 = 108u;
    }
    let _e787 = local_141;
    switch _e787 {
        case 107u: {
            let _e788 = local_139;
            let _e792 = records[_e788].nested.value;
            local_142 = _e792;
        }
        default: {
            let _e793 = local_140;
            let _e797 = records[_e793].nested.value;
            local_142 = _e797;
        }
    }
    let _e798 = local_142;
    let _e800 = (_e798 + 7i);
    let _e801 = local_141;
    switch _e801 {
        case 107u: {
            let _e802 = local_139;
            records[_e802].nested.value = _e800;
        }
        default: {
            let _e806 = local_140;
            records[_e806].nested.value = _e800;
        }
    }
    local_32 = u32(1i);
    let _e813 = local_32;
    local_143 = u32(_e813);
    let _e817 = local_143;
    let _e821 = records[_e817].nested.value;
    local_144 = _e821;
    let _e822 = local_144;
    out[21i] = u32(_e822);
    choose = false;
    let _e827 = choose;
    if _e827 {
        local_33 = u32(1i);
        local_35 = 3u;
    } else {
        local_34 = u32(2i);
        local_35 = 4u;
    }
    let _e839 = local_35;
    let _e842 = local_33;
    local_146 = u32(_e842);
    let _e846 = local_34;
    local_147 = u32(_e846);
    switch _e839 {
        case 3u: {
            local_145 = 116u;
        }
        default: {
            local_145 = 117u;
        }
    }
    let _e851 = local_146;
    local_148 = u32(_e851);
    let _e854 = local_147;
    local_149 = u32(_e854);
    let _e858 = local_145;
    switch _e858 {
        case 116u: {
            let _e859 = local_148;
            let _e863 = records[_e859].point.z;
            local_150 = _e863;
        }
        default: {
            let _e864 = local_149;
            let _e868 = records[_e864].point.z;
            local_150 = _e868;
        }
    }
    let _e869 = local_150;
    let _e871 = (_e869 - 2i);
    let _e872 = local_145;
    switch _e872 {
        case 116u: {
            let _e873 = local_148;
            records[_e873].point.z = _e871;
        }
        default: {
            let _e877 = local_149;
            records[_e877].point.z = _e871;
        }
    }
    local_36 = u32(2i);
    let _e884 = local_36;
    local_151 = u32(_e884);
    let _e888 = local_151;
    let _e892 = records[_e888].point.z;
    local_152 = _e892;
    let _e893 = local_152;
    out[22i] = u32(_e893);
    frozen.nested.value = 67i;
    frozen.point.x = 71i;
    frozen.point.y = 73i;
    frozen.point.z = 79i;
    frozen.samples[0] = 83i;
    frozen.samples[1] = 89i;
    frozen.samples[2] = 97i;
    let _e918 = frozen.point.z;
    local_153 = _e918;
    let _e919 = local_153;
    out[23i] = u32(_e919);
    pair.x = 101i;
    pair.y = 103i;
    let _e930 = pair.y;
    local_154 = _e930;
    let _e931 = local_154;
    out[24i] = u32(_e931);
    mutable_pair.x = 107i;
    mutable_pair.y = 109i;
    let _e942 = mutable_pair.x;
    local_155 = _e942;
    let _e943 = local_155;
    mutable_pair.x = (_e943 + 4i);
    let _e949 = mutable_pair.x;
    local_156 = _e949;
    let _e950 = local_156;
    out[25i] = u32(_e950);
    select_ = false;
    let _e956 = select_;
    if _e956 {
        local_37 = u32(0i);
        local_39 = 8u;
    } else {
        local_38 = u32(1i);
        local_39 = 9u;
    }
    let _e968 = local_39;
    let _e971 = local_37;
    local_158 = u32(_e971);
    let _e975 = local_38;
    local_159 = u32(_e975);
    switch _e968 {
        case 8u: {
            local_157 = 135u;
        }
        default: {
            local_157 = 137u;
        }
    }
    let _e981 = local_157;
    switch _e981 {
        case 135u: {
            let _e982 = local_158;
            let _e986 = records[_e982].nested.value;
            local_160 = _e986;
        }
        default: {
            let _e987 = local_159;
            let _e991 = records[_e987].nested.value;
            local_160 = _e991;
        }
    }
    let _e992 = local_160;
    let _e994 = (_e992 + 1i);
    let _e995 = local_157;
    switch _e995 {
        case 135u: {
            let _e996 = local_158;
            records[_e996].nested.value = _e994;
        }
        default: {
            let _e1000 = local_159;
            records[_e1000].nested.value = _e994;
        }
    }
    local_40 = _e992;
    local_41 = u32(1i);
    let _e1008 = local_41;
    local_161 = u32(_e1008);
    let _e1012 = local_161;
    let _e1016 = records[_e1012].nested.value;
    local_162 = _e1016;
    let _e1017 = local_162;
    out[26i] = u32(_e1017);
    local_42 = u32(2i);
    local_43 = u32(1i);
    let _e1027 = local_42;
    local_163 = u32(_e1027);
    let _e1031 = local_163;
    let _e1034 = local_43;
    let _e1036 = records[_e1031].samples[_e1034];
    local_164 = _e1036;
    let _e1037 = local_164;
    let _e1040 = local_163;
    let _e1043 = local_43;
    records[_e1040].samples[_e1043] = (_e1037 + 3i);
    local_44 = u32(2i);
    local_45 = u32(1i);
    let _e1051 = local_44;
    local_165 = u32(_e1051);
    let _e1055 = local_165;
    let _e1058 = local_45;
    let _e1060 = records[_e1055].samples[_e1058];
    local_166 = _e1060;
    let _e1061 = local_166;
    out[27i] = u32(_e1061);
    let _e1069 = records[0i].point.x;
    out[28i] = u32(_e1069);
    let _e1077 = records[1i].point.x;
    out[29i] = u32(_e1077);
    let _e1085 = records[2i].nested.value;
    out[30i] = u32(_e1085);
    let _e1089 = index;
    out[31i] = u32(_e1089);
}
