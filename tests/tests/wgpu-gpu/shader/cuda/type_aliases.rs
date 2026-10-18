use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::type_aliases::cuda_alias_casts_use_all_existing_scalar_cast_forms/1",
            r#"typedef int Whole;
        using Frozen = const Whole;
        __global__ void kernel(int *out) {
            Whole a = (Whole)3.5f;
            Whole b = static_cast<Whole>(4.5f);
            Whole c = Whole(5.5f);
            Whole d = Frozen(6.5f);
            out[0] = a + b + c + d;
        }"#,
            &[0],
            &[18],
        ),
        case(
            "cuda::type_aliases::cuda_alias_chains_resolve_without_recursive_type_expansion/1",
            r#"typedef int Element0;
using Element1 = Element0;
using Element2 = Element1;
using Element3 = Element2;
using Element4 = Element3;
using Element5 = Element4;
using Element6 = Element5;
using Element7 = Element6;
using Element8 = Element7;
using Element9 = Element8;
using Element10 = Element9;
using Element11 = Element10;
using Element12 = Element11;
using Element13 = Element12;
using Element14 = Element13;
using Element15 = Element14;
using Element16 = Element15;
using Element17 = Element16;
using Element18 = Element17;
using Element19 = Element18;
using Element20 = Element19;
using Element21 = Element20;
using Element22 = Element21;
using Element23 = Element22;
using Element24 = Element23;
using Element25 = Element24;
using Element26 = Element25;
using Element27 = Element26;
using Element28 = Element27;
using Element29 = Element28;
using Element30 = Element29;
using Element31 = Element30;
using Element32 = Element31;
using Element33 = Element32;
using Element34 = Element33;
using Element35 = Element34;
using Element36 = Element35;
using Element37 = Element36;
using Element38 = Element37;
using Element39 = Element38;
using Element40 = Element39;
using Element41 = Element40;
using Element42 = Element41;
using Element43 = Element42;
using Element44 = Element43;
using Element45 = Element44;
using Element46 = Element45;
using Element47 = Element46;
using Element48 = Element47;
using Element49 = Element48;
using Element50 = Element49;
using Element51 = Element50;
using Element52 = Element51;
using Element53 = Element52;
using Element54 = Element53;
using Element55 = Element54;
using Element56 = Element55;
using Element57 = Element56;
using Element58 = Element57;
using Element59 = Element58;
using Element60 = Element59;
using Element61 = Element60;
using Element62 = Element61;
using Element63 = Element62;
using Element64 = Element63;
using Element65 = Element64;
using Element66 = Element65;
using Element67 = Element66;
using Element68 = Element67;
using Element69 = Element68;
using Element70 = Element69;
using Element71 = Element70;
using Element72 = Element71;
using Element73 = Element72;
using Element74 = Element73;
using Element75 = Element74;
using Element76 = Element75;
using Element77 = Element76;
using Element78 = Element77;
using Element79 = Element78;
using Element80 = Element79;
using Element81 = Element80;
using Element82 = Element81;
using Element83 = Element82;
using Element84 = Element83;
using Element85 = Element84;
using Element86 = Element85;
using Element87 = Element86;
using Element88 = Element87;
using Element89 = Element88;
using Element90 = Element89;
using Element91 = Element90;
using Element92 = Element91;
using Element93 = Element92;
using Element94 = Element93;
using Element95 = Element94;
using Element96 = Element95;
using Element97 = Element96;
using Element98 = Element97;
using Element99 = Element98;
using Element100 = Element99;
using Element101 = Element100;
using Element102 = Element101;
using Element103 = Element102;
using Element104 = Element103;
using Element105 = Element104;
using Element106 = Element105;
using Element107 = Element106;
using Element108 = Element107;
using Element109 = Element108;
using Element110 = Element109;
using Element111 = Element110;
using Element112 = Element111;
using Element113 = Element112;
using Element114 = Element113;
using Element115 = Element114;
using Element116 = Element115;
using Element117 = Element116;
using Element118 = Element117;
using Element119 = Element118;
using Element120 = Element119;
using Element121 = Element120;
using Element122 = Element121;
using Element123 = Element122;
using Element124 = Element123;
using Element125 = Element124;
using Element126 = Element125;
using Element127 = Element126;
using Element128 = Element127;
using Element129 = Element128;
using Element130 = Element129;
using Element131 = Element130;
using Element132 = Element131;
using Element133 = Element132;
using Element134 = Element133;
using Element135 = Element134;
using Element136 = Element135;
using Element137 = Element136;
using Element138 = Element137;
using Element139 = Element138;
using Element140 = Element139;
using Element141 = Element140;
using Element142 = Element141;
using Element143 = Element142;
using Element144 = Element143;
using Element145 = Element144;
using Element146 = Element145;
using Element147 = Element146;
using Element148 = Element147;
using Element149 = Element148;
using Element150 = Element149;
using Element151 = Element150;
using Element152 = Element151;
using Element153 = Element152;
using Element154 = Element153;
using Element155 = Element154;
using Element156 = Element155;
using Element157 = Element156;
using Element158 = Element157;
using Element159 = Element158;
using Element160 = Element159;
using Element161 = Element160;
using Element162 = Element161;
using Element163 = Element162;
using Element164 = Element163;
using Element165 = Element164;
using Element166 = Element165;
using Element167 = Element166;
using Element168 = Element167;
using Element169 = Element168;
using Element170 = Element169;
using Element171 = Element170;
using Element172 = Element171;
using Element173 = Element172;
using Element174 = Element173;
using Element175 = Element174;
using Element176 = Element175;
using Element177 = Element176;
using Element178 = Element177;
using Element179 = Element178;
using Element180 = Element179;
using Element181 = Element180;
using Element182 = Element181;
using Element183 = Element182;
using Element184 = Element183;
using Element185 = Element184;
using Element186 = Element185;
using Element187 = Element186;
using Element188 = Element187;
using Element189 = Element188;
using Element190 = Element189;
using Element191 = Element190;
using Element192 = Element191;
using Element193 = Element192;
using Element194 = Element193;
using Element195 = Element194;
using Element196 = Element195;
using Element197 = Element196;
using Element198 = Element197;
using Element199 = Element198;
using Element200 = Element199;
using Element201 = Element200;
using Element202 = Element201;
using Element203 = Element202;
using Element204 = Element203;
using Element205 = Element204;
using Element206 = Element205;
using Element207 = Element206;
using Element208 = Element207;
using Element209 = Element208;
using Element210 = Element209;
using Element211 = Element210;
using Element212 = Element211;
using Element213 = Element212;
using Element214 = Element213;
using Element215 = Element214;
using Element216 = Element215;
using Element217 = Element216;
using Element218 = Element217;
using Element219 = Element218;
using Element220 = Element219;
using Element221 = Element220;
using Element222 = Element221;
using Element223 = Element222;
using Element224 = Element223;
using Element225 = Element224;
using Element226 = Element225;
using Element227 = Element226;
using Element228 = Element227;
using Element229 = Element228;
using Element230 = Element229;
using Element231 = Element230;
using Element232 = Element231;
using Element233 = Element232;
using Element234 = Element233;
using Element235 = Element234;
using Element236 = Element235;
using Element237 = Element236;
using Element238 = Element237;
using Element239 = Element238;
using Element240 = Element239;
using Element241 = Element240;
using Element242 = Element241;
using Element243 = Element242;
using Element244 = Element243;
using Element245 = Element244;
using Element246 = Element245;
using Element247 = Element246;
using Element248 = Element247;
using Element249 = Element248;
using Element250 = Element249;
using Element251 = Element250;
using Element252 = Element251;
using Element253 = Element252;
using Element254 = Element253;
using Element255 = Element254;
__global__ void kernel(int *out) { Element255 value = 7; out[0] = value; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::type_aliases::cuda_alias_declaration_locus_preserves_outer_type_lookup/1",
            r#"using Element = int;
            __global__ void kernel(int *out) {
                { typedef Element Element; Element value = 4; out[0] = value; }
            }"#,
            &[0],
            &[4],
        ),
        case(
            "cuda::type_aliases::cuda_alias_declaration_locus_preserves_outer_type_lookup/2",
            r#"using Element = int;
            __global__ void kernel(int *out) {
                { using Element = Element; Element value = 4; out[0] = value; }
            }"#,
            &[0],
            &[4],
        ),
        case(
            "cuda::type_aliases::cuda_alias_expression_contexts_keep_functional_casts_unambiguous/1",
            r#"using Whole = int;
        __global__ void kernel(int *out) {
            int value = 3;
            Whole(4); Whole(value + 1);
            out[0] = Whole(value); out[1] = Whole((value));
        }"#,
            &[0, 0],
            &[3, 3],
        ),
        case(
            "cuda::type_aliases::cuda_alias_redeclarations_accept_the_same_canonical_type/1",
            r#"typedef int Element;
        using Element = int;
        typedef Element Element;
        using Frozen = const Element;
        typedef const Frozen Frozen;
        __global__ void kernel(int *out) {
            typedef unsigned Count;
            using Count = unsigned int;
            Element value = 3; Count amount = 4u;
            out[0] = value + amount;
        }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::type_aliases::cuda_block_aliases_can_hide_values_and_restore_the_original_value/1",
            r#"__global__ void kernel(int *out) {
        int Quantity = 7;
        { typedef int Quantity; Quantity inner = 3; out[0] = inner; }
        out[1] = Quantity;
    }"#,
            &[0, 0],
            &[3, 7],
        ),
        case(
            "cuda::type_aliases::cuda_block_aliases_shadow_outer_types_and_restore_them_after_exit/1",
            r#"using Quantity = int;
        __global__ void kernel(int *out) {
            Quantity outer = 2;
            { using Quantity = float; Quantity inner = 1.5f; out[0] = inner * 2.0f; }
            Quantity after = 5; out[1] = outer + after;
        }"#,
            &[0, 0],
            &[3, 7],
        ),
        case(
            "cuda::type_aliases::cuda_inherited_const_is_preserved_and_redundant_qualification_is_ignored/1",
            r#"typedef const int Frozen;
        using Fixed = Frozen;
        typedef const Fixed Again;
        __global__ void kernel(int *out) {
            const Fixed a = 2; Fixed const b = 3; Again c = 4;
            out[0] = a + b + c;
        }"#,
            &[0],
            &[9],
        ),
        case(
            "cuda::type_aliases::cuda_local_aliases_hide_helpers_in_functional_cast_syntax/1",
            r#"__device__ int Measure(int value) { return value + 100; }
        __global__ void kernel(int *out) { using Measure = int; out[0] = Measure(3.5f); }"#,
            &[0],
            &[3],
        ),
        case(
            "cuda::type_aliases::cuda_local_values_can_hide_global_aliases/1",
            r#"typedef int Quantity;
        __global__ void kernel(int *out) {
            Quantity Quantity = 3; Quantity += 2; out[0] = Quantity;
        }"#,
            &[0],
            &[5],
        ),
        case(
            "cuda::type_aliases::cuda_scalar_alias_chains_and_multiple_declarators_keep_the_same_type/1",
            r#"typedef int Whole, Distance, Position;
        using Coordinate = Position;
        typedef Coordinate Index;
        __global__ void kernel(int *out) {
            Whole a = 3; Distance b = 4; Index c = 5; out[0] = a + b + c;
        }"#,
            &[0],
            &[12],
        ),
    ]
}
