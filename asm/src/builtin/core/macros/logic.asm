; #[macro] nand: {
;     ($inl: reg, $inh: reg, $frl: lit | reg, $frh: lit | reg) => {
;         nand $inl, $frl
;         nand $inh, $frh
;     }
; }
;
; #[macro] not: {
;     ($inl: reg, $inh: reg) => {
;         not $inl
;         not $inh
;     }
; }
;
; #[macro] xnor: {
;     ($inl: reg, $inh: reg, $frl: lit | reg, $frh: lit | reg) => {
;         xnor $inl, $frl
;         xnor $inh, $frh
;     }
; }
;
;
; #[macro] xor: {
;     ($inl: reg, $inh: reg, $frl: lit | reg, $frh: lit | reg) => {
;         xor $inl, $frl
;         xor $inh, $frh
;     }
; }
