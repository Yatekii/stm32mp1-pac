#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_gicd: [u8; 4usize],
    #[doc = "0x04 - GICD interrupt controller type register"]
    pub gicd_typer: GICD_TYPER,
    #[doc = "0x08 - GICD implementer identification register"]
    pub gicd_iidr: GICD_IIDR,
    _reserved3: [u8; 116usize],
    #[doc = "0x80 - GICD interrupt group registers"]
    pub gicd_igroupr0: GICD_IGROUPR0,
    #[doc = "0x84 - GICD interrupt group registers"]
    pub gicd_igroupr1: GICD_IGROUPR1,
    #[doc = "0x88 - GICD interrupt group registers"]
    pub gicd_igroupr2: GICD_IGROUPR2,
    #[doc = "0x8c - GICD interrupt group registers"]
    pub gicd_igroupr3: GICD_IGROUPR3,
    #[doc = "0x90 - GICD interrupt group registers"]
    pub gicd_igroupr4: GICD_IGROUPR4,
    #[doc = "0x94 - GICD interrupt group registers"]
    pub gicd_igroupr5: GICD_IGROUPR5,
    #[doc = "0x98 - GICD interrupt group registers"]
    pub gicd_igroupr6: GICD_IGROUPR6,
    #[doc = "0x9c - GICD interrupt group registers"]
    pub gicd_igroupr7: GICD_IGROUPR7,
    #[doc = "0xa0 - GICD interrupt group registers"]
    pub gicd_igroupr8: GICD_IGROUPR8,
    _reserved12: [u8; 92usize],
    #[doc = "0x100 - GICD interrupt set-enable register"]
    pub isenabler0: ISENABLER0,
    #[doc = "0x104 - GICD interrupt set-enable register"]
    pub isenabler1: ISENABLER1,
    #[doc = "0x108 - GICD interrupt set-enable register"]
    pub isenabler2: ISENABLER2,
    #[doc = "0x10c - GICD interrupt set-enable register"]
    pub isenabler3: ISENABLER3,
    #[doc = "0x110 - GICD interrupt set-enable register"]
    pub isenabler4: ISENABLER4,
    #[doc = "0x114 - GICD interrupt set-enable register"]
    pub isenabler5: ISENABLER5,
    #[doc = "0x118 - GICD interrupt set-enable register"]
    pub isenabler6: ISENABLER6,
    #[doc = "0x11c - GICD interrupt set-enable register"]
    pub isenabler7: ISENABLER7,
    #[doc = "0x120 - GICD interrupt set-enable register"]
    pub isenabler8: ISENABLER8,
    _reserved21: [u8; 8usize],
    #[doc = "0x12c - GICD interrupt set-active registers"]
    pub isactiver0: ISACTIVER0,
    _reserved22: [u8; 80usize],
    #[doc = "0x180 - GICD interrupt clear-enable register"]
    pub icenabler0: ICENABLER0,
    #[doc = "0x184 - GICD interrupt clear-enable register"]
    pub icenabler1: ICENABLER1,
    #[doc = "0x188 - GICD interrupt clear-enable register"]
    pub icenabler2: ICENABLER2,
    #[doc = "0x18c - GICD interrupt clear-enable register"]
    pub icenabler3: ICENABLER3,
    #[doc = "0x190 - GICD interrupt clear-enable register"]
    pub icenabler4: ICENABLER4,
    #[doc = "0x194 - GICD interrupt clear-enable register"]
    pub icenabler5: ICENABLER5,
    #[doc = "0x198 - GICD interrupt clear-enable register"]
    pub icenabler6: ICENABLER6,
    #[doc = "0x19c - GICD interrupt clear-enable register"]
    pub icenabler7: ICENABLER7,
    #[doc = "0x1a0 - GICD interrupt clear-enable register"]
    pub icenabler8: ICENABLER8,
    _reserved31: [u8; 92usize],
    #[doc = "0x200 - GICD interrupt set-pending registers"]
    pub ispendr0: ISPENDR0,
    #[doc = "0x204 - GICD interrupt set-pending registers"]
    pub ispendr1: ISPENDR1,
    #[doc = "0x208 - GICD interrupt set-pending registers"]
    pub ispendr2: ISPENDR2,
    #[doc = "0x20c - GICD interrupt set-pending registers"]
    pub ispendr3: ISPENDR3,
    #[doc = "0x210 - GICD interrupt set-pending registers"]
    pub ispendr4: ISPENDR4,
    #[doc = "0x214 - GICD interrupt set-pending registers"]
    pub ispendr5: ISPENDR5,
    #[doc = "0x218 - GICD interrupt set-pending registers"]
    pub ispendr6: ISPENDR6,
    #[doc = "0x21c - GICD interrupt set-pending registers"]
    pub ispendr7: ISPENDR7,
    #[doc = "0x220 - GICD interrupt set-pending registers"]
    pub ispendr8: ISPENDR8,
    _reserved40: [u8; 92usize],
    #[doc = "0x280 - GICD interrupt clear-pending registers"]
    pub icpendr0: ICPENDR0,
    #[doc = "0x284 - GICD interrupt clear-pending registers"]
    pub icpendr1: ICPENDR1,
    #[doc = "0x288 - GICD interrupt clear-pending registers"]
    pub icpendr2: ICPENDR2,
    #[doc = "0x28c - GICD interrupt clear-pending registers"]
    pub icpendr3: ICPENDR3,
    #[doc = "0x290 - GICD interrupt clear-pending registers"]
    pub icpendr4: ICPENDR4,
    #[doc = "0x294 - GICD interrupt clear-pending registers"]
    pub icpendr5: ICPENDR5,
    #[doc = "0x298 - GICD interrupt clear-pending registers"]
    pub icpendr6: ICPENDR6,
    #[doc = "0x29c - GICD interrupt clear-pending registers"]
    pub icpendr7: ICPENDR7,
    #[doc = "0x2a0 - GICD interrupt clear-pending registers"]
    pub icpendr8: ICPENDR8,
    _reserved49: [u8; 96usize],
    #[doc = "0x304 - GICD interrupt set-active registers"]
    pub isactiver1: ISACTIVER1,
    #[doc = "0x308 - GICD interrupt set-active registers"]
    pub isactiver2: ISACTIVER2,
    #[doc = "0x30c - GICD interrupt set-active registers"]
    pub isactiver3: ISACTIVER3,
    #[doc = "0x310 - GICD interrupt set-active registers"]
    pub isactiver4: ISACTIVER4,
    #[doc = "0x314 - GICD interrupt set-active registers"]
    pub isactiver5: ISACTIVER5,
    #[doc = "0x318 - GICD interrupt set-active registers"]
    pub isactiver6: ISACTIVER6,
    #[doc = "0x31c - GICD interrupt set-active registers"]
    pub isactiver7: ISACTIVER7,
    #[doc = "0x320 - GICD interrupt set-active registers"]
    pub isactiver8: ISACTIVER8,
    _reserved57: [u8; 92usize],
    #[doc = "0x380 - GICD interrupt clear-active registers"]
    pub icactiver0: ICACTIVER0,
    #[doc = "0x384 - GICD interrupt clear-active registers"]
    pub icactiver1: ICACTIVER1,
    #[doc = "0x388 - GICD interrupt clear-active registers"]
    pub icactiver2: ICACTIVER2,
    #[doc = "0x38c - GICD interrupt clear-active registers"]
    pub icactiver3: ICACTIVER3,
    #[doc = "0x390 - GICD interrupt clear-active registers"]
    pub icactiver4: ICACTIVER4,
    #[doc = "0x394 - GICD interrupt clear-active registers"]
    pub icactiver5: ICACTIVER5,
    #[doc = "0x398 - GICD interrupt clear-active registers"]
    pub icactiver6: ICACTIVER6,
    #[doc = "0x39c - GICD interrupt clear-active registers"]
    pub icactiver7: ICACTIVER7,
    #[doc = "0x3a0 - GICD interrupt clear-active registers"]
    pub icactiver8: ICACTIVER8,
    _reserved66: [u8; 92usize],
    #[doc = "0x400 - GICD interrupt priority registers"]
    pub ipriorityr0: IPRIORITYR0,
    #[doc = "0x404 - GICD interrupt priority registers"]
    pub ipriorityr1: IPRIORITYR1,
    #[doc = "0x408 - GICD interrupt priority registers"]
    pub ipriorityr2: IPRIORITYR2,
    #[doc = "0x40c - GICD interrupt priority registers"]
    pub ipriorityr3: IPRIORITYR3,
    #[doc = "0x410 - GICD interrupt priority registers"]
    pub ipriorityr4: IPRIORITYR4,
    #[doc = "0x414 - GICD interrupt priority registers"]
    pub ipriorityr5: IPRIORITYR5,
    #[doc = "0x418 - GICD interrupt priority registers"]
    pub ipriorityr6: IPRIORITYR6,
    #[doc = "0x41c - GICD interrupt priority registers"]
    pub ipriorityr7: IPRIORITYR7,
    #[doc = "0x420 - GICD interrupt priority registers"]
    pub ipriorityr8: IPRIORITYR8,
    #[doc = "0x424 - GICD interrupt priority registers"]
    pub ipriorityr9: IPRIORITYR9,
    #[doc = "0x428 - GICD interrupt priority registers"]
    pub ipriorityr10: IPRIORITYR10,
    #[doc = "0x42c - GICD interrupt priority registers"]
    pub ipriorityr11: IPRIORITYR11,
    #[doc = "0x430 - GICD interrupt priority registers"]
    pub ipriorityr12: IPRIORITYR12,
    #[doc = "0x434 - GICD interrupt priority registers"]
    pub ipriorityr13: IPRIORITYR13,
    #[doc = "0x438 - GICD interrupt priority registers"]
    pub ipriorityr14: IPRIORITYR14,
    #[doc = "0x43c - GICD interrupt priority registers"]
    pub ipriorityr15: IPRIORITYR15,
    #[doc = "0x440 - GICD interrupt priority registers"]
    pub ipriorityr16: IPRIORITYR16,
    #[doc = "0x444 - GICD interrupt priority registers"]
    pub ipriorityr17: IPRIORITYR17,
    #[doc = "0x448 - GICD interrupt priority registers"]
    pub ipriorityr18: IPRIORITYR18,
    #[doc = "0x44c - GICD interrupt priority registers"]
    pub ipriorityr19: IPRIORITYR19,
    #[doc = "0x450 - GICD interrupt priority registers"]
    pub ipriorityr20: IPRIORITYR20,
    #[doc = "0x454 - GICD interrupt priority registers"]
    pub ipriorityr21: IPRIORITYR21,
    #[doc = "0x458 - GICD interrupt priority registers"]
    pub ipriorityr22: IPRIORITYR22,
    #[doc = "0x45c - GICD interrupt priority registers"]
    pub ipriorityr23: IPRIORITYR23,
    #[doc = "0x460 - GICD interrupt priority registers"]
    pub ipriorityr24: IPRIORITYR24,
    #[doc = "0x464 - GICD interrupt priority registers"]
    pub ipriorityr25: IPRIORITYR25,
    #[doc = "0x468 - GICD interrupt priority registers"]
    pub ipriorityr26: IPRIORITYR26,
    #[doc = "0x46c - GICD interrupt priority registers"]
    pub ipriorityr27: IPRIORITYR27,
    #[doc = "0x470 - GICD interrupt priority registers"]
    pub ipriorityr28: IPRIORITYR28,
    #[doc = "0x474 - GICD interrupt priority registers"]
    pub ipriorityr29: IPRIORITYR29,
    #[doc = "0x478 - GICD interrupt priority registers"]
    pub ipriorityr30: IPRIORITYR30,
    #[doc = "0x47c - GICD interrupt priority registers"]
    pub ipriorityr31: IPRIORITYR31,
    #[doc = "0x480 - GICD interrupt priority registers"]
    pub ipriorityr32: IPRIORITYR32,
    #[doc = "0x484 - GICD interrupt priority registers"]
    pub ipriorityr33: IPRIORITYR33,
    #[doc = "0x488 - GICD interrupt priority registers"]
    pub ipriorityr34: IPRIORITYR34,
    #[doc = "0x48c - GICD interrupt priority registers"]
    pub ipriorityr35: IPRIORITYR35,
    #[doc = "0x490 - GICD interrupt priority registers"]
    pub ipriorityr36: IPRIORITYR36,
    #[doc = "0x494 - GICD interrupt priority registers"]
    pub ipriorityr37: IPRIORITYR37,
    #[doc = "0x498 - GICD interrupt priority registers"]
    pub ipriorityr38: IPRIORITYR38,
    #[doc = "0x49c - GICD interrupt priority registers"]
    pub ipriorityr39: IPRIORITYR39,
    #[doc = "0x4a0 - GICD interrupt priority registers"]
    pub ipriorityr40: IPRIORITYR40,
    #[doc = "0x4a4 - GICD interrupt priority registers"]
    pub ipriorityr41: IPRIORITYR41,
    #[doc = "0x4a8 - GICD interrupt priority registers"]
    pub ipriorityr42: IPRIORITYR42,
    #[doc = "0x4ac - GICD interrupt priority registers"]
    pub ipriorityr43: IPRIORITYR43,
    #[doc = "0x4b0 - GICD interrupt priority registers"]
    pub ipriorityr44: IPRIORITYR44,
    #[doc = "0x4b4 - GICD interrupt priority registers"]
    pub ipriorityr45: IPRIORITYR45,
    #[doc = "0x4b8 - GICD interrupt priority registers"]
    pub ipriorityr46: IPRIORITYR46,
    #[doc = "0x4bc - GICD interrupt priority registers"]
    pub ipriorityr47: IPRIORITYR47,
    #[doc = "0x4c0 - GICD interrupt priority registers"]
    pub ipriorityr48: IPRIORITYR48,
    #[doc = "0x4c4 - GICD interrupt priority registers"]
    pub ipriorityr49: IPRIORITYR49,
    #[doc = "0x4c8 - GICD interrupt priority registers"]
    pub ipriorityr50: IPRIORITYR50,
    #[doc = "0x4cc - GICD interrupt priority registers"]
    pub ipriorityr51: IPRIORITYR51,
    #[doc = "0x4d0 - GICD interrupt priority registers"]
    pub ipriorityr52: IPRIORITYR52,
    #[doc = "0x4d4 - GICD interrupt priority registers"]
    pub ipriorityr53: IPRIORITYR53,
    #[doc = "0x4d8 - GICD interrupt priority registers"]
    pub ipriorityr54: IPRIORITYR54,
    #[doc = "0x4dc - GICD interrupt priority registers"]
    pub ipriorityr55: IPRIORITYR55,
    #[doc = "0x4e0 - GICD interrupt priority registers"]
    pub ipriorityr56: IPRIORITYR56,
    #[doc = "0x4e4 - GICD interrupt priority registers"]
    pub ipriorityr57: IPRIORITYR57,
    #[doc = "0x4e8 - GICD interrupt priority registers"]
    pub ipriorityr58: IPRIORITYR58,
    #[doc = "0x4ec - GICD interrupt priority registers"]
    pub ipriorityr59: IPRIORITYR59,
    #[doc = "0x4f0 - GICD interrupt priority registers"]
    pub ipriorityr60: IPRIORITYR60,
    #[doc = "0x4f4 - GICD interrupt priority registers"]
    pub ipriorityr61: IPRIORITYR61,
    #[doc = "0x4f8 - GICD interrupt priority registers"]
    pub ipriorityr62: IPRIORITYR62,
    #[doc = "0x4fc - GICD interrupt priority registers"]
    pub ipriorityr63: IPRIORITYR63,
    #[doc = "0x500 - GICD interrupt priority registers"]
    pub ipriorityr64: IPRIORITYR64,
    #[doc = "0x504 - GICD interrupt priority registers"]
    pub ipriorityr65: IPRIORITYR65,
    #[doc = "0x508 - GICD interrupt priority registers"]
    pub ipriorityr66: IPRIORITYR66,
    #[doc = "0x50c - GICD interrupt priority registers"]
    pub ipriorityr67: IPRIORITYR67,
    #[doc = "0x510 - GICD interrupt priority registers"]
    pub ipriorityr68: IPRIORITYR68,
    #[doc = "0x514 - GICD interrupt priority registers"]
    pub ipriorityr69: IPRIORITYR69,
    #[doc = "0x518 - GICD interrupt priority registers"]
    pub ipriorityr70: IPRIORITYR70,
    #[doc = "0x51c - GICD interrupt priority registers"]
    pub ipriorityr71: IPRIORITYR71,
    _reserved138: [u8; 736usize],
    #[doc = "0x800 - GICD interrupt processor target registers"]
    pub itargetsr0: ITARGETSR0,
    #[doc = "0x804 - GICD interrupt processor target registers"]
    pub itargetsr1: ITARGETSR1,
    #[doc = "0x808 - GICD interrupt processor target registers"]
    pub itargetsr2: ITARGETSR2,
    #[doc = "0x80c - GICD interrupt processor target registers"]
    pub itargetsr3: ITARGETSR3,
    #[doc = "0x810 - GICD interrupt processor target registers"]
    pub itargetsr4: ITARGETSR4,
    #[doc = "0x814 - GICD interrupt processor target registers"]
    pub itargetsr5: ITARGETSR5,
    #[doc = "0x818 - GICD interrupt processor target registers"]
    pub itargetsr6: ITARGETSR6,
    #[doc = "0x81c - GICD interrupt processor target registers"]
    pub itargetsr7: ITARGETSR7,
    #[doc = "0x820 - GICD interrupt processor target registers"]
    pub itargetsr8: ITARGETSR8,
    #[doc = "0x824 - GICD interrupt processor target registers"]
    pub itargetsr9: ITARGETSR9,
    #[doc = "0x828 - GICD interrupt processor target registers"]
    pub itargetsr10: ITARGETSR10,
    #[doc = "0x82c - GICD interrupt processor target registers"]
    pub itargetsr11: ITARGETSR11,
    #[doc = "0x830 - GICD interrupt processor target registers"]
    pub itargetsr12: ITARGETSR12,
    #[doc = "0x834 - GICD interrupt processor target registers"]
    pub itargetsr13: ITARGETSR13,
    #[doc = "0x838 - GICD interrupt processor target registers"]
    pub itargetsr14: ITARGETSR14,
    #[doc = "0x83c - GICD interrupt processor target registers"]
    pub itargetsr15: ITARGETSR15,
    #[doc = "0x840 - GICD interrupt processor target registers"]
    pub itargetsr16: ITARGETSR16,
    #[doc = "0x844 - GICD interrupt processor target registers"]
    pub itargetsr17: ITARGETSR17,
    #[doc = "0x848 - GICD interrupt processor target registers"]
    pub itargetsr18: ITARGETSR18,
    #[doc = "0x84c - GICD interrupt processor target registers"]
    pub itargetsr19: ITARGETSR19,
    #[doc = "0x850 - GICD interrupt processor target registers"]
    pub itargetsr20: ITARGETSR20,
    #[doc = "0x854 - GICD interrupt processor target registers"]
    pub itargetsr21: ITARGETSR21,
    #[doc = "0x858 - GICD interrupt processor target registers"]
    pub itargetsr22: ITARGETSR22,
    #[doc = "0x85c - GICD interrupt processor target registers"]
    pub itargetsr23: ITARGETSR23,
    #[doc = "0x860 - GICD interrupt processor target registers"]
    pub itargetsr24: ITARGETSR24,
    #[doc = "0x864 - GICD interrupt processor target registers"]
    pub itargetsr25: ITARGETSR25,
    #[doc = "0x868 - GICD interrupt processor target registers"]
    pub itargetsr26: ITARGETSR26,
    #[doc = "0x86c - GICD interrupt processor target registers"]
    pub itargetsr27: ITARGETSR27,
    #[doc = "0x870 - GICD interrupt processor target registers"]
    pub itargetsr28: ITARGETSR28,
    #[doc = "0x874 - GICD interrupt processor target registers"]
    pub itargetsr29: ITARGETSR29,
    #[doc = "0x878 - GICD interrupt processor target registers"]
    pub itargetsr30: ITARGETSR30,
    #[doc = "0x87c - GICD interrupt processor target registers"]
    pub itargetsr31: ITARGETSR31,
    #[doc = "0x880 - GICD interrupt processor target registers"]
    pub itargetsr32: ITARGETSR32,
    #[doc = "0x884 - GICD interrupt processor target registers"]
    pub itargetsr33: ITARGETSR33,
    #[doc = "0x888 - GICD interrupt processor target registers"]
    pub itargetsr34: ITARGETSR34,
    #[doc = "0x88c - GICD interrupt processor target registers"]
    pub itargetsr35: ITARGETSR35,
    #[doc = "0x890 - GICD interrupt processor target registers"]
    pub itargetsr36: ITARGETSR36,
    #[doc = "0x894 - GICD interrupt processor target registers"]
    pub itargetsr37: ITARGETSR37,
    #[doc = "0x898 - GICD interrupt processor target registers"]
    pub itargetsr38: ITARGETSR38,
    #[doc = "0x89c - GICD interrupt processor target registers"]
    pub itargetsr39: ITARGETSR39,
    #[doc = "0x8a0 - GICD interrupt processor target registers"]
    pub itargetsr40: ITARGETSR40,
    #[doc = "0x8a4 - GICD interrupt processor target registers"]
    pub itargetsr41: ITARGETSR41,
    #[doc = "0x8a8 - GICD interrupt processor target registers"]
    pub itargetsr42: ITARGETSR42,
    #[doc = "0x8ac - GICD interrupt processor target registers"]
    pub itargetsr43: ITARGETSR43,
    #[doc = "0x8b0 - GICD interrupt processor target registers"]
    pub itargetsr44: ITARGETSR44,
    #[doc = "0x8b4 - GICD interrupt processor target registers"]
    pub itargetsr45: ITARGETSR45,
    #[doc = "0x8b8 - GICD interrupt processor target registers"]
    pub itargetsr46: ITARGETSR46,
    #[doc = "0x8bc - GICD interrupt processor target registers"]
    pub itargetsr47: ITARGETSR47,
    #[doc = "0x8c0 - GICD interrupt processor target registers"]
    pub itargetsr48: ITARGETSR48,
    #[doc = "0x8c4 - GICD interrupt processor target registers"]
    pub itargetsr49: ITARGETSR49,
    #[doc = "0x8c8 - GICD interrupt processor target registers"]
    pub itargetsr50: ITARGETSR50,
    #[doc = "0x8cc - GICD interrupt processor target registers"]
    pub itargetsr51: ITARGETSR51,
    #[doc = "0x8d0 - GICD interrupt processor target registers"]
    pub itargetsr52: ITARGETSR52,
    #[doc = "0x8d4 - GICD interrupt processor target registers"]
    pub itargetsr53: ITARGETSR53,
    #[doc = "0x8d8 - GICD interrupt processor target registers"]
    pub itargetsr54: ITARGETSR54,
    #[doc = "0x8dc - GICD interrupt processor target registers"]
    pub itargetsr55: ITARGETSR55,
    #[doc = "0x8e0 - GICD interrupt processor target registers"]
    pub itargetsr56: ITARGETSR56,
    #[doc = "0x8e4 - GICD interrupt processor target registers"]
    pub itargetsr57: ITARGETSR57,
    #[doc = "0x8e8 - GICD interrupt processor target registers"]
    pub itargetsr58: ITARGETSR58,
    #[doc = "0x8ec - GICD interrupt processor target registers"]
    pub itargetsr59: ITARGETSR59,
    #[doc = "0x8f0 - GICD interrupt processor target registers"]
    pub itargetsr60: ITARGETSR60,
    #[doc = "0x8f4 - GICD interrupt processor target registers"]
    pub itargetsr61: ITARGETSR61,
    #[doc = "0x8f8 - GICD interrupt processor target registers"]
    pub itargetsr62: ITARGETSR62,
    #[doc = "0x8fc - GICD interrupt processor target registers"]
    pub itargetsr63: ITARGETSR63,
    #[doc = "0x900 - GICD interrupt processor target registers"]
    pub itargetsr64: ITARGETSR64,
    #[doc = "0x904 - GICD interrupt processor target registers"]
    pub itargetsr65: ITARGETSR65,
    #[doc = "0x908 - GICD interrupt processor target registers"]
    pub itargetsr66: ITARGETSR66,
    #[doc = "0x90c - GICD interrupt processor target registers"]
    pub itargetsr67: ITARGETSR67,
    #[doc = "0x910 - GICD interrupt processor target registers"]
    pub itargetsr68: ITARGETSR68,
    #[doc = "0x914 - GICD interrupt processor target registers"]
    pub itargetsr69: ITARGETSR69,
    #[doc = "0x918 - GICD interrupt processor target registers"]
    pub itargetsr70: ITARGETSR70,
    #[doc = "0x91c - GICD interrupt processor target registers"]
    pub itargetsr71: ITARGETSR71,
    _reserved210: [u8; 736usize],
    #[doc = "0xc00 - GICD interrupt configuration register"]
    pub icfgr0: ICFGR0,
    #[doc = "0xc04 - GICD interrupt configuration register"]
    pub icfgr1: ICFGR1,
    #[doc = "0xc08 - GICD interrupt configuration register"]
    pub icfgr2: ICFGR2,
    #[doc = "0xc0c - GICD interrupt configuration register"]
    pub icfgr3: ICFGR3,
    #[doc = "0xc10 - GICD interrupt configuration register"]
    pub icfgr4: ICFGR4,
    #[doc = "0xc14 - GICD interrupt configuration register"]
    pub icfgr5: ICFGR5,
    #[doc = "0xc18 - GICD interrupt configuration register"]
    pub icfgr6: ICFGR6,
    #[doc = "0xc1c - GICD interrupt configuration register"]
    pub icfgr7: ICFGR7,
    #[doc = "0xc20 - GICD interrupt configuration register"]
    pub icfgr8: ICFGR8,
    #[doc = "0xc24 - GICD interrupt configuration register"]
    pub icfgr9: ICFGR9,
    #[doc = "0xc28 - GICD interrupt configuration register"]
    pub icfgr10: ICFGR10,
    #[doc = "0xc2c - GICD interrupt configuration register"]
    pub icfgr11: ICFGR11,
    #[doc = "0xc30 - GICD interrupt configuration register"]
    pub icfgr12: ICFGR12,
    #[doc = "0xc34 - GICD interrupt configuration register"]
    pub icfgr13: ICFGR13,
    #[doc = "0xc38 - GICD interrupt configuration register"]
    pub icfgr14: ICFGR14,
    #[doc = "0xc3c - GICD interrupt configuration register"]
    pub icfgr15: ICFGR15,
    #[doc = "0xc40 - GICD interrupt configuration register"]
    pub icfgr16: ICFGR16,
    #[doc = "0xc44 - GICD interrupt configuration register"]
    pub icfgr17: ICFGR17,
    _reserved228: [u8; 184usize],
    #[doc = "0xd00 - GICD private peripheral interrupt status register"]
    pub ppisr: PPISR,
    #[doc = "0xd04 - GICD shared peripheral interrupt registers"]
    pub spisr0: SPISR0,
    #[doc = "0xd08 - GICD shared peripheral interrupt registers"]
    pub spisr1: SPISR1,
    #[doc = "0xd0c - GICD shared peripheral interrupt registers"]
    pub spisr2: SPISR2,
    #[doc = "0xd10 - GICD shared peripheral interrupt registers"]
    pub spisr3: SPISR3,
    #[doc = "0xd14 - GICD shared peripheral interrupt registers"]
    pub spisr4: SPISR4,
    #[doc = "0xd18 - GICD shared peripheral interrupt registers"]
    pub spisr5: SPISR5,
    #[doc = "0xd1c - GICD shared peripheral interrupt registers"]
    pub spisr6: SPISR6,
    #[doc = "0xd20 - GICD shared peripheral interrupt registers"]
    pub spisr7: SPISR7,
    _reserved237: [u8; 476usize],
    #[doc = "0xf00 - GICD software generated interrupt register"]
    pub sgir: SGIR,
    _reserved238: [u8; 12usize],
    #[doc = "0xf10 - GICD SGI clear-pending registers"]
    pub cpendsgir0: CPENDSGIR0,
    #[doc = "0xf14 - GICD SGI clear-pending registers"]
    pub cpendsgir1: CPENDSGIR1,
    #[doc = "0xf18 - GICD SGI clear-pending registers"]
    pub cpendsgir2: CPENDSGIR2,
    #[doc = "0xf1c - GICD SGI clear-pending registers"]
    pub cpendsgir3: CPENDSGIR3,
    #[doc = "0xf20 - GICD SGI set-pending registers"]
    pub spendsgir0: SPENDSGIR0,
    #[doc = "0xf24 - GICD SGI set-pending registers"]
    pub spendsgir1: SPENDSGIR1,
    #[doc = "0xf28 - GICD SGI set-pending registers"]
    pub spendsgir2: SPENDSGIR2,
    #[doc = "0xf2c - GICD SGI set-pending registers"]
    pub spendsgir3: SPENDSGIR3,
    _reserved246: [u8; 160usize],
    #[doc = "0xfd0 - GICD peripheral ID4 register"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - GICD peripheral ID5 register"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - GICD peripheral ID6 register"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - GICD peripheral ID7 register"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - GICD peripheral ID0 register"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - GICD peripheral ID1 register"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - GICD peripheral ID2 register"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - GICD peripheral ID3 register"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - GICD component ID0 register"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - GICD component ID1 register"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - GICD component ID2 register"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - GICD component ID3 register"]
    pub cidr3: CIDR3,
}
impl RegisterBlock {
    #[doc = "0x00 - GICD control (non-secure access) register"]
    #[inline(always)]
    pub fn gicd_ctlrns(&self) -> &GICD_CTLRNS {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const GICD_CTLRNS) }
    }
    #[doc = "0x00 - GICD control (non-secure access) register"]
    #[inline(always)]
    pub fn gicd_ctlrns_mut(&self) -> &mut GICD_CTLRNS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut GICD_CTLRNS) }
    }
    #[doc = "0x00 - GICD control register"]
    #[inline(always)]
    pub fn gicd_ctlr(&self) -> &GICD_CTLR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const GICD_CTLR) }
    }
    #[doc = "0x00 - GICD control register"]
    #[inline(always)]
    pub fn gicd_ctlr_mut(&self) -> &mut GICD_CTLR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut GICD_CTLR) }
    }
}
#[doc = "GICD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ctlr](gicd_ctlr) module"]
pub type GICD_CTLR = crate::Reg<u32, _GICD_CTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CTLR;
#[doc = "`read()` method returns [gicd_ctlr::R](gicd_ctlr::R) reader structure"]
impl crate::Readable for GICD_CTLR {}
#[doc = "`write(|w| ..)` method takes [gicd_ctlr::W](gicd_ctlr::W) writer structure"]
impl crate::Writable for GICD_CTLR {}
#[doc = "GICD control register"]
pub mod gicd_ctlr;
#[doc = "GICD control (non-secure access) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ctlrns](gicd_ctlrns) module"]
pub type GICD_CTLRNS = crate::Reg<u32, _GICD_CTLRNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CTLRNS;
#[doc = "`read()` method returns [gicd_ctlrns::R](gicd_ctlrns::R) reader structure"]
impl crate::Readable for GICD_CTLRNS {}
#[doc = "`write(|w| ..)` method takes [gicd_ctlrns::W](gicd_ctlrns::W) writer structure"]
impl crate::Writable for GICD_CTLRNS {}
#[doc = "GICD control (non-secure access) register"]
pub mod gicd_ctlrns;
#[doc = "GICD interrupt controller type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_typer](gicd_typer) module"]
pub type GICD_TYPER = crate::Reg<u32, _GICD_TYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_TYPER;
#[doc = "`read()` method returns [gicd_typer::R](gicd_typer::R) reader structure"]
impl crate::Readable for GICD_TYPER {}
#[doc = "GICD interrupt controller type register"]
pub mod gicd_typer;
#[doc = "GICD implementer identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_iidr](gicd_iidr) module"]
pub type GICD_IIDR = crate::Reg<u32, _GICD_IIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IIDR;
#[doc = "`read()` method returns [gicd_iidr::R](gicd_iidr::R) reader structure"]
impl crate::Readable for GICD_IIDR {}
#[doc = "GICD implementer identification register"]
pub mod gicd_iidr;
#[doc = "GICD interrupt group registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr0](gicd_igroupr0) module"]
pub type GICD_IGROUPR0 = crate::Reg<u32, _GICD_IGROUPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR0;
#[doc = "`read()` method returns [gicd_igroupr0::R](gicd_igroupr0::R) reader structure"]
impl crate::Readable for GICD_IGROUPR0 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr0::W](gicd_igroupr0::W) writer structure"]
impl crate::Writable for GICD_IGROUPR0 {}
#[doc = "GICD interrupt group registers"]
pub mod gicd_igroupr0;
#[doc = "GICD interrupt group registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr1](gicd_igroupr1) module"]
pub type GICD_IGROUPR1 = crate::Reg<u32, _GICD_IGROUPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR1;
#[doc = "`read()` method returns [gicd_igroupr1::R](gicd_igroupr1::R) reader structure"]
impl crate::Readable for GICD_IGROUPR1 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr1::W](gicd_igroupr1::W) writer structure"]
impl crate::Writable for GICD_IGROUPR1 {}
#[doc = "GICD interrupt group registers"]
pub mod gicd_igroupr1;
#[doc = "GICD interrupt group registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr2](gicd_igroupr2) module"]
pub type GICD_IGROUPR2 = crate::Reg<u32, _GICD_IGROUPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR2;
#[doc = "`read()` method returns [gicd_igroupr2::R](gicd_igroupr2::R) reader structure"]
impl crate::Readable for GICD_IGROUPR2 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr2::W](gicd_igroupr2::W) writer structure"]
impl crate::Writable for GICD_IGROUPR2 {}
#[doc = "GICD interrupt group registers"]
pub mod gicd_igroupr2;
#[doc = "GICD interrupt group registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr3](gicd_igroupr3) module"]
pub type GICD_IGROUPR3 = crate::Reg<u32, _GICD_IGROUPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR3;
#[doc = "`read()` method returns [gicd_igroupr3::R](gicd_igroupr3::R) reader structure"]
impl crate::Readable for GICD_IGROUPR3 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr3::W](gicd_igroupr3::W) writer structure"]
impl crate::Writable for GICD_IGROUPR3 {}
#[doc = "GICD interrupt group registers"]
pub mod gicd_igroupr3;
#[doc = "GICD interrupt group registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr4](gicd_igroupr4) module"]
pub type GICD_IGROUPR4 = crate::Reg<u32, _GICD_IGROUPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR4;
#[doc = "`read()` method returns [gicd_igroupr4::R](gicd_igroupr4::R) reader structure"]
impl crate::Readable for GICD_IGROUPR4 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr4::W](gicd_igroupr4::W) writer structure"]
impl crate::Writable for GICD_IGROUPR4 {}
#[doc = "GICD interrupt group registers"]
pub mod gicd_igroupr4;
#[doc = "GICD interrupt group registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr5](gicd_igroupr5) module"]
pub type GICD_IGROUPR5 = crate::Reg<u32, _GICD_IGROUPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR5;
#[doc = "`read()` method returns [gicd_igroupr5::R](gicd_igroupr5::R) reader structure"]
impl crate::Readable for GICD_IGROUPR5 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr5::W](gicd_igroupr5::W) writer structure"]
impl crate::Writable for GICD_IGROUPR5 {}
#[doc = "GICD interrupt group registers"]
pub mod gicd_igroupr5;
#[doc = "GICD interrupt group registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr6](gicd_igroupr6) module"]
pub type GICD_IGROUPR6 = crate::Reg<u32, _GICD_IGROUPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR6;
#[doc = "`read()` method returns [gicd_igroupr6::R](gicd_igroupr6::R) reader structure"]
impl crate::Readable for GICD_IGROUPR6 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr6::W](gicd_igroupr6::W) writer structure"]
impl crate::Writable for GICD_IGROUPR6 {}
#[doc = "GICD interrupt group registers"]
pub mod gicd_igroupr6;
#[doc = "GICD interrupt group registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr7](gicd_igroupr7) module"]
pub type GICD_IGROUPR7 = crate::Reg<u32, _GICD_IGROUPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR7;
#[doc = "`read()` method returns [gicd_igroupr7::R](gicd_igroupr7::R) reader structure"]
impl crate::Readable for GICD_IGROUPR7 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr7::W](gicd_igroupr7::W) writer structure"]
impl crate::Writable for GICD_IGROUPR7 {}
#[doc = "GICD interrupt group registers"]
pub mod gicd_igroupr7;
#[doc = "GICD interrupt group registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr8](gicd_igroupr8) module"]
pub type GICD_IGROUPR8 = crate::Reg<u32, _GICD_IGROUPR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR8;
#[doc = "`read()` method returns [gicd_igroupr8::R](gicd_igroupr8::R) reader structure"]
impl crate::Readable for GICD_IGROUPR8 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr8::W](gicd_igroupr8::W) writer structure"]
impl crate::Writable for GICD_IGROUPR8 {}
#[doc = "GICD interrupt group registers"]
pub mod gicd_igroupr8;
#[doc = "GICD interrupt set-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isenabler0](isenabler0) module"]
pub type ISENABLER0 = crate::Reg<u32, _ISENABLER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISENABLER0;
#[doc = "`read()` method returns [isenabler0::R](isenabler0::R) reader structure"]
impl crate::Readable for ISENABLER0 {}
#[doc = "`write(|w| ..)` method takes [isenabler0::W](isenabler0::W) writer structure"]
impl crate::Writable for ISENABLER0 {}
#[doc = "GICD interrupt set-enable register"]
pub mod isenabler0;
#[doc = "GICD interrupt set-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isenabler1](isenabler1) module"]
pub type ISENABLER1 = crate::Reg<u32, _ISENABLER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISENABLER1;
#[doc = "`read()` method returns [isenabler1::R](isenabler1::R) reader structure"]
impl crate::Readable for ISENABLER1 {}
#[doc = "`write(|w| ..)` method takes [isenabler1::W](isenabler1::W) writer structure"]
impl crate::Writable for ISENABLER1 {}
#[doc = "GICD interrupt set-enable register"]
pub mod isenabler1;
#[doc = "GICD interrupt set-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isenabler2](isenabler2) module"]
pub type ISENABLER2 = crate::Reg<u32, _ISENABLER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISENABLER2;
#[doc = "`read()` method returns [isenabler2::R](isenabler2::R) reader structure"]
impl crate::Readable for ISENABLER2 {}
#[doc = "`write(|w| ..)` method takes [isenabler2::W](isenabler2::W) writer structure"]
impl crate::Writable for ISENABLER2 {}
#[doc = "GICD interrupt set-enable register"]
pub mod isenabler2;
#[doc = "GICD interrupt set-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isenabler3](isenabler3) module"]
pub type ISENABLER3 = crate::Reg<u32, _ISENABLER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISENABLER3;
#[doc = "`read()` method returns [isenabler3::R](isenabler3::R) reader structure"]
impl crate::Readable for ISENABLER3 {}
#[doc = "`write(|w| ..)` method takes [isenabler3::W](isenabler3::W) writer structure"]
impl crate::Writable for ISENABLER3 {}
#[doc = "GICD interrupt set-enable register"]
pub mod isenabler3;
#[doc = "GICD interrupt set-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isenabler4](isenabler4) module"]
pub type ISENABLER4 = crate::Reg<u32, _ISENABLER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISENABLER4;
#[doc = "`read()` method returns [isenabler4::R](isenabler4::R) reader structure"]
impl crate::Readable for ISENABLER4 {}
#[doc = "`write(|w| ..)` method takes [isenabler4::W](isenabler4::W) writer structure"]
impl crate::Writable for ISENABLER4 {}
#[doc = "GICD interrupt set-enable register"]
pub mod isenabler4;
#[doc = "GICD interrupt set-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isenabler5](isenabler5) module"]
pub type ISENABLER5 = crate::Reg<u32, _ISENABLER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISENABLER5;
#[doc = "`read()` method returns [isenabler5::R](isenabler5::R) reader structure"]
impl crate::Readable for ISENABLER5 {}
#[doc = "`write(|w| ..)` method takes [isenabler5::W](isenabler5::W) writer structure"]
impl crate::Writable for ISENABLER5 {}
#[doc = "GICD interrupt set-enable register"]
pub mod isenabler5;
#[doc = "GICD interrupt set-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isenabler6](isenabler6) module"]
pub type ISENABLER6 = crate::Reg<u32, _ISENABLER6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISENABLER6;
#[doc = "`read()` method returns [isenabler6::R](isenabler6::R) reader structure"]
impl crate::Readable for ISENABLER6 {}
#[doc = "`write(|w| ..)` method takes [isenabler6::W](isenabler6::W) writer structure"]
impl crate::Writable for ISENABLER6 {}
#[doc = "GICD interrupt set-enable register"]
pub mod isenabler6;
#[doc = "GICD interrupt set-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isenabler7](isenabler7) module"]
pub type ISENABLER7 = crate::Reg<u32, _ISENABLER7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISENABLER7;
#[doc = "`read()` method returns [isenabler7::R](isenabler7::R) reader structure"]
impl crate::Readable for ISENABLER7 {}
#[doc = "`write(|w| ..)` method takes [isenabler7::W](isenabler7::W) writer structure"]
impl crate::Writable for ISENABLER7 {}
#[doc = "GICD interrupt set-enable register"]
pub mod isenabler7;
#[doc = "GICD interrupt set-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isenabler8](isenabler8) module"]
pub type ISENABLER8 = crate::Reg<u32, _ISENABLER8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISENABLER8;
#[doc = "`read()` method returns [isenabler8::R](isenabler8::R) reader structure"]
impl crate::Readable for ISENABLER8 {}
#[doc = "`write(|w| ..)` method takes [isenabler8::W](isenabler8::W) writer structure"]
impl crate::Writable for ISENABLER8 {}
#[doc = "GICD interrupt set-enable register"]
pub mod isenabler8;
#[doc = "GICD interrupt clear-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icenabler0](icenabler0) module"]
pub type ICENABLER0 = crate::Reg<u32, _ICENABLER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICENABLER0;
#[doc = "`read()` method returns [icenabler0::R](icenabler0::R) reader structure"]
impl crate::Readable for ICENABLER0 {}
#[doc = "`write(|w| ..)` method takes [icenabler0::W](icenabler0::W) writer structure"]
impl crate::Writable for ICENABLER0 {}
#[doc = "GICD interrupt clear-enable register"]
pub mod icenabler0;
#[doc = "GICD interrupt clear-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icenabler1](icenabler1) module"]
pub type ICENABLER1 = crate::Reg<u32, _ICENABLER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICENABLER1;
#[doc = "`read()` method returns [icenabler1::R](icenabler1::R) reader structure"]
impl crate::Readable for ICENABLER1 {}
#[doc = "`write(|w| ..)` method takes [icenabler1::W](icenabler1::W) writer structure"]
impl crate::Writable for ICENABLER1 {}
#[doc = "GICD interrupt clear-enable register"]
pub mod icenabler1;
#[doc = "GICD interrupt clear-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icenabler2](icenabler2) module"]
pub type ICENABLER2 = crate::Reg<u32, _ICENABLER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICENABLER2;
#[doc = "`read()` method returns [icenabler2::R](icenabler2::R) reader structure"]
impl crate::Readable for ICENABLER2 {}
#[doc = "`write(|w| ..)` method takes [icenabler2::W](icenabler2::W) writer structure"]
impl crate::Writable for ICENABLER2 {}
#[doc = "GICD interrupt clear-enable register"]
pub mod icenabler2;
#[doc = "GICD interrupt clear-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icenabler3](icenabler3) module"]
pub type ICENABLER3 = crate::Reg<u32, _ICENABLER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICENABLER3;
#[doc = "`read()` method returns [icenabler3::R](icenabler3::R) reader structure"]
impl crate::Readable for ICENABLER3 {}
#[doc = "`write(|w| ..)` method takes [icenabler3::W](icenabler3::W) writer structure"]
impl crate::Writable for ICENABLER3 {}
#[doc = "GICD interrupt clear-enable register"]
pub mod icenabler3;
#[doc = "GICD interrupt clear-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icenabler4](icenabler4) module"]
pub type ICENABLER4 = crate::Reg<u32, _ICENABLER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICENABLER4;
#[doc = "`read()` method returns [icenabler4::R](icenabler4::R) reader structure"]
impl crate::Readable for ICENABLER4 {}
#[doc = "`write(|w| ..)` method takes [icenabler4::W](icenabler4::W) writer structure"]
impl crate::Writable for ICENABLER4 {}
#[doc = "GICD interrupt clear-enable register"]
pub mod icenabler4;
#[doc = "GICD interrupt clear-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icenabler5](icenabler5) module"]
pub type ICENABLER5 = crate::Reg<u32, _ICENABLER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICENABLER5;
#[doc = "`read()` method returns [icenabler5::R](icenabler5::R) reader structure"]
impl crate::Readable for ICENABLER5 {}
#[doc = "`write(|w| ..)` method takes [icenabler5::W](icenabler5::W) writer structure"]
impl crate::Writable for ICENABLER5 {}
#[doc = "GICD interrupt clear-enable register"]
pub mod icenabler5;
#[doc = "GICD interrupt clear-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icenabler6](icenabler6) module"]
pub type ICENABLER6 = crate::Reg<u32, _ICENABLER6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICENABLER6;
#[doc = "`read()` method returns [icenabler6::R](icenabler6::R) reader structure"]
impl crate::Readable for ICENABLER6 {}
#[doc = "`write(|w| ..)` method takes [icenabler6::W](icenabler6::W) writer structure"]
impl crate::Writable for ICENABLER6 {}
#[doc = "GICD interrupt clear-enable register"]
pub mod icenabler6;
#[doc = "GICD interrupt clear-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icenabler7](icenabler7) module"]
pub type ICENABLER7 = crate::Reg<u32, _ICENABLER7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICENABLER7;
#[doc = "`read()` method returns [icenabler7::R](icenabler7::R) reader structure"]
impl crate::Readable for ICENABLER7 {}
#[doc = "`write(|w| ..)` method takes [icenabler7::W](icenabler7::W) writer structure"]
impl crate::Writable for ICENABLER7 {}
#[doc = "GICD interrupt clear-enable register"]
pub mod icenabler7;
#[doc = "GICD interrupt clear-enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icenabler8](icenabler8) module"]
pub type ICENABLER8 = crate::Reg<u32, _ICENABLER8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICENABLER8;
#[doc = "`read()` method returns [icenabler8::R](icenabler8::R) reader structure"]
impl crate::Readable for ICENABLER8 {}
#[doc = "`write(|w| ..)` method takes [icenabler8::W](icenabler8::W) writer structure"]
impl crate::Writable for ICENABLER8 {}
#[doc = "GICD interrupt clear-enable register"]
pub mod icenabler8;
#[doc = "GICD interrupt set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispendr0](ispendr0) module"]
pub type ISPENDR0 = crate::Reg<u32, _ISPENDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPENDR0;
#[doc = "`read()` method returns [ispendr0::R](ispendr0::R) reader structure"]
impl crate::Readable for ISPENDR0 {}
#[doc = "`write(|w| ..)` method takes [ispendr0::W](ispendr0::W) writer structure"]
impl crate::Writable for ISPENDR0 {}
#[doc = "GICD interrupt set-pending registers"]
pub mod ispendr0;
#[doc = "GICD interrupt set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispendr1](ispendr1) module"]
pub type ISPENDR1 = crate::Reg<u32, _ISPENDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPENDR1;
#[doc = "`read()` method returns [ispendr1::R](ispendr1::R) reader structure"]
impl crate::Readable for ISPENDR1 {}
#[doc = "`write(|w| ..)` method takes [ispendr1::W](ispendr1::W) writer structure"]
impl crate::Writable for ISPENDR1 {}
#[doc = "GICD interrupt set-pending registers"]
pub mod ispendr1;
#[doc = "GICD interrupt set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispendr2](ispendr2) module"]
pub type ISPENDR2 = crate::Reg<u32, _ISPENDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPENDR2;
#[doc = "`read()` method returns [ispendr2::R](ispendr2::R) reader structure"]
impl crate::Readable for ISPENDR2 {}
#[doc = "`write(|w| ..)` method takes [ispendr2::W](ispendr2::W) writer structure"]
impl crate::Writable for ISPENDR2 {}
#[doc = "GICD interrupt set-pending registers"]
pub mod ispendr2;
#[doc = "GICD interrupt set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispendr3](ispendr3) module"]
pub type ISPENDR3 = crate::Reg<u32, _ISPENDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPENDR3;
#[doc = "`read()` method returns [ispendr3::R](ispendr3::R) reader structure"]
impl crate::Readable for ISPENDR3 {}
#[doc = "`write(|w| ..)` method takes [ispendr3::W](ispendr3::W) writer structure"]
impl crate::Writable for ISPENDR3 {}
#[doc = "GICD interrupt set-pending registers"]
pub mod ispendr3;
#[doc = "GICD interrupt set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispendr4](ispendr4) module"]
pub type ISPENDR4 = crate::Reg<u32, _ISPENDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPENDR4;
#[doc = "`read()` method returns [ispendr4::R](ispendr4::R) reader structure"]
impl crate::Readable for ISPENDR4 {}
#[doc = "`write(|w| ..)` method takes [ispendr4::W](ispendr4::W) writer structure"]
impl crate::Writable for ISPENDR4 {}
#[doc = "GICD interrupt set-pending registers"]
pub mod ispendr4;
#[doc = "GICD interrupt set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispendr5](ispendr5) module"]
pub type ISPENDR5 = crate::Reg<u32, _ISPENDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPENDR5;
#[doc = "`read()` method returns [ispendr5::R](ispendr5::R) reader structure"]
impl crate::Readable for ISPENDR5 {}
#[doc = "`write(|w| ..)` method takes [ispendr5::W](ispendr5::W) writer structure"]
impl crate::Writable for ISPENDR5 {}
#[doc = "GICD interrupt set-pending registers"]
pub mod ispendr5;
#[doc = "GICD interrupt set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispendr6](ispendr6) module"]
pub type ISPENDR6 = crate::Reg<u32, _ISPENDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPENDR6;
#[doc = "`read()` method returns [ispendr6::R](ispendr6::R) reader structure"]
impl crate::Readable for ISPENDR6 {}
#[doc = "`write(|w| ..)` method takes [ispendr6::W](ispendr6::W) writer structure"]
impl crate::Writable for ISPENDR6 {}
#[doc = "GICD interrupt set-pending registers"]
pub mod ispendr6;
#[doc = "GICD interrupt set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispendr7](ispendr7) module"]
pub type ISPENDR7 = crate::Reg<u32, _ISPENDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPENDR7;
#[doc = "`read()` method returns [ispendr7::R](ispendr7::R) reader structure"]
impl crate::Readable for ISPENDR7 {}
#[doc = "`write(|w| ..)` method takes [ispendr7::W](ispendr7::W) writer structure"]
impl crate::Writable for ISPENDR7 {}
#[doc = "GICD interrupt set-pending registers"]
pub mod ispendr7;
#[doc = "GICD interrupt set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispendr8](ispendr8) module"]
pub type ISPENDR8 = crate::Reg<u32, _ISPENDR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISPENDR8;
#[doc = "`read()` method returns [ispendr8::R](ispendr8::R) reader structure"]
impl crate::Readable for ISPENDR8 {}
#[doc = "`write(|w| ..)` method takes [ispendr8::W](ispendr8::W) writer structure"]
impl crate::Writable for ISPENDR8 {}
#[doc = "GICD interrupt set-pending registers"]
pub mod ispendr8;
#[doc = "GICD interrupt clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icpendr0](icpendr0) module"]
pub type ICPENDR0 = crate::Reg<u32, _ICPENDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICPENDR0;
#[doc = "`read()` method returns [icpendr0::R](icpendr0::R) reader structure"]
impl crate::Readable for ICPENDR0 {}
#[doc = "`write(|w| ..)` method takes [icpendr0::W](icpendr0::W) writer structure"]
impl crate::Writable for ICPENDR0 {}
#[doc = "GICD interrupt clear-pending registers"]
pub mod icpendr0;
#[doc = "GICD interrupt clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icpendr1](icpendr1) module"]
pub type ICPENDR1 = crate::Reg<u32, _ICPENDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICPENDR1;
#[doc = "`read()` method returns [icpendr1::R](icpendr1::R) reader structure"]
impl crate::Readable for ICPENDR1 {}
#[doc = "`write(|w| ..)` method takes [icpendr1::W](icpendr1::W) writer structure"]
impl crate::Writable for ICPENDR1 {}
#[doc = "GICD interrupt clear-pending registers"]
pub mod icpendr1;
#[doc = "GICD interrupt clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icpendr2](icpendr2) module"]
pub type ICPENDR2 = crate::Reg<u32, _ICPENDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICPENDR2;
#[doc = "`read()` method returns [icpendr2::R](icpendr2::R) reader structure"]
impl crate::Readable for ICPENDR2 {}
#[doc = "`write(|w| ..)` method takes [icpendr2::W](icpendr2::W) writer structure"]
impl crate::Writable for ICPENDR2 {}
#[doc = "GICD interrupt clear-pending registers"]
pub mod icpendr2;
#[doc = "GICD interrupt clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icpendr3](icpendr3) module"]
pub type ICPENDR3 = crate::Reg<u32, _ICPENDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICPENDR3;
#[doc = "`read()` method returns [icpendr3::R](icpendr3::R) reader structure"]
impl crate::Readable for ICPENDR3 {}
#[doc = "`write(|w| ..)` method takes [icpendr3::W](icpendr3::W) writer structure"]
impl crate::Writable for ICPENDR3 {}
#[doc = "GICD interrupt clear-pending registers"]
pub mod icpendr3;
#[doc = "GICD interrupt clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icpendr4](icpendr4) module"]
pub type ICPENDR4 = crate::Reg<u32, _ICPENDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICPENDR4;
#[doc = "`read()` method returns [icpendr4::R](icpendr4::R) reader structure"]
impl crate::Readable for ICPENDR4 {}
#[doc = "`write(|w| ..)` method takes [icpendr4::W](icpendr4::W) writer structure"]
impl crate::Writable for ICPENDR4 {}
#[doc = "GICD interrupt clear-pending registers"]
pub mod icpendr4;
#[doc = "GICD interrupt clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icpendr5](icpendr5) module"]
pub type ICPENDR5 = crate::Reg<u32, _ICPENDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICPENDR5;
#[doc = "`read()` method returns [icpendr5::R](icpendr5::R) reader structure"]
impl crate::Readable for ICPENDR5 {}
#[doc = "`write(|w| ..)` method takes [icpendr5::W](icpendr5::W) writer structure"]
impl crate::Writable for ICPENDR5 {}
#[doc = "GICD interrupt clear-pending registers"]
pub mod icpendr5;
#[doc = "GICD interrupt clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icpendr6](icpendr6) module"]
pub type ICPENDR6 = crate::Reg<u32, _ICPENDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICPENDR6;
#[doc = "`read()` method returns [icpendr6::R](icpendr6::R) reader structure"]
impl crate::Readable for ICPENDR6 {}
#[doc = "`write(|w| ..)` method takes [icpendr6::W](icpendr6::W) writer structure"]
impl crate::Writable for ICPENDR6 {}
#[doc = "GICD interrupt clear-pending registers"]
pub mod icpendr6;
#[doc = "GICD interrupt clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icpendr7](icpendr7) module"]
pub type ICPENDR7 = crate::Reg<u32, _ICPENDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICPENDR7;
#[doc = "`read()` method returns [icpendr7::R](icpendr7::R) reader structure"]
impl crate::Readable for ICPENDR7 {}
#[doc = "`write(|w| ..)` method takes [icpendr7::W](icpendr7::W) writer structure"]
impl crate::Writable for ICPENDR7 {}
#[doc = "GICD interrupt clear-pending registers"]
pub mod icpendr7;
#[doc = "GICD interrupt clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icpendr8](icpendr8) module"]
pub type ICPENDR8 = crate::Reg<u32, _ICPENDR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICPENDR8;
#[doc = "`read()` method returns [icpendr8::R](icpendr8::R) reader structure"]
impl crate::Readable for ICPENDR8 {}
#[doc = "`write(|w| ..)` method takes [icpendr8::W](icpendr8::W) writer structure"]
impl crate::Writable for ICPENDR8 {}
#[doc = "GICD interrupt clear-pending registers"]
pub mod icpendr8;
#[doc = "GICD interrupt set-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isactiver0](isactiver0) module"]
pub type ISACTIVER0 = crate::Reg<u32, _ISACTIVER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISACTIVER0;
#[doc = "`read()` method returns [isactiver0::R](isactiver0::R) reader structure"]
impl crate::Readable for ISACTIVER0 {}
#[doc = "`write(|w| ..)` method takes [isactiver0::W](isactiver0::W) writer structure"]
impl crate::Writable for ISACTIVER0 {}
#[doc = "GICD interrupt set-active registers"]
pub mod isactiver0;
#[doc = "GICD interrupt set-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isactiver1](isactiver1) module"]
pub type ISACTIVER1 = crate::Reg<u32, _ISACTIVER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISACTIVER1;
#[doc = "`read()` method returns [isactiver1::R](isactiver1::R) reader structure"]
impl crate::Readable for ISACTIVER1 {}
#[doc = "`write(|w| ..)` method takes [isactiver1::W](isactiver1::W) writer structure"]
impl crate::Writable for ISACTIVER1 {}
#[doc = "GICD interrupt set-active registers"]
pub mod isactiver1;
#[doc = "GICD interrupt set-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isactiver2](isactiver2) module"]
pub type ISACTIVER2 = crate::Reg<u32, _ISACTIVER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISACTIVER2;
#[doc = "`read()` method returns [isactiver2::R](isactiver2::R) reader structure"]
impl crate::Readable for ISACTIVER2 {}
#[doc = "`write(|w| ..)` method takes [isactiver2::W](isactiver2::W) writer structure"]
impl crate::Writable for ISACTIVER2 {}
#[doc = "GICD interrupt set-active registers"]
pub mod isactiver2;
#[doc = "GICD interrupt set-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isactiver3](isactiver3) module"]
pub type ISACTIVER3 = crate::Reg<u32, _ISACTIVER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISACTIVER3;
#[doc = "`read()` method returns [isactiver3::R](isactiver3::R) reader structure"]
impl crate::Readable for ISACTIVER3 {}
#[doc = "`write(|w| ..)` method takes [isactiver3::W](isactiver3::W) writer structure"]
impl crate::Writable for ISACTIVER3 {}
#[doc = "GICD interrupt set-active registers"]
pub mod isactiver3;
#[doc = "GICD interrupt set-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isactiver4](isactiver4) module"]
pub type ISACTIVER4 = crate::Reg<u32, _ISACTIVER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISACTIVER4;
#[doc = "`read()` method returns [isactiver4::R](isactiver4::R) reader structure"]
impl crate::Readable for ISACTIVER4 {}
#[doc = "`write(|w| ..)` method takes [isactiver4::W](isactiver4::W) writer structure"]
impl crate::Writable for ISACTIVER4 {}
#[doc = "GICD interrupt set-active registers"]
pub mod isactiver4;
#[doc = "GICD interrupt set-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isactiver5](isactiver5) module"]
pub type ISACTIVER5 = crate::Reg<u32, _ISACTIVER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISACTIVER5;
#[doc = "`read()` method returns [isactiver5::R](isactiver5::R) reader structure"]
impl crate::Readable for ISACTIVER5 {}
#[doc = "`write(|w| ..)` method takes [isactiver5::W](isactiver5::W) writer structure"]
impl crate::Writable for ISACTIVER5 {}
#[doc = "GICD interrupt set-active registers"]
pub mod isactiver5;
#[doc = "GICD interrupt set-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isactiver6](isactiver6) module"]
pub type ISACTIVER6 = crate::Reg<u32, _ISACTIVER6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISACTIVER6;
#[doc = "`read()` method returns [isactiver6::R](isactiver6::R) reader structure"]
impl crate::Readable for ISACTIVER6 {}
#[doc = "`write(|w| ..)` method takes [isactiver6::W](isactiver6::W) writer structure"]
impl crate::Writable for ISACTIVER6 {}
#[doc = "GICD interrupt set-active registers"]
pub mod isactiver6;
#[doc = "GICD interrupt set-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isactiver7](isactiver7) module"]
pub type ISACTIVER7 = crate::Reg<u32, _ISACTIVER7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISACTIVER7;
#[doc = "`read()` method returns [isactiver7::R](isactiver7::R) reader structure"]
impl crate::Readable for ISACTIVER7 {}
#[doc = "`write(|w| ..)` method takes [isactiver7::W](isactiver7::W) writer structure"]
impl crate::Writable for ISACTIVER7 {}
#[doc = "GICD interrupt set-active registers"]
pub mod isactiver7;
#[doc = "GICD interrupt set-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isactiver8](isactiver8) module"]
pub type ISACTIVER8 = crate::Reg<u32, _ISACTIVER8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISACTIVER8;
#[doc = "`read()` method returns [isactiver8::R](isactiver8::R) reader structure"]
impl crate::Readable for ISACTIVER8 {}
#[doc = "`write(|w| ..)` method takes [isactiver8::W](isactiver8::W) writer structure"]
impl crate::Writable for ISACTIVER8 {}
#[doc = "GICD interrupt set-active registers"]
pub mod isactiver8;
#[doc = "GICD interrupt clear-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icactiver0](icactiver0) module"]
pub type ICACTIVER0 = crate::Reg<u32, _ICACTIVER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACTIVER0;
#[doc = "`read()` method returns [icactiver0::R](icactiver0::R) reader structure"]
impl crate::Readable for ICACTIVER0 {}
#[doc = "`write(|w| ..)` method takes [icactiver0::W](icactiver0::W) writer structure"]
impl crate::Writable for ICACTIVER0 {}
#[doc = "GICD interrupt clear-active registers"]
pub mod icactiver0;
#[doc = "GICD interrupt clear-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icactiver1](icactiver1) module"]
pub type ICACTIVER1 = crate::Reg<u32, _ICACTIVER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACTIVER1;
#[doc = "`read()` method returns [icactiver1::R](icactiver1::R) reader structure"]
impl crate::Readable for ICACTIVER1 {}
#[doc = "`write(|w| ..)` method takes [icactiver1::W](icactiver1::W) writer structure"]
impl crate::Writable for ICACTIVER1 {}
#[doc = "GICD interrupt clear-active registers"]
pub mod icactiver1;
#[doc = "GICD interrupt clear-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icactiver2](icactiver2) module"]
pub type ICACTIVER2 = crate::Reg<u32, _ICACTIVER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACTIVER2;
#[doc = "`read()` method returns [icactiver2::R](icactiver2::R) reader structure"]
impl crate::Readable for ICACTIVER2 {}
#[doc = "`write(|w| ..)` method takes [icactiver2::W](icactiver2::W) writer structure"]
impl crate::Writable for ICACTIVER2 {}
#[doc = "GICD interrupt clear-active registers"]
pub mod icactiver2;
#[doc = "GICD interrupt clear-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icactiver3](icactiver3) module"]
pub type ICACTIVER3 = crate::Reg<u32, _ICACTIVER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACTIVER3;
#[doc = "`read()` method returns [icactiver3::R](icactiver3::R) reader structure"]
impl crate::Readable for ICACTIVER3 {}
#[doc = "`write(|w| ..)` method takes [icactiver3::W](icactiver3::W) writer structure"]
impl crate::Writable for ICACTIVER3 {}
#[doc = "GICD interrupt clear-active registers"]
pub mod icactiver3;
#[doc = "GICD interrupt clear-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icactiver4](icactiver4) module"]
pub type ICACTIVER4 = crate::Reg<u32, _ICACTIVER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACTIVER4;
#[doc = "`read()` method returns [icactiver4::R](icactiver4::R) reader structure"]
impl crate::Readable for ICACTIVER4 {}
#[doc = "`write(|w| ..)` method takes [icactiver4::W](icactiver4::W) writer structure"]
impl crate::Writable for ICACTIVER4 {}
#[doc = "GICD interrupt clear-active registers"]
pub mod icactiver4;
#[doc = "GICD interrupt clear-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icactiver5](icactiver5) module"]
pub type ICACTIVER5 = crate::Reg<u32, _ICACTIVER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACTIVER5;
#[doc = "`read()` method returns [icactiver5::R](icactiver5::R) reader structure"]
impl crate::Readable for ICACTIVER5 {}
#[doc = "`write(|w| ..)` method takes [icactiver5::W](icactiver5::W) writer structure"]
impl crate::Writable for ICACTIVER5 {}
#[doc = "GICD interrupt clear-active registers"]
pub mod icactiver5;
#[doc = "GICD interrupt clear-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icactiver6](icactiver6) module"]
pub type ICACTIVER6 = crate::Reg<u32, _ICACTIVER6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACTIVER6;
#[doc = "`read()` method returns [icactiver6::R](icactiver6::R) reader structure"]
impl crate::Readable for ICACTIVER6 {}
#[doc = "`write(|w| ..)` method takes [icactiver6::W](icactiver6::W) writer structure"]
impl crate::Writable for ICACTIVER6 {}
#[doc = "GICD interrupt clear-active registers"]
pub mod icactiver6;
#[doc = "GICD interrupt clear-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icactiver7](icactiver7) module"]
pub type ICACTIVER7 = crate::Reg<u32, _ICACTIVER7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACTIVER7;
#[doc = "`read()` method returns [icactiver7::R](icactiver7::R) reader structure"]
impl crate::Readable for ICACTIVER7 {}
#[doc = "`write(|w| ..)` method takes [icactiver7::W](icactiver7::W) writer structure"]
impl crate::Writable for ICACTIVER7 {}
#[doc = "GICD interrupt clear-active registers"]
pub mod icactiver7;
#[doc = "GICD interrupt clear-active registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icactiver8](icactiver8) module"]
pub type ICACTIVER8 = crate::Reg<u32, _ICACTIVER8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACTIVER8;
#[doc = "`read()` method returns [icactiver8::R](icactiver8::R) reader structure"]
impl crate::Readable for ICACTIVER8 {}
#[doc = "`write(|w| ..)` method takes [icactiver8::W](icactiver8::W) writer structure"]
impl crate::Writable for ICACTIVER8 {}
#[doc = "GICD interrupt clear-active registers"]
pub mod icactiver8;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr0](ipriorityr0) module"]
pub type IPRIORITYR0 = crate::Reg<u32, _IPRIORITYR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR0;
#[doc = "`read()` method returns [ipriorityr0::R](ipriorityr0::R) reader structure"]
impl crate::Readable for IPRIORITYR0 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr0::W](ipriorityr0::W) writer structure"]
impl crate::Writable for IPRIORITYR0 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr0;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr1](ipriorityr1) module"]
pub type IPRIORITYR1 = crate::Reg<u32, _IPRIORITYR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR1;
#[doc = "`read()` method returns [ipriorityr1::R](ipriorityr1::R) reader structure"]
impl crate::Readable for IPRIORITYR1 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr1::W](ipriorityr1::W) writer structure"]
impl crate::Writable for IPRIORITYR1 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr1;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr2](ipriorityr2) module"]
pub type IPRIORITYR2 = crate::Reg<u32, _IPRIORITYR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR2;
#[doc = "`read()` method returns [ipriorityr2::R](ipriorityr2::R) reader structure"]
impl crate::Readable for IPRIORITYR2 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr2::W](ipriorityr2::W) writer structure"]
impl crate::Writable for IPRIORITYR2 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr2;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr3](ipriorityr3) module"]
pub type IPRIORITYR3 = crate::Reg<u32, _IPRIORITYR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR3;
#[doc = "`read()` method returns [ipriorityr3::R](ipriorityr3::R) reader structure"]
impl crate::Readable for IPRIORITYR3 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr3::W](ipriorityr3::W) writer structure"]
impl crate::Writable for IPRIORITYR3 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr3;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr4](ipriorityr4) module"]
pub type IPRIORITYR4 = crate::Reg<u32, _IPRIORITYR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR4;
#[doc = "`read()` method returns [ipriorityr4::R](ipriorityr4::R) reader structure"]
impl crate::Readable for IPRIORITYR4 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr4::W](ipriorityr4::W) writer structure"]
impl crate::Writable for IPRIORITYR4 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr4;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr5](ipriorityr5) module"]
pub type IPRIORITYR5 = crate::Reg<u32, _IPRIORITYR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR5;
#[doc = "`read()` method returns [ipriorityr5::R](ipriorityr5::R) reader structure"]
impl crate::Readable for IPRIORITYR5 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr5::W](ipriorityr5::W) writer structure"]
impl crate::Writable for IPRIORITYR5 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr5;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr6](ipriorityr6) module"]
pub type IPRIORITYR6 = crate::Reg<u32, _IPRIORITYR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR6;
#[doc = "`read()` method returns [ipriorityr6::R](ipriorityr6::R) reader structure"]
impl crate::Readable for IPRIORITYR6 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr6::W](ipriorityr6::W) writer structure"]
impl crate::Writable for IPRIORITYR6 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr6;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr7](ipriorityr7) module"]
pub type IPRIORITYR7 = crate::Reg<u32, _IPRIORITYR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR7;
#[doc = "`read()` method returns [ipriorityr7::R](ipriorityr7::R) reader structure"]
impl crate::Readable for IPRIORITYR7 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr7::W](ipriorityr7::W) writer structure"]
impl crate::Writable for IPRIORITYR7 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr7;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr8](ipriorityr8) module"]
pub type IPRIORITYR8 = crate::Reg<u32, _IPRIORITYR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR8;
#[doc = "`read()` method returns [ipriorityr8::R](ipriorityr8::R) reader structure"]
impl crate::Readable for IPRIORITYR8 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr8::W](ipriorityr8::W) writer structure"]
impl crate::Writable for IPRIORITYR8 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr8;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr9](ipriorityr9) module"]
pub type IPRIORITYR9 = crate::Reg<u32, _IPRIORITYR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR9;
#[doc = "`read()` method returns [ipriorityr9::R](ipriorityr9::R) reader structure"]
impl crate::Readable for IPRIORITYR9 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr9::W](ipriorityr9::W) writer structure"]
impl crate::Writable for IPRIORITYR9 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr9;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr10](ipriorityr10) module"]
pub type IPRIORITYR10 = crate::Reg<u32, _IPRIORITYR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR10;
#[doc = "`read()` method returns [ipriorityr10::R](ipriorityr10::R) reader structure"]
impl crate::Readable for IPRIORITYR10 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr10::W](ipriorityr10::W) writer structure"]
impl crate::Writable for IPRIORITYR10 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr10;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr11](ipriorityr11) module"]
pub type IPRIORITYR11 = crate::Reg<u32, _IPRIORITYR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR11;
#[doc = "`read()` method returns [ipriorityr11::R](ipriorityr11::R) reader structure"]
impl crate::Readable for IPRIORITYR11 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr11::W](ipriorityr11::W) writer structure"]
impl crate::Writable for IPRIORITYR11 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr11;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr12](ipriorityr12) module"]
pub type IPRIORITYR12 = crate::Reg<u32, _IPRIORITYR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR12;
#[doc = "`read()` method returns [ipriorityr12::R](ipriorityr12::R) reader structure"]
impl crate::Readable for IPRIORITYR12 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr12::W](ipriorityr12::W) writer structure"]
impl crate::Writable for IPRIORITYR12 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr12;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr13](ipriorityr13) module"]
pub type IPRIORITYR13 = crate::Reg<u32, _IPRIORITYR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR13;
#[doc = "`read()` method returns [ipriorityr13::R](ipriorityr13::R) reader structure"]
impl crate::Readable for IPRIORITYR13 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr13::W](ipriorityr13::W) writer structure"]
impl crate::Writable for IPRIORITYR13 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr13;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr14](ipriorityr14) module"]
pub type IPRIORITYR14 = crate::Reg<u32, _IPRIORITYR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR14;
#[doc = "`read()` method returns [ipriorityr14::R](ipriorityr14::R) reader structure"]
impl crate::Readable for IPRIORITYR14 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr14::W](ipriorityr14::W) writer structure"]
impl crate::Writable for IPRIORITYR14 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr14;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr15](ipriorityr15) module"]
pub type IPRIORITYR15 = crate::Reg<u32, _IPRIORITYR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR15;
#[doc = "`read()` method returns [ipriorityr15::R](ipriorityr15::R) reader structure"]
impl crate::Readable for IPRIORITYR15 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr15::W](ipriorityr15::W) writer structure"]
impl crate::Writable for IPRIORITYR15 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr15;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr16](ipriorityr16) module"]
pub type IPRIORITYR16 = crate::Reg<u32, _IPRIORITYR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR16;
#[doc = "`read()` method returns [ipriorityr16::R](ipriorityr16::R) reader structure"]
impl crate::Readable for IPRIORITYR16 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr16::W](ipriorityr16::W) writer structure"]
impl crate::Writable for IPRIORITYR16 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr16;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr17](ipriorityr17) module"]
pub type IPRIORITYR17 = crate::Reg<u32, _IPRIORITYR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR17;
#[doc = "`read()` method returns [ipriorityr17::R](ipriorityr17::R) reader structure"]
impl crate::Readable for IPRIORITYR17 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr17::W](ipriorityr17::W) writer structure"]
impl crate::Writable for IPRIORITYR17 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr17;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr18](ipriorityr18) module"]
pub type IPRIORITYR18 = crate::Reg<u32, _IPRIORITYR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR18;
#[doc = "`read()` method returns [ipriorityr18::R](ipriorityr18::R) reader structure"]
impl crate::Readable for IPRIORITYR18 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr18::W](ipriorityr18::W) writer structure"]
impl crate::Writable for IPRIORITYR18 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr18;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr19](ipriorityr19) module"]
pub type IPRIORITYR19 = crate::Reg<u32, _IPRIORITYR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR19;
#[doc = "`read()` method returns [ipriorityr19::R](ipriorityr19::R) reader structure"]
impl crate::Readable for IPRIORITYR19 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr19::W](ipriorityr19::W) writer structure"]
impl crate::Writable for IPRIORITYR19 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr19;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr20](ipriorityr20) module"]
pub type IPRIORITYR20 = crate::Reg<u32, _IPRIORITYR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR20;
#[doc = "`read()` method returns [ipriorityr20::R](ipriorityr20::R) reader structure"]
impl crate::Readable for IPRIORITYR20 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr20::W](ipriorityr20::W) writer structure"]
impl crate::Writable for IPRIORITYR20 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr20;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr21](ipriorityr21) module"]
pub type IPRIORITYR21 = crate::Reg<u32, _IPRIORITYR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR21;
#[doc = "`read()` method returns [ipriorityr21::R](ipriorityr21::R) reader structure"]
impl crate::Readable for IPRIORITYR21 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr21::W](ipriorityr21::W) writer structure"]
impl crate::Writable for IPRIORITYR21 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr21;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr22](ipriorityr22) module"]
pub type IPRIORITYR22 = crate::Reg<u32, _IPRIORITYR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR22;
#[doc = "`read()` method returns [ipriorityr22::R](ipriorityr22::R) reader structure"]
impl crate::Readable for IPRIORITYR22 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr22::W](ipriorityr22::W) writer structure"]
impl crate::Writable for IPRIORITYR22 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr22;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr23](ipriorityr23) module"]
pub type IPRIORITYR23 = crate::Reg<u32, _IPRIORITYR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR23;
#[doc = "`read()` method returns [ipriorityr23::R](ipriorityr23::R) reader structure"]
impl crate::Readable for IPRIORITYR23 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr23::W](ipriorityr23::W) writer structure"]
impl crate::Writable for IPRIORITYR23 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr23;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr24](ipriorityr24) module"]
pub type IPRIORITYR24 = crate::Reg<u32, _IPRIORITYR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR24;
#[doc = "`read()` method returns [ipriorityr24::R](ipriorityr24::R) reader structure"]
impl crate::Readable for IPRIORITYR24 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr24::W](ipriorityr24::W) writer structure"]
impl crate::Writable for IPRIORITYR24 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr24;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr25](ipriorityr25) module"]
pub type IPRIORITYR25 = crate::Reg<u32, _IPRIORITYR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR25;
#[doc = "`read()` method returns [ipriorityr25::R](ipriorityr25::R) reader structure"]
impl crate::Readable for IPRIORITYR25 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr25::W](ipriorityr25::W) writer structure"]
impl crate::Writable for IPRIORITYR25 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr25;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr26](ipriorityr26) module"]
pub type IPRIORITYR26 = crate::Reg<u32, _IPRIORITYR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR26;
#[doc = "`read()` method returns [ipriorityr26::R](ipriorityr26::R) reader structure"]
impl crate::Readable for IPRIORITYR26 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr26::W](ipriorityr26::W) writer structure"]
impl crate::Writable for IPRIORITYR26 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr26;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr27](ipriorityr27) module"]
pub type IPRIORITYR27 = crate::Reg<u32, _IPRIORITYR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR27;
#[doc = "`read()` method returns [ipriorityr27::R](ipriorityr27::R) reader structure"]
impl crate::Readable for IPRIORITYR27 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr27::W](ipriorityr27::W) writer structure"]
impl crate::Writable for IPRIORITYR27 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr27;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr28](ipriorityr28) module"]
pub type IPRIORITYR28 = crate::Reg<u32, _IPRIORITYR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR28;
#[doc = "`read()` method returns [ipriorityr28::R](ipriorityr28::R) reader structure"]
impl crate::Readable for IPRIORITYR28 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr28::W](ipriorityr28::W) writer structure"]
impl crate::Writable for IPRIORITYR28 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr28;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr29](ipriorityr29) module"]
pub type IPRIORITYR29 = crate::Reg<u32, _IPRIORITYR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR29;
#[doc = "`read()` method returns [ipriorityr29::R](ipriorityr29::R) reader structure"]
impl crate::Readable for IPRIORITYR29 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr29::W](ipriorityr29::W) writer structure"]
impl crate::Writable for IPRIORITYR29 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr29;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr30](ipriorityr30) module"]
pub type IPRIORITYR30 = crate::Reg<u32, _IPRIORITYR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR30;
#[doc = "`read()` method returns [ipriorityr30::R](ipriorityr30::R) reader structure"]
impl crate::Readable for IPRIORITYR30 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr30::W](ipriorityr30::W) writer structure"]
impl crate::Writable for IPRIORITYR30 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr30;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr31](ipriorityr31) module"]
pub type IPRIORITYR31 = crate::Reg<u32, _IPRIORITYR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR31;
#[doc = "`read()` method returns [ipriorityr31::R](ipriorityr31::R) reader structure"]
impl crate::Readable for IPRIORITYR31 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr31::W](ipriorityr31::W) writer structure"]
impl crate::Writable for IPRIORITYR31 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr31;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr32](ipriorityr32) module"]
pub type IPRIORITYR32 = crate::Reg<u32, _IPRIORITYR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR32;
#[doc = "`read()` method returns [ipriorityr32::R](ipriorityr32::R) reader structure"]
impl crate::Readable for IPRIORITYR32 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr32::W](ipriorityr32::W) writer structure"]
impl crate::Writable for IPRIORITYR32 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr32;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr33](ipriorityr33) module"]
pub type IPRIORITYR33 = crate::Reg<u32, _IPRIORITYR33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR33;
#[doc = "`read()` method returns [ipriorityr33::R](ipriorityr33::R) reader structure"]
impl crate::Readable for IPRIORITYR33 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr33::W](ipriorityr33::W) writer structure"]
impl crate::Writable for IPRIORITYR33 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr33;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr34](ipriorityr34) module"]
pub type IPRIORITYR34 = crate::Reg<u32, _IPRIORITYR34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR34;
#[doc = "`read()` method returns [ipriorityr34::R](ipriorityr34::R) reader structure"]
impl crate::Readable for IPRIORITYR34 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr34::W](ipriorityr34::W) writer structure"]
impl crate::Writable for IPRIORITYR34 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr34;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr35](ipriorityr35) module"]
pub type IPRIORITYR35 = crate::Reg<u32, _IPRIORITYR35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR35;
#[doc = "`read()` method returns [ipriorityr35::R](ipriorityr35::R) reader structure"]
impl crate::Readable for IPRIORITYR35 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr35::W](ipriorityr35::W) writer structure"]
impl crate::Writable for IPRIORITYR35 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr35;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr36](ipriorityr36) module"]
pub type IPRIORITYR36 = crate::Reg<u32, _IPRIORITYR36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR36;
#[doc = "`read()` method returns [ipriorityr36::R](ipriorityr36::R) reader structure"]
impl crate::Readable for IPRIORITYR36 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr36::W](ipriorityr36::W) writer structure"]
impl crate::Writable for IPRIORITYR36 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr36;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr37](ipriorityr37) module"]
pub type IPRIORITYR37 = crate::Reg<u32, _IPRIORITYR37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR37;
#[doc = "`read()` method returns [ipriorityr37::R](ipriorityr37::R) reader structure"]
impl crate::Readable for IPRIORITYR37 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr37::W](ipriorityr37::W) writer structure"]
impl crate::Writable for IPRIORITYR37 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr37;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr38](ipriorityr38) module"]
pub type IPRIORITYR38 = crate::Reg<u32, _IPRIORITYR38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR38;
#[doc = "`read()` method returns [ipriorityr38::R](ipriorityr38::R) reader structure"]
impl crate::Readable for IPRIORITYR38 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr38::W](ipriorityr38::W) writer structure"]
impl crate::Writable for IPRIORITYR38 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr38;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr39](ipriorityr39) module"]
pub type IPRIORITYR39 = crate::Reg<u32, _IPRIORITYR39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR39;
#[doc = "`read()` method returns [ipriorityr39::R](ipriorityr39::R) reader structure"]
impl crate::Readable for IPRIORITYR39 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr39::W](ipriorityr39::W) writer structure"]
impl crate::Writable for IPRIORITYR39 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr39;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr40](ipriorityr40) module"]
pub type IPRIORITYR40 = crate::Reg<u32, _IPRIORITYR40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR40;
#[doc = "`read()` method returns [ipriorityr40::R](ipriorityr40::R) reader structure"]
impl crate::Readable for IPRIORITYR40 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr40::W](ipriorityr40::W) writer structure"]
impl crate::Writable for IPRIORITYR40 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr40;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr41](ipriorityr41) module"]
pub type IPRIORITYR41 = crate::Reg<u32, _IPRIORITYR41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR41;
#[doc = "`read()` method returns [ipriorityr41::R](ipriorityr41::R) reader structure"]
impl crate::Readable for IPRIORITYR41 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr41::W](ipriorityr41::W) writer structure"]
impl crate::Writable for IPRIORITYR41 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr41;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr42](ipriorityr42) module"]
pub type IPRIORITYR42 = crate::Reg<u32, _IPRIORITYR42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR42;
#[doc = "`read()` method returns [ipriorityr42::R](ipriorityr42::R) reader structure"]
impl crate::Readable for IPRIORITYR42 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr42::W](ipriorityr42::W) writer structure"]
impl crate::Writable for IPRIORITYR42 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr42;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr43](ipriorityr43) module"]
pub type IPRIORITYR43 = crate::Reg<u32, _IPRIORITYR43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR43;
#[doc = "`read()` method returns [ipriorityr43::R](ipriorityr43::R) reader structure"]
impl crate::Readable for IPRIORITYR43 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr43::W](ipriorityr43::W) writer structure"]
impl crate::Writable for IPRIORITYR43 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr43;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr44](ipriorityr44) module"]
pub type IPRIORITYR44 = crate::Reg<u32, _IPRIORITYR44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR44;
#[doc = "`read()` method returns [ipriorityr44::R](ipriorityr44::R) reader structure"]
impl crate::Readable for IPRIORITYR44 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr44::W](ipriorityr44::W) writer structure"]
impl crate::Writable for IPRIORITYR44 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr44;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr45](ipriorityr45) module"]
pub type IPRIORITYR45 = crate::Reg<u32, _IPRIORITYR45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR45;
#[doc = "`read()` method returns [ipriorityr45::R](ipriorityr45::R) reader structure"]
impl crate::Readable for IPRIORITYR45 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr45::W](ipriorityr45::W) writer structure"]
impl crate::Writable for IPRIORITYR45 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr45;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr46](ipriorityr46) module"]
pub type IPRIORITYR46 = crate::Reg<u32, _IPRIORITYR46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR46;
#[doc = "`read()` method returns [ipriorityr46::R](ipriorityr46::R) reader structure"]
impl crate::Readable for IPRIORITYR46 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr46::W](ipriorityr46::W) writer structure"]
impl crate::Writable for IPRIORITYR46 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr46;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr47](ipriorityr47) module"]
pub type IPRIORITYR47 = crate::Reg<u32, _IPRIORITYR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR47;
#[doc = "`read()` method returns [ipriorityr47::R](ipriorityr47::R) reader structure"]
impl crate::Readable for IPRIORITYR47 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr47::W](ipriorityr47::W) writer structure"]
impl crate::Writable for IPRIORITYR47 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr47;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr48](ipriorityr48) module"]
pub type IPRIORITYR48 = crate::Reg<u32, _IPRIORITYR48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR48;
#[doc = "`read()` method returns [ipriorityr48::R](ipriorityr48::R) reader structure"]
impl crate::Readable for IPRIORITYR48 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr48::W](ipriorityr48::W) writer structure"]
impl crate::Writable for IPRIORITYR48 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr48;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr49](ipriorityr49) module"]
pub type IPRIORITYR49 = crate::Reg<u32, _IPRIORITYR49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR49;
#[doc = "`read()` method returns [ipriorityr49::R](ipriorityr49::R) reader structure"]
impl crate::Readable for IPRIORITYR49 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr49::W](ipriorityr49::W) writer structure"]
impl crate::Writable for IPRIORITYR49 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr49;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr50](ipriorityr50) module"]
pub type IPRIORITYR50 = crate::Reg<u32, _IPRIORITYR50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR50;
#[doc = "`read()` method returns [ipriorityr50::R](ipriorityr50::R) reader structure"]
impl crate::Readable for IPRIORITYR50 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr50::W](ipriorityr50::W) writer structure"]
impl crate::Writable for IPRIORITYR50 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr50;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr51](ipriorityr51) module"]
pub type IPRIORITYR51 = crate::Reg<u32, _IPRIORITYR51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR51;
#[doc = "`read()` method returns [ipriorityr51::R](ipriorityr51::R) reader structure"]
impl crate::Readable for IPRIORITYR51 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr51::W](ipriorityr51::W) writer structure"]
impl crate::Writable for IPRIORITYR51 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr51;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr52](ipriorityr52) module"]
pub type IPRIORITYR52 = crate::Reg<u32, _IPRIORITYR52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR52;
#[doc = "`read()` method returns [ipriorityr52::R](ipriorityr52::R) reader structure"]
impl crate::Readable for IPRIORITYR52 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr52::W](ipriorityr52::W) writer structure"]
impl crate::Writable for IPRIORITYR52 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr52;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr53](ipriorityr53) module"]
pub type IPRIORITYR53 = crate::Reg<u32, _IPRIORITYR53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR53;
#[doc = "`read()` method returns [ipriorityr53::R](ipriorityr53::R) reader structure"]
impl crate::Readable for IPRIORITYR53 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr53::W](ipriorityr53::W) writer structure"]
impl crate::Writable for IPRIORITYR53 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr53;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr54](ipriorityr54) module"]
pub type IPRIORITYR54 = crate::Reg<u32, _IPRIORITYR54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR54;
#[doc = "`read()` method returns [ipriorityr54::R](ipriorityr54::R) reader structure"]
impl crate::Readable for IPRIORITYR54 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr54::W](ipriorityr54::W) writer structure"]
impl crate::Writable for IPRIORITYR54 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr54;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr55](ipriorityr55) module"]
pub type IPRIORITYR55 = crate::Reg<u32, _IPRIORITYR55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR55;
#[doc = "`read()` method returns [ipriorityr55::R](ipriorityr55::R) reader structure"]
impl crate::Readable for IPRIORITYR55 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr55::W](ipriorityr55::W) writer structure"]
impl crate::Writable for IPRIORITYR55 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr55;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr56](ipriorityr56) module"]
pub type IPRIORITYR56 = crate::Reg<u32, _IPRIORITYR56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR56;
#[doc = "`read()` method returns [ipriorityr56::R](ipriorityr56::R) reader structure"]
impl crate::Readable for IPRIORITYR56 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr56::W](ipriorityr56::W) writer structure"]
impl crate::Writable for IPRIORITYR56 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr56;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr57](ipriorityr57) module"]
pub type IPRIORITYR57 = crate::Reg<u32, _IPRIORITYR57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR57;
#[doc = "`read()` method returns [ipriorityr57::R](ipriorityr57::R) reader structure"]
impl crate::Readable for IPRIORITYR57 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr57::W](ipriorityr57::W) writer structure"]
impl crate::Writable for IPRIORITYR57 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr57;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr58](ipriorityr58) module"]
pub type IPRIORITYR58 = crate::Reg<u32, _IPRIORITYR58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR58;
#[doc = "`read()` method returns [ipriorityr58::R](ipriorityr58::R) reader structure"]
impl crate::Readable for IPRIORITYR58 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr58::W](ipriorityr58::W) writer structure"]
impl crate::Writable for IPRIORITYR58 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr58;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr59](ipriorityr59) module"]
pub type IPRIORITYR59 = crate::Reg<u32, _IPRIORITYR59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR59;
#[doc = "`read()` method returns [ipriorityr59::R](ipriorityr59::R) reader structure"]
impl crate::Readable for IPRIORITYR59 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr59::W](ipriorityr59::W) writer structure"]
impl crate::Writable for IPRIORITYR59 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr59;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr60](ipriorityr60) module"]
pub type IPRIORITYR60 = crate::Reg<u32, _IPRIORITYR60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR60;
#[doc = "`read()` method returns [ipriorityr60::R](ipriorityr60::R) reader structure"]
impl crate::Readable for IPRIORITYR60 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr60::W](ipriorityr60::W) writer structure"]
impl crate::Writable for IPRIORITYR60 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr60;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr61](ipriorityr61) module"]
pub type IPRIORITYR61 = crate::Reg<u32, _IPRIORITYR61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR61;
#[doc = "`read()` method returns [ipriorityr61::R](ipriorityr61::R) reader structure"]
impl crate::Readable for IPRIORITYR61 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr61::W](ipriorityr61::W) writer structure"]
impl crate::Writable for IPRIORITYR61 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr61;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr62](ipriorityr62) module"]
pub type IPRIORITYR62 = crate::Reg<u32, _IPRIORITYR62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR62;
#[doc = "`read()` method returns [ipriorityr62::R](ipriorityr62::R) reader structure"]
impl crate::Readable for IPRIORITYR62 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr62::W](ipriorityr62::W) writer structure"]
impl crate::Writable for IPRIORITYR62 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr62;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr63](ipriorityr63) module"]
pub type IPRIORITYR63 = crate::Reg<u32, _IPRIORITYR63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR63;
#[doc = "`read()` method returns [ipriorityr63::R](ipriorityr63::R) reader structure"]
impl crate::Readable for IPRIORITYR63 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr63::W](ipriorityr63::W) writer structure"]
impl crate::Writable for IPRIORITYR63 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr63;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr64](ipriorityr64) module"]
pub type IPRIORITYR64 = crate::Reg<u32, _IPRIORITYR64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR64;
#[doc = "`read()` method returns [ipriorityr64::R](ipriorityr64::R) reader structure"]
impl crate::Readable for IPRIORITYR64 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr64::W](ipriorityr64::W) writer structure"]
impl crate::Writable for IPRIORITYR64 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr64;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr65](ipriorityr65) module"]
pub type IPRIORITYR65 = crate::Reg<u32, _IPRIORITYR65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR65;
#[doc = "`read()` method returns [ipriorityr65::R](ipriorityr65::R) reader structure"]
impl crate::Readable for IPRIORITYR65 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr65::W](ipriorityr65::W) writer structure"]
impl crate::Writable for IPRIORITYR65 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr65;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr66](ipriorityr66) module"]
pub type IPRIORITYR66 = crate::Reg<u32, _IPRIORITYR66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR66;
#[doc = "`read()` method returns [ipriorityr66::R](ipriorityr66::R) reader structure"]
impl crate::Readable for IPRIORITYR66 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr66::W](ipriorityr66::W) writer structure"]
impl crate::Writable for IPRIORITYR66 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr66;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr67](ipriorityr67) module"]
pub type IPRIORITYR67 = crate::Reg<u32, _IPRIORITYR67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR67;
#[doc = "`read()` method returns [ipriorityr67::R](ipriorityr67::R) reader structure"]
impl crate::Readable for IPRIORITYR67 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr67::W](ipriorityr67::W) writer structure"]
impl crate::Writable for IPRIORITYR67 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr67;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr68](ipriorityr68) module"]
pub type IPRIORITYR68 = crate::Reg<u32, _IPRIORITYR68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR68;
#[doc = "`read()` method returns [ipriorityr68::R](ipriorityr68::R) reader structure"]
impl crate::Readable for IPRIORITYR68 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr68::W](ipriorityr68::W) writer structure"]
impl crate::Writable for IPRIORITYR68 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr68;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr69](ipriorityr69) module"]
pub type IPRIORITYR69 = crate::Reg<u32, _IPRIORITYR69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR69;
#[doc = "`read()` method returns [ipriorityr69::R](ipriorityr69::R) reader structure"]
impl crate::Readable for IPRIORITYR69 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr69::W](ipriorityr69::W) writer structure"]
impl crate::Writable for IPRIORITYR69 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr69;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr70](ipriorityr70) module"]
pub type IPRIORITYR70 = crate::Reg<u32, _IPRIORITYR70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR70;
#[doc = "`read()` method returns [ipriorityr70::R](ipriorityr70::R) reader structure"]
impl crate::Readable for IPRIORITYR70 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr70::W](ipriorityr70::W) writer structure"]
impl crate::Writable for IPRIORITYR70 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr70;
#[doc = "GICD interrupt priority registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipriorityr71](ipriorityr71) module"]
pub type IPRIORITYR71 = crate::Reg<u32, _IPRIORITYR71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRIORITYR71;
#[doc = "`read()` method returns [ipriorityr71::R](ipriorityr71::R) reader structure"]
impl crate::Readable for IPRIORITYR71 {}
#[doc = "`write(|w| ..)` method takes [ipriorityr71::W](ipriorityr71::W) writer structure"]
impl crate::Writable for IPRIORITYR71 {}
#[doc = "GICD interrupt priority registers"]
pub mod ipriorityr71;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr0](itargetsr0) module"]
pub type ITARGETSR0 = crate::Reg<u32, _ITARGETSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR0;
#[doc = "`read()` method returns [itargetsr0::R](itargetsr0::R) reader structure"]
impl crate::Readable for ITARGETSR0 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr0;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr1](itargetsr1) module"]
pub type ITARGETSR1 = crate::Reg<u32, _ITARGETSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR1;
#[doc = "`read()` method returns [itargetsr1::R](itargetsr1::R) reader structure"]
impl crate::Readable for ITARGETSR1 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr1;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr2](itargetsr2) module"]
pub type ITARGETSR2 = crate::Reg<u32, _ITARGETSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR2;
#[doc = "`read()` method returns [itargetsr2::R](itargetsr2::R) reader structure"]
impl crate::Readable for ITARGETSR2 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr2;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr3](itargetsr3) module"]
pub type ITARGETSR3 = crate::Reg<u32, _ITARGETSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR3;
#[doc = "`read()` method returns [itargetsr3::R](itargetsr3::R) reader structure"]
impl crate::Readable for ITARGETSR3 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr3;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr4](itargetsr4) module"]
pub type ITARGETSR4 = crate::Reg<u32, _ITARGETSR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR4;
#[doc = "`read()` method returns [itargetsr4::R](itargetsr4::R) reader structure"]
impl crate::Readable for ITARGETSR4 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr4;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr5](itargetsr5) module"]
pub type ITARGETSR5 = crate::Reg<u32, _ITARGETSR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR5;
#[doc = "`read()` method returns [itargetsr5::R](itargetsr5::R) reader structure"]
impl crate::Readable for ITARGETSR5 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr5;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr6](itargetsr6) module"]
pub type ITARGETSR6 = crate::Reg<u32, _ITARGETSR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR6;
#[doc = "`read()` method returns [itargetsr6::R](itargetsr6::R) reader structure"]
impl crate::Readable for ITARGETSR6 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr6;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr7](itargetsr7) module"]
pub type ITARGETSR7 = crate::Reg<u32, _ITARGETSR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR7;
#[doc = "`read()` method returns [itargetsr7::R](itargetsr7::R) reader structure"]
impl crate::Readable for ITARGETSR7 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr7;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr8](itargetsr8) module"]
pub type ITARGETSR8 = crate::Reg<u32, _ITARGETSR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR8;
#[doc = "`read()` method returns [itargetsr8::R](itargetsr8::R) reader structure"]
impl crate::Readable for ITARGETSR8 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr8;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr9](itargetsr9) module"]
pub type ITARGETSR9 = crate::Reg<u32, _ITARGETSR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR9;
#[doc = "`read()` method returns [itargetsr9::R](itargetsr9::R) reader structure"]
impl crate::Readable for ITARGETSR9 {}
#[doc = "`write(|w| ..)` method takes [itargetsr9::W](itargetsr9::W) writer structure"]
impl crate::Writable for ITARGETSR9 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr9;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr10](itargetsr10) module"]
pub type ITARGETSR10 = crate::Reg<u32, _ITARGETSR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR10;
#[doc = "`read()` method returns [itargetsr10::R](itargetsr10::R) reader structure"]
impl crate::Readable for ITARGETSR10 {}
#[doc = "`write(|w| ..)` method takes [itargetsr10::W](itargetsr10::W) writer structure"]
impl crate::Writable for ITARGETSR10 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr10;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr11](itargetsr11) module"]
pub type ITARGETSR11 = crate::Reg<u32, _ITARGETSR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR11;
#[doc = "`read()` method returns [itargetsr11::R](itargetsr11::R) reader structure"]
impl crate::Readable for ITARGETSR11 {}
#[doc = "`write(|w| ..)` method takes [itargetsr11::W](itargetsr11::W) writer structure"]
impl crate::Writable for ITARGETSR11 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr11;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr12](itargetsr12) module"]
pub type ITARGETSR12 = crate::Reg<u32, _ITARGETSR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR12;
#[doc = "`read()` method returns [itargetsr12::R](itargetsr12::R) reader structure"]
impl crate::Readable for ITARGETSR12 {}
#[doc = "`write(|w| ..)` method takes [itargetsr12::W](itargetsr12::W) writer structure"]
impl crate::Writable for ITARGETSR12 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr12;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr13](itargetsr13) module"]
pub type ITARGETSR13 = crate::Reg<u32, _ITARGETSR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR13;
#[doc = "`read()` method returns [itargetsr13::R](itargetsr13::R) reader structure"]
impl crate::Readable for ITARGETSR13 {}
#[doc = "`write(|w| ..)` method takes [itargetsr13::W](itargetsr13::W) writer structure"]
impl crate::Writable for ITARGETSR13 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr13;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr14](itargetsr14) module"]
pub type ITARGETSR14 = crate::Reg<u32, _ITARGETSR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR14;
#[doc = "`read()` method returns [itargetsr14::R](itargetsr14::R) reader structure"]
impl crate::Readable for ITARGETSR14 {}
#[doc = "`write(|w| ..)` method takes [itargetsr14::W](itargetsr14::W) writer structure"]
impl crate::Writable for ITARGETSR14 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr14;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr15](itargetsr15) module"]
pub type ITARGETSR15 = crate::Reg<u32, _ITARGETSR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR15;
#[doc = "`read()` method returns [itargetsr15::R](itargetsr15::R) reader structure"]
impl crate::Readable for ITARGETSR15 {}
#[doc = "`write(|w| ..)` method takes [itargetsr15::W](itargetsr15::W) writer structure"]
impl crate::Writable for ITARGETSR15 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr15;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr16](itargetsr16) module"]
pub type ITARGETSR16 = crate::Reg<u32, _ITARGETSR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR16;
#[doc = "`read()` method returns [itargetsr16::R](itargetsr16::R) reader structure"]
impl crate::Readable for ITARGETSR16 {}
#[doc = "`write(|w| ..)` method takes [itargetsr16::W](itargetsr16::W) writer structure"]
impl crate::Writable for ITARGETSR16 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr16;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr17](itargetsr17) module"]
pub type ITARGETSR17 = crate::Reg<u32, _ITARGETSR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR17;
#[doc = "`read()` method returns [itargetsr17::R](itargetsr17::R) reader structure"]
impl crate::Readable for ITARGETSR17 {}
#[doc = "`write(|w| ..)` method takes [itargetsr17::W](itargetsr17::W) writer structure"]
impl crate::Writable for ITARGETSR17 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr17;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr18](itargetsr18) module"]
pub type ITARGETSR18 = crate::Reg<u32, _ITARGETSR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR18;
#[doc = "`read()` method returns [itargetsr18::R](itargetsr18::R) reader structure"]
impl crate::Readable for ITARGETSR18 {}
#[doc = "`write(|w| ..)` method takes [itargetsr18::W](itargetsr18::W) writer structure"]
impl crate::Writable for ITARGETSR18 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr18;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr19](itargetsr19) module"]
pub type ITARGETSR19 = crate::Reg<u32, _ITARGETSR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR19;
#[doc = "`read()` method returns [itargetsr19::R](itargetsr19::R) reader structure"]
impl crate::Readable for ITARGETSR19 {}
#[doc = "`write(|w| ..)` method takes [itargetsr19::W](itargetsr19::W) writer structure"]
impl crate::Writable for ITARGETSR19 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr19;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr20](itargetsr20) module"]
pub type ITARGETSR20 = crate::Reg<u32, _ITARGETSR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR20;
#[doc = "`read()` method returns [itargetsr20::R](itargetsr20::R) reader structure"]
impl crate::Readable for ITARGETSR20 {}
#[doc = "`write(|w| ..)` method takes [itargetsr20::W](itargetsr20::W) writer structure"]
impl crate::Writable for ITARGETSR20 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr20;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr21](itargetsr21) module"]
pub type ITARGETSR21 = crate::Reg<u32, _ITARGETSR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR21;
#[doc = "`read()` method returns [itargetsr21::R](itargetsr21::R) reader structure"]
impl crate::Readable for ITARGETSR21 {}
#[doc = "`write(|w| ..)` method takes [itargetsr21::W](itargetsr21::W) writer structure"]
impl crate::Writable for ITARGETSR21 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr21;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr22](itargetsr22) module"]
pub type ITARGETSR22 = crate::Reg<u32, _ITARGETSR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR22;
#[doc = "`read()` method returns [itargetsr22::R](itargetsr22::R) reader structure"]
impl crate::Readable for ITARGETSR22 {}
#[doc = "`write(|w| ..)` method takes [itargetsr22::W](itargetsr22::W) writer structure"]
impl crate::Writable for ITARGETSR22 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr22;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr23](itargetsr23) module"]
pub type ITARGETSR23 = crate::Reg<u32, _ITARGETSR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR23;
#[doc = "`read()` method returns [itargetsr23::R](itargetsr23::R) reader structure"]
impl crate::Readable for ITARGETSR23 {}
#[doc = "`write(|w| ..)` method takes [itargetsr23::W](itargetsr23::W) writer structure"]
impl crate::Writable for ITARGETSR23 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr23;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr24](itargetsr24) module"]
pub type ITARGETSR24 = crate::Reg<u32, _ITARGETSR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR24;
#[doc = "`read()` method returns [itargetsr24::R](itargetsr24::R) reader structure"]
impl crate::Readable for ITARGETSR24 {}
#[doc = "`write(|w| ..)` method takes [itargetsr24::W](itargetsr24::W) writer structure"]
impl crate::Writable for ITARGETSR24 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr24;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr25](itargetsr25) module"]
pub type ITARGETSR25 = crate::Reg<u32, _ITARGETSR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR25;
#[doc = "`read()` method returns [itargetsr25::R](itargetsr25::R) reader structure"]
impl crate::Readable for ITARGETSR25 {}
#[doc = "`write(|w| ..)` method takes [itargetsr25::W](itargetsr25::W) writer structure"]
impl crate::Writable for ITARGETSR25 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr25;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr26](itargetsr26) module"]
pub type ITARGETSR26 = crate::Reg<u32, _ITARGETSR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR26;
#[doc = "`read()` method returns [itargetsr26::R](itargetsr26::R) reader structure"]
impl crate::Readable for ITARGETSR26 {}
#[doc = "`write(|w| ..)` method takes [itargetsr26::W](itargetsr26::W) writer structure"]
impl crate::Writable for ITARGETSR26 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr26;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr27](itargetsr27) module"]
pub type ITARGETSR27 = crate::Reg<u32, _ITARGETSR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR27;
#[doc = "`read()` method returns [itargetsr27::R](itargetsr27::R) reader structure"]
impl crate::Readable for ITARGETSR27 {}
#[doc = "`write(|w| ..)` method takes [itargetsr27::W](itargetsr27::W) writer structure"]
impl crate::Writable for ITARGETSR27 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr27;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr28](itargetsr28) module"]
pub type ITARGETSR28 = crate::Reg<u32, _ITARGETSR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR28;
#[doc = "`read()` method returns [itargetsr28::R](itargetsr28::R) reader structure"]
impl crate::Readable for ITARGETSR28 {}
#[doc = "`write(|w| ..)` method takes [itargetsr28::W](itargetsr28::W) writer structure"]
impl crate::Writable for ITARGETSR28 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr28;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr29](itargetsr29) module"]
pub type ITARGETSR29 = crate::Reg<u32, _ITARGETSR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR29;
#[doc = "`read()` method returns [itargetsr29::R](itargetsr29::R) reader structure"]
impl crate::Readable for ITARGETSR29 {}
#[doc = "`write(|w| ..)` method takes [itargetsr29::W](itargetsr29::W) writer structure"]
impl crate::Writable for ITARGETSR29 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr29;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr30](itargetsr30) module"]
pub type ITARGETSR30 = crate::Reg<u32, _ITARGETSR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR30;
#[doc = "`read()` method returns [itargetsr30::R](itargetsr30::R) reader structure"]
impl crate::Readable for ITARGETSR30 {}
#[doc = "`write(|w| ..)` method takes [itargetsr30::W](itargetsr30::W) writer structure"]
impl crate::Writable for ITARGETSR30 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr30;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr31](itargetsr31) module"]
pub type ITARGETSR31 = crate::Reg<u32, _ITARGETSR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR31;
#[doc = "`read()` method returns [itargetsr31::R](itargetsr31::R) reader structure"]
impl crate::Readable for ITARGETSR31 {}
#[doc = "`write(|w| ..)` method takes [itargetsr31::W](itargetsr31::W) writer structure"]
impl crate::Writable for ITARGETSR31 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr31;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr32](itargetsr32) module"]
pub type ITARGETSR32 = crate::Reg<u32, _ITARGETSR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR32;
#[doc = "`read()` method returns [itargetsr32::R](itargetsr32::R) reader structure"]
impl crate::Readable for ITARGETSR32 {}
#[doc = "`write(|w| ..)` method takes [itargetsr32::W](itargetsr32::W) writer structure"]
impl crate::Writable for ITARGETSR32 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr32;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr33](itargetsr33) module"]
pub type ITARGETSR33 = crate::Reg<u32, _ITARGETSR33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR33;
#[doc = "`read()` method returns [itargetsr33::R](itargetsr33::R) reader structure"]
impl crate::Readable for ITARGETSR33 {}
#[doc = "`write(|w| ..)` method takes [itargetsr33::W](itargetsr33::W) writer structure"]
impl crate::Writable for ITARGETSR33 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr33;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr34](itargetsr34) module"]
pub type ITARGETSR34 = crate::Reg<u32, _ITARGETSR34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR34;
#[doc = "`read()` method returns [itargetsr34::R](itargetsr34::R) reader structure"]
impl crate::Readable for ITARGETSR34 {}
#[doc = "`write(|w| ..)` method takes [itargetsr34::W](itargetsr34::W) writer structure"]
impl crate::Writable for ITARGETSR34 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr34;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr35](itargetsr35) module"]
pub type ITARGETSR35 = crate::Reg<u32, _ITARGETSR35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR35;
#[doc = "`read()` method returns [itargetsr35::R](itargetsr35::R) reader structure"]
impl crate::Readable for ITARGETSR35 {}
#[doc = "`write(|w| ..)` method takes [itargetsr35::W](itargetsr35::W) writer structure"]
impl crate::Writable for ITARGETSR35 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr35;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr36](itargetsr36) module"]
pub type ITARGETSR36 = crate::Reg<u32, _ITARGETSR36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR36;
#[doc = "`read()` method returns [itargetsr36::R](itargetsr36::R) reader structure"]
impl crate::Readable for ITARGETSR36 {}
#[doc = "`write(|w| ..)` method takes [itargetsr36::W](itargetsr36::W) writer structure"]
impl crate::Writable for ITARGETSR36 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr36;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr37](itargetsr37) module"]
pub type ITARGETSR37 = crate::Reg<u32, _ITARGETSR37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR37;
#[doc = "`read()` method returns [itargetsr37::R](itargetsr37::R) reader structure"]
impl crate::Readable for ITARGETSR37 {}
#[doc = "`write(|w| ..)` method takes [itargetsr37::W](itargetsr37::W) writer structure"]
impl crate::Writable for ITARGETSR37 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr37;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr38](itargetsr38) module"]
pub type ITARGETSR38 = crate::Reg<u32, _ITARGETSR38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR38;
#[doc = "`read()` method returns [itargetsr38::R](itargetsr38::R) reader structure"]
impl crate::Readable for ITARGETSR38 {}
#[doc = "`write(|w| ..)` method takes [itargetsr38::W](itargetsr38::W) writer structure"]
impl crate::Writable for ITARGETSR38 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr38;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr39](itargetsr39) module"]
pub type ITARGETSR39 = crate::Reg<u32, _ITARGETSR39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR39;
#[doc = "`read()` method returns [itargetsr39::R](itargetsr39::R) reader structure"]
impl crate::Readable for ITARGETSR39 {}
#[doc = "`write(|w| ..)` method takes [itargetsr39::W](itargetsr39::W) writer structure"]
impl crate::Writable for ITARGETSR39 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr39;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr40](itargetsr40) module"]
pub type ITARGETSR40 = crate::Reg<u32, _ITARGETSR40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR40;
#[doc = "`read()` method returns [itargetsr40::R](itargetsr40::R) reader structure"]
impl crate::Readable for ITARGETSR40 {}
#[doc = "`write(|w| ..)` method takes [itargetsr40::W](itargetsr40::W) writer structure"]
impl crate::Writable for ITARGETSR40 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr40;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr41](itargetsr41) module"]
pub type ITARGETSR41 = crate::Reg<u32, _ITARGETSR41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR41;
#[doc = "`read()` method returns [itargetsr41::R](itargetsr41::R) reader structure"]
impl crate::Readable for ITARGETSR41 {}
#[doc = "`write(|w| ..)` method takes [itargetsr41::W](itargetsr41::W) writer structure"]
impl crate::Writable for ITARGETSR41 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr41;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr42](itargetsr42) module"]
pub type ITARGETSR42 = crate::Reg<u32, _ITARGETSR42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR42;
#[doc = "`read()` method returns [itargetsr42::R](itargetsr42::R) reader structure"]
impl crate::Readable for ITARGETSR42 {}
#[doc = "`write(|w| ..)` method takes [itargetsr42::W](itargetsr42::W) writer structure"]
impl crate::Writable for ITARGETSR42 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr42;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr43](itargetsr43) module"]
pub type ITARGETSR43 = crate::Reg<u32, _ITARGETSR43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR43;
#[doc = "`read()` method returns [itargetsr43::R](itargetsr43::R) reader structure"]
impl crate::Readable for ITARGETSR43 {}
#[doc = "`write(|w| ..)` method takes [itargetsr43::W](itargetsr43::W) writer structure"]
impl crate::Writable for ITARGETSR43 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr43;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr44](itargetsr44) module"]
pub type ITARGETSR44 = crate::Reg<u32, _ITARGETSR44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR44;
#[doc = "`read()` method returns [itargetsr44::R](itargetsr44::R) reader structure"]
impl crate::Readable for ITARGETSR44 {}
#[doc = "`write(|w| ..)` method takes [itargetsr44::W](itargetsr44::W) writer structure"]
impl crate::Writable for ITARGETSR44 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr44;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr45](itargetsr45) module"]
pub type ITARGETSR45 = crate::Reg<u32, _ITARGETSR45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR45;
#[doc = "`read()` method returns [itargetsr45::R](itargetsr45::R) reader structure"]
impl crate::Readable for ITARGETSR45 {}
#[doc = "`write(|w| ..)` method takes [itargetsr45::W](itargetsr45::W) writer structure"]
impl crate::Writable for ITARGETSR45 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr45;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr46](itargetsr46) module"]
pub type ITARGETSR46 = crate::Reg<u32, _ITARGETSR46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR46;
#[doc = "`read()` method returns [itargetsr46::R](itargetsr46::R) reader structure"]
impl crate::Readable for ITARGETSR46 {}
#[doc = "`write(|w| ..)` method takes [itargetsr46::W](itargetsr46::W) writer structure"]
impl crate::Writable for ITARGETSR46 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr46;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr47](itargetsr47) module"]
pub type ITARGETSR47 = crate::Reg<u32, _ITARGETSR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR47;
#[doc = "`read()` method returns [itargetsr47::R](itargetsr47::R) reader structure"]
impl crate::Readable for ITARGETSR47 {}
#[doc = "`write(|w| ..)` method takes [itargetsr47::W](itargetsr47::W) writer structure"]
impl crate::Writable for ITARGETSR47 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr47;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr48](itargetsr48) module"]
pub type ITARGETSR48 = crate::Reg<u32, _ITARGETSR48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR48;
#[doc = "`read()` method returns [itargetsr48::R](itargetsr48::R) reader structure"]
impl crate::Readable for ITARGETSR48 {}
#[doc = "`write(|w| ..)` method takes [itargetsr48::W](itargetsr48::W) writer structure"]
impl crate::Writable for ITARGETSR48 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr48;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr49](itargetsr49) module"]
pub type ITARGETSR49 = crate::Reg<u32, _ITARGETSR49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR49;
#[doc = "`read()` method returns [itargetsr49::R](itargetsr49::R) reader structure"]
impl crate::Readable for ITARGETSR49 {}
#[doc = "`write(|w| ..)` method takes [itargetsr49::W](itargetsr49::W) writer structure"]
impl crate::Writable for ITARGETSR49 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr49;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr50](itargetsr50) module"]
pub type ITARGETSR50 = crate::Reg<u32, _ITARGETSR50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR50;
#[doc = "`read()` method returns [itargetsr50::R](itargetsr50::R) reader structure"]
impl crate::Readable for ITARGETSR50 {}
#[doc = "`write(|w| ..)` method takes [itargetsr50::W](itargetsr50::W) writer structure"]
impl crate::Writable for ITARGETSR50 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr50;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr51](itargetsr51) module"]
pub type ITARGETSR51 = crate::Reg<u32, _ITARGETSR51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR51;
#[doc = "`read()` method returns [itargetsr51::R](itargetsr51::R) reader structure"]
impl crate::Readable for ITARGETSR51 {}
#[doc = "`write(|w| ..)` method takes [itargetsr51::W](itargetsr51::W) writer structure"]
impl crate::Writable for ITARGETSR51 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr51;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr52](itargetsr52) module"]
pub type ITARGETSR52 = crate::Reg<u32, _ITARGETSR52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR52;
#[doc = "`read()` method returns [itargetsr52::R](itargetsr52::R) reader structure"]
impl crate::Readable for ITARGETSR52 {}
#[doc = "`write(|w| ..)` method takes [itargetsr52::W](itargetsr52::W) writer structure"]
impl crate::Writable for ITARGETSR52 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr52;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr53](itargetsr53) module"]
pub type ITARGETSR53 = crate::Reg<u32, _ITARGETSR53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR53;
#[doc = "`read()` method returns [itargetsr53::R](itargetsr53::R) reader structure"]
impl crate::Readable for ITARGETSR53 {}
#[doc = "`write(|w| ..)` method takes [itargetsr53::W](itargetsr53::W) writer structure"]
impl crate::Writable for ITARGETSR53 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr53;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr54](itargetsr54) module"]
pub type ITARGETSR54 = crate::Reg<u32, _ITARGETSR54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR54;
#[doc = "`read()` method returns [itargetsr54::R](itargetsr54::R) reader structure"]
impl crate::Readable for ITARGETSR54 {}
#[doc = "`write(|w| ..)` method takes [itargetsr54::W](itargetsr54::W) writer structure"]
impl crate::Writable for ITARGETSR54 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr54;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr55](itargetsr55) module"]
pub type ITARGETSR55 = crate::Reg<u32, _ITARGETSR55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR55;
#[doc = "`read()` method returns [itargetsr55::R](itargetsr55::R) reader structure"]
impl crate::Readable for ITARGETSR55 {}
#[doc = "`write(|w| ..)` method takes [itargetsr55::W](itargetsr55::W) writer structure"]
impl crate::Writable for ITARGETSR55 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr55;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr56](itargetsr56) module"]
pub type ITARGETSR56 = crate::Reg<u32, _ITARGETSR56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR56;
#[doc = "`read()` method returns [itargetsr56::R](itargetsr56::R) reader structure"]
impl crate::Readable for ITARGETSR56 {}
#[doc = "`write(|w| ..)` method takes [itargetsr56::W](itargetsr56::W) writer structure"]
impl crate::Writable for ITARGETSR56 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr56;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr57](itargetsr57) module"]
pub type ITARGETSR57 = crate::Reg<u32, _ITARGETSR57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR57;
#[doc = "`read()` method returns [itargetsr57::R](itargetsr57::R) reader structure"]
impl crate::Readable for ITARGETSR57 {}
#[doc = "`write(|w| ..)` method takes [itargetsr57::W](itargetsr57::W) writer structure"]
impl crate::Writable for ITARGETSR57 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr57;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr58](itargetsr58) module"]
pub type ITARGETSR58 = crate::Reg<u32, _ITARGETSR58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR58;
#[doc = "`read()` method returns [itargetsr58::R](itargetsr58::R) reader structure"]
impl crate::Readable for ITARGETSR58 {}
#[doc = "`write(|w| ..)` method takes [itargetsr58::W](itargetsr58::W) writer structure"]
impl crate::Writable for ITARGETSR58 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr58;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr59](itargetsr59) module"]
pub type ITARGETSR59 = crate::Reg<u32, _ITARGETSR59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR59;
#[doc = "`read()` method returns [itargetsr59::R](itargetsr59::R) reader structure"]
impl crate::Readable for ITARGETSR59 {}
#[doc = "`write(|w| ..)` method takes [itargetsr59::W](itargetsr59::W) writer structure"]
impl crate::Writable for ITARGETSR59 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr59;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr60](itargetsr60) module"]
pub type ITARGETSR60 = crate::Reg<u32, _ITARGETSR60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR60;
#[doc = "`read()` method returns [itargetsr60::R](itargetsr60::R) reader structure"]
impl crate::Readable for ITARGETSR60 {}
#[doc = "`write(|w| ..)` method takes [itargetsr60::W](itargetsr60::W) writer structure"]
impl crate::Writable for ITARGETSR60 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr60;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr61](itargetsr61) module"]
pub type ITARGETSR61 = crate::Reg<u32, _ITARGETSR61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR61;
#[doc = "`read()` method returns [itargetsr61::R](itargetsr61::R) reader structure"]
impl crate::Readable for ITARGETSR61 {}
#[doc = "`write(|w| ..)` method takes [itargetsr61::W](itargetsr61::W) writer structure"]
impl crate::Writable for ITARGETSR61 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr61;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr62](itargetsr62) module"]
pub type ITARGETSR62 = crate::Reg<u32, _ITARGETSR62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR62;
#[doc = "`read()` method returns [itargetsr62::R](itargetsr62::R) reader structure"]
impl crate::Readable for ITARGETSR62 {}
#[doc = "`write(|w| ..)` method takes [itargetsr62::W](itargetsr62::W) writer structure"]
impl crate::Writable for ITARGETSR62 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr62;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr63](itargetsr63) module"]
pub type ITARGETSR63 = crate::Reg<u32, _ITARGETSR63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR63;
#[doc = "`read()` method returns [itargetsr63::R](itargetsr63::R) reader structure"]
impl crate::Readable for ITARGETSR63 {}
#[doc = "`write(|w| ..)` method takes [itargetsr63::W](itargetsr63::W) writer structure"]
impl crate::Writable for ITARGETSR63 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr63;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr64](itargetsr64) module"]
pub type ITARGETSR64 = crate::Reg<u32, _ITARGETSR64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR64;
#[doc = "`read()` method returns [itargetsr64::R](itargetsr64::R) reader structure"]
impl crate::Readable for ITARGETSR64 {}
#[doc = "`write(|w| ..)` method takes [itargetsr64::W](itargetsr64::W) writer structure"]
impl crate::Writable for ITARGETSR64 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr64;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr65](itargetsr65) module"]
pub type ITARGETSR65 = crate::Reg<u32, _ITARGETSR65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR65;
#[doc = "`read()` method returns [itargetsr65::R](itargetsr65::R) reader structure"]
impl crate::Readable for ITARGETSR65 {}
#[doc = "`write(|w| ..)` method takes [itargetsr65::W](itargetsr65::W) writer structure"]
impl crate::Writable for ITARGETSR65 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr65;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr66](itargetsr66) module"]
pub type ITARGETSR66 = crate::Reg<u32, _ITARGETSR66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR66;
#[doc = "`read()` method returns [itargetsr66::R](itargetsr66::R) reader structure"]
impl crate::Readable for ITARGETSR66 {}
#[doc = "`write(|w| ..)` method takes [itargetsr66::W](itargetsr66::W) writer structure"]
impl crate::Writable for ITARGETSR66 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr66;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr67](itargetsr67) module"]
pub type ITARGETSR67 = crate::Reg<u32, _ITARGETSR67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR67;
#[doc = "`read()` method returns [itargetsr67::R](itargetsr67::R) reader structure"]
impl crate::Readable for ITARGETSR67 {}
#[doc = "`write(|w| ..)` method takes [itargetsr67::W](itargetsr67::W) writer structure"]
impl crate::Writable for ITARGETSR67 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr67;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr68](itargetsr68) module"]
pub type ITARGETSR68 = crate::Reg<u32, _ITARGETSR68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR68;
#[doc = "`read()` method returns [itargetsr68::R](itargetsr68::R) reader structure"]
impl crate::Readable for ITARGETSR68 {}
#[doc = "`write(|w| ..)` method takes [itargetsr68::W](itargetsr68::W) writer structure"]
impl crate::Writable for ITARGETSR68 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr68;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr69](itargetsr69) module"]
pub type ITARGETSR69 = crate::Reg<u32, _ITARGETSR69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR69;
#[doc = "`read()` method returns [itargetsr69::R](itargetsr69::R) reader structure"]
impl crate::Readable for ITARGETSR69 {}
#[doc = "`write(|w| ..)` method takes [itargetsr69::W](itargetsr69::W) writer structure"]
impl crate::Writable for ITARGETSR69 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr69;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr70](itargetsr70) module"]
pub type ITARGETSR70 = crate::Reg<u32, _ITARGETSR70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR70;
#[doc = "`read()` method returns [itargetsr70::R](itargetsr70::R) reader structure"]
impl crate::Readable for ITARGETSR70 {}
#[doc = "`write(|w| ..)` method takes [itargetsr70::W](itargetsr70::W) writer structure"]
impl crate::Writable for ITARGETSR70 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr70;
#[doc = "GICD interrupt processor target registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itargetsr71](itargetsr71) module"]
pub type ITARGETSR71 = crate::Reg<u32, _ITARGETSR71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITARGETSR71;
#[doc = "`read()` method returns [itargetsr71::R](itargetsr71::R) reader structure"]
impl crate::Readable for ITARGETSR71 {}
#[doc = "`write(|w| ..)` method takes [itargetsr71::W](itargetsr71::W) writer structure"]
impl crate::Writable for ITARGETSR71 {}
#[doc = "GICD interrupt processor target registers"]
pub mod itargetsr71;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr0](icfgr0) module"]
pub type ICFGR0 = crate::Reg<u32, _ICFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR0;
#[doc = "`read()` method returns [icfgr0::R](icfgr0::R) reader structure"]
impl crate::Readable for ICFGR0 {}
#[doc = "`write(|w| ..)` method takes [icfgr0::W](icfgr0::W) writer structure"]
impl crate::Writable for ICFGR0 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr0;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr1](icfgr1) module"]
pub type ICFGR1 = crate::Reg<u32, _ICFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR1;
#[doc = "`read()` method returns [icfgr1::R](icfgr1::R) reader structure"]
impl crate::Readable for ICFGR1 {}
#[doc = "`write(|w| ..)` method takes [icfgr1::W](icfgr1::W) writer structure"]
impl crate::Writable for ICFGR1 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr1;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr2](icfgr2) module"]
pub type ICFGR2 = crate::Reg<u32, _ICFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR2;
#[doc = "`read()` method returns [icfgr2::R](icfgr2::R) reader structure"]
impl crate::Readable for ICFGR2 {}
#[doc = "`write(|w| ..)` method takes [icfgr2::W](icfgr2::W) writer structure"]
impl crate::Writable for ICFGR2 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr2;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr3](icfgr3) module"]
pub type ICFGR3 = crate::Reg<u32, _ICFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR3;
#[doc = "`read()` method returns [icfgr3::R](icfgr3::R) reader structure"]
impl crate::Readable for ICFGR3 {}
#[doc = "`write(|w| ..)` method takes [icfgr3::W](icfgr3::W) writer structure"]
impl crate::Writable for ICFGR3 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr3;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr4](icfgr4) module"]
pub type ICFGR4 = crate::Reg<u32, _ICFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR4;
#[doc = "`read()` method returns [icfgr4::R](icfgr4::R) reader structure"]
impl crate::Readable for ICFGR4 {}
#[doc = "`write(|w| ..)` method takes [icfgr4::W](icfgr4::W) writer structure"]
impl crate::Writable for ICFGR4 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr4;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr5](icfgr5) module"]
pub type ICFGR5 = crate::Reg<u32, _ICFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR5;
#[doc = "`read()` method returns [icfgr5::R](icfgr5::R) reader structure"]
impl crate::Readable for ICFGR5 {}
#[doc = "`write(|w| ..)` method takes [icfgr5::W](icfgr5::W) writer structure"]
impl crate::Writable for ICFGR5 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr5;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr6](icfgr6) module"]
pub type ICFGR6 = crate::Reg<u32, _ICFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR6;
#[doc = "`read()` method returns [icfgr6::R](icfgr6::R) reader structure"]
impl crate::Readable for ICFGR6 {}
#[doc = "`write(|w| ..)` method takes [icfgr6::W](icfgr6::W) writer structure"]
impl crate::Writable for ICFGR6 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr6;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr7](icfgr7) module"]
pub type ICFGR7 = crate::Reg<u32, _ICFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR7;
#[doc = "`read()` method returns [icfgr7::R](icfgr7::R) reader structure"]
impl crate::Readable for ICFGR7 {}
#[doc = "`write(|w| ..)` method takes [icfgr7::W](icfgr7::W) writer structure"]
impl crate::Writable for ICFGR7 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr7;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr8](icfgr8) module"]
pub type ICFGR8 = crate::Reg<u32, _ICFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR8;
#[doc = "`read()` method returns [icfgr8::R](icfgr8::R) reader structure"]
impl crate::Readable for ICFGR8 {}
#[doc = "`write(|w| ..)` method takes [icfgr8::W](icfgr8::W) writer structure"]
impl crate::Writable for ICFGR8 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr8;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr9](icfgr9) module"]
pub type ICFGR9 = crate::Reg<u32, _ICFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR9;
#[doc = "`read()` method returns [icfgr9::R](icfgr9::R) reader structure"]
impl crate::Readable for ICFGR9 {}
#[doc = "`write(|w| ..)` method takes [icfgr9::W](icfgr9::W) writer structure"]
impl crate::Writable for ICFGR9 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr9;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr10](icfgr10) module"]
pub type ICFGR10 = crate::Reg<u32, _ICFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR10;
#[doc = "`read()` method returns [icfgr10::R](icfgr10::R) reader structure"]
impl crate::Readable for ICFGR10 {}
#[doc = "`write(|w| ..)` method takes [icfgr10::W](icfgr10::W) writer structure"]
impl crate::Writable for ICFGR10 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr10;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr11](icfgr11) module"]
pub type ICFGR11 = crate::Reg<u32, _ICFGR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR11;
#[doc = "`read()` method returns [icfgr11::R](icfgr11::R) reader structure"]
impl crate::Readable for ICFGR11 {}
#[doc = "`write(|w| ..)` method takes [icfgr11::W](icfgr11::W) writer structure"]
impl crate::Writable for ICFGR11 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr11;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr12](icfgr12) module"]
pub type ICFGR12 = crate::Reg<u32, _ICFGR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR12;
#[doc = "`read()` method returns [icfgr12::R](icfgr12::R) reader structure"]
impl crate::Readable for ICFGR12 {}
#[doc = "`write(|w| ..)` method takes [icfgr12::W](icfgr12::W) writer structure"]
impl crate::Writable for ICFGR12 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr12;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr13](icfgr13) module"]
pub type ICFGR13 = crate::Reg<u32, _ICFGR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR13;
#[doc = "`read()` method returns [icfgr13::R](icfgr13::R) reader structure"]
impl crate::Readable for ICFGR13 {}
#[doc = "`write(|w| ..)` method takes [icfgr13::W](icfgr13::W) writer structure"]
impl crate::Writable for ICFGR13 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr13;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr14](icfgr14) module"]
pub type ICFGR14 = crate::Reg<u32, _ICFGR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR14;
#[doc = "`read()` method returns [icfgr14::R](icfgr14::R) reader structure"]
impl crate::Readable for ICFGR14 {}
#[doc = "`write(|w| ..)` method takes [icfgr14::W](icfgr14::W) writer structure"]
impl crate::Writable for ICFGR14 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr14;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr15](icfgr15) module"]
pub type ICFGR15 = crate::Reg<u32, _ICFGR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR15;
#[doc = "`read()` method returns [icfgr15::R](icfgr15::R) reader structure"]
impl crate::Readable for ICFGR15 {}
#[doc = "`write(|w| ..)` method takes [icfgr15::W](icfgr15::W) writer structure"]
impl crate::Writable for ICFGR15 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr15;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr16](icfgr16) module"]
pub type ICFGR16 = crate::Reg<u32, _ICFGR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR16;
#[doc = "`read()` method returns [icfgr16::R](icfgr16::R) reader structure"]
impl crate::Readable for ICFGR16 {}
#[doc = "`write(|w| ..)` method takes [icfgr16::W](icfgr16::W) writer structure"]
impl crate::Writable for ICFGR16 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr16;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfgr17](icfgr17) module"]
pub type ICFGR17 = crate::Reg<u32, _ICFGR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICFGR17;
#[doc = "`read()` method returns [icfgr17::R](icfgr17::R) reader structure"]
impl crate::Readable for ICFGR17 {}
#[doc = "`write(|w| ..)` method takes [icfgr17::W](icfgr17::W) writer structure"]
impl crate::Writable for ICFGR17 {}
#[doc = "GICD interrupt configuration register"]
pub mod icfgr17;
#[doc = "GICD private peripheral interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppisr](ppisr) module"]
pub type PPISR = crate::Reg<u32, _PPISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPISR;
#[doc = "`read()` method returns [ppisr::R](ppisr::R) reader structure"]
impl crate::Readable for PPISR {}
#[doc = "GICD private peripheral interrupt status register"]
pub mod ppisr;
#[doc = "GICD shared peripheral interrupt registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spisr0](spisr0) module"]
pub type SPISR0 = crate::Reg<u32, _SPISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPISR0;
#[doc = "`read()` method returns [spisr0::R](spisr0::R) reader structure"]
impl crate::Readable for SPISR0 {}
#[doc = "GICD shared peripheral interrupt registers"]
pub mod spisr0;
#[doc = "GICD shared peripheral interrupt registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spisr1](spisr1) module"]
pub type SPISR1 = crate::Reg<u32, _SPISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPISR1;
#[doc = "`read()` method returns [spisr1::R](spisr1::R) reader structure"]
impl crate::Readable for SPISR1 {}
#[doc = "GICD shared peripheral interrupt registers"]
pub mod spisr1;
#[doc = "GICD shared peripheral interrupt registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spisr2](spisr2) module"]
pub type SPISR2 = crate::Reg<u32, _SPISR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPISR2;
#[doc = "`read()` method returns [spisr2::R](spisr2::R) reader structure"]
impl crate::Readable for SPISR2 {}
#[doc = "GICD shared peripheral interrupt registers"]
pub mod spisr2;
#[doc = "GICD shared peripheral interrupt registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spisr3](spisr3) module"]
pub type SPISR3 = crate::Reg<u32, _SPISR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPISR3;
#[doc = "`read()` method returns [spisr3::R](spisr3::R) reader structure"]
impl crate::Readable for SPISR3 {}
#[doc = "GICD shared peripheral interrupt registers"]
pub mod spisr3;
#[doc = "GICD shared peripheral interrupt registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spisr4](spisr4) module"]
pub type SPISR4 = crate::Reg<u32, _SPISR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPISR4;
#[doc = "`read()` method returns [spisr4::R](spisr4::R) reader structure"]
impl crate::Readable for SPISR4 {}
#[doc = "GICD shared peripheral interrupt registers"]
pub mod spisr4;
#[doc = "GICD shared peripheral interrupt registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spisr5](spisr5) module"]
pub type SPISR5 = crate::Reg<u32, _SPISR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPISR5;
#[doc = "`read()` method returns [spisr5::R](spisr5::R) reader structure"]
impl crate::Readable for SPISR5 {}
#[doc = "GICD shared peripheral interrupt registers"]
pub mod spisr5;
#[doc = "GICD shared peripheral interrupt registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spisr6](spisr6) module"]
pub type SPISR6 = crate::Reg<u32, _SPISR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPISR6;
#[doc = "`read()` method returns [spisr6::R](spisr6::R) reader structure"]
impl crate::Readable for SPISR6 {}
#[doc = "GICD shared peripheral interrupt registers"]
pub mod spisr6;
#[doc = "GICD shared peripheral interrupt registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spisr7](spisr7) module"]
pub type SPISR7 = crate::Reg<u32, _SPISR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPISR7;
#[doc = "`read()` method returns [spisr7::R](spisr7::R) reader structure"]
impl crate::Readable for SPISR7 {}
#[doc = "GICD shared peripheral interrupt registers"]
pub mod spisr7;
#[doc = "GICD software generated interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sgir](sgir) module"]
pub type SGIR = crate::Reg<u32, _SGIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SGIR;
#[doc = "`write(|w| ..)` method takes [sgir::W](sgir::W) writer structure"]
impl crate::Writable for SGIR {}
#[doc = "GICD software generated interrupt register"]
pub mod sgir;
#[doc = "GICD SGI clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpendsgir0](cpendsgir0) module"]
pub type CPENDSGIR0 = crate::Reg<u32, _CPENDSGIR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPENDSGIR0;
#[doc = "`read()` method returns [cpendsgir0::R](cpendsgir0::R) reader structure"]
impl crate::Readable for CPENDSGIR0 {}
#[doc = "`write(|w| ..)` method takes [cpendsgir0::W](cpendsgir0::W) writer structure"]
impl crate::Writable for CPENDSGIR0 {}
#[doc = "GICD SGI clear-pending registers"]
pub mod cpendsgir0;
#[doc = "GICD SGI clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpendsgir1](cpendsgir1) module"]
pub type CPENDSGIR1 = crate::Reg<u32, _CPENDSGIR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPENDSGIR1;
#[doc = "`read()` method returns [cpendsgir1::R](cpendsgir1::R) reader structure"]
impl crate::Readable for CPENDSGIR1 {}
#[doc = "`write(|w| ..)` method takes [cpendsgir1::W](cpendsgir1::W) writer structure"]
impl crate::Writable for CPENDSGIR1 {}
#[doc = "GICD SGI clear-pending registers"]
pub mod cpendsgir1;
#[doc = "GICD SGI clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpendsgir2](cpendsgir2) module"]
pub type CPENDSGIR2 = crate::Reg<u32, _CPENDSGIR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPENDSGIR2;
#[doc = "`read()` method returns [cpendsgir2::R](cpendsgir2::R) reader structure"]
impl crate::Readable for CPENDSGIR2 {}
#[doc = "`write(|w| ..)` method takes [cpendsgir2::W](cpendsgir2::W) writer structure"]
impl crate::Writable for CPENDSGIR2 {}
#[doc = "GICD SGI clear-pending registers"]
pub mod cpendsgir2;
#[doc = "GICD SGI clear-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpendsgir3](cpendsgir3) module"]
pub type CPENDSGIR3 = crate::Reg<u32, _CPENDSGIR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPENDSGIR3;
#[doc = "`read()` method returns [cpendsgir3::R](cpendsgir3::R) reader structure"]
impl crate::Readable for CPENDSGIR3 {}
#[doc = "`write(|w| ..)` method takes [cpendsgir3::W](cpendsgir3::W) writer structure"]
impl crate::Writable for CPENDSGIR3 {}
#[doc = "GICD SGI clear-pending registers"]
pub mod cpendsgir3;
#[doc = "GICD SGI set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spendsgir0](spendsgir0) module"]
pub type SPENDSGIR0 = crate::Reg<u32, _SPENDSGIR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPENDSGIR0;
#[doc = "`read()` method returns [spendsgir0::R](spendsgir0::R) reader structure"]
impl crate::Readable for SPENDSGIR0 {}
#[doc = "`write(|w| ..)` method takes [spendsgir0::W](spendsgir0::W) writer structure"]
impl crate::Writable for SPENDSGIR0 {}
#[doc = "GICD SGI set-pending registers"]
pub mod spendsgir0;
#[doc = "GICD SGI set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spendsgir1](spendsgir1) module"]
pub type SPENDSGIR1 = crate::Reg<u32, _SPENDSGIR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPENDSGIR1;
#[doc = "`read()` method returns [spendsgir1::R](spendsgir1::R) reader structure"]
impl crate::Readable for SPENDSGIR1 {}
#[doc = "`write(|w| ..)` method takes [spendsgir1::W](spendsgir1::W) writer structure"]
impl crate::Writable for SPENDSGIR1 {}
#[doc = "GICD SGI set-pending registers"]
pub mod spendsgir1;
#[doc = "GICD SGI set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spendsgir2](spendsgir2) module"]
pub type SPENDSGIR2 = crate::Reg<u32, _SPENDSGIR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPENDSGIR2;
#[doc = "`read()` method returns [spendsgir2::R](spendsgir2::R) reader structure"]
impl crate::Readable for SPENDSGIR2 {}
#[doc = "`write(|w| ..)` method takes [spendsgir2::W](spendsgir2::W) writer structure"]
impl crate::Writable for SPENDSGIR2 {}
#[doc = "GICD SGI set-pending registers"]
pub mod spendsgir2;
#[doc = "GICD SGI set-pending registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spendsgir3](spendsgir3) module"]
pub type SPENDSGIR3 = crate::Reg<u32, _SPENDSGIR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPENDSGIR3;
#[doc = "`read()` method returns [spendsgir3::R](spendsgir3::R) reader structure"]
impl crate::Readable for SPENDSGIR3 {}
#[doc = "`write(|w| ..)` method takes [spendsgir3::W](spendsgir3::W) writer structure"]
impl crate::Writable for SPENDSGIR3 {}
#[doc = "GICD SGI set-pending registers"]
pub mod spendsgir3;
#[doc = "GICD peripheral ID4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr4](pidr4) module"]
pub type PIDR4 = crate::Reg<u32, _PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR4;
#[doc = "`read()` method returns [pidr4::R](pidr4::R) reader structure"]
impl crate::Readable for PIDR4 {}
#[doc = "GICD peripheral ID4 register"]
pub mod pidr4;
#[doc = "GICD peripheral ID5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr5](pidr5) module"]
pub type PIDR5 = crate::Reg<u32, _PIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR5;
#[doc = "`read()` method returns [pidr5::R](pidr5::R) reader structure"]
impl crate::Readable for PIDR5 {}
#[doc = "GICD peripheral ID5 register"]
pub mod pidr5;
#[doc = "GICD peripheral ID6 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr6](pidr6) module"]
pub type PIDR6 = crate::Reg<u32, _PIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR6;
#[doc = "`read()` method returns [pidr6::R](pidr6::R) reader structure"]
impl crate::Readable for PIDR6 {}
#[doc = "GICD peripheral ID6 register"]
pub mod pidr6;
#[doc = "GICD peripheral ID7 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr7](pidr7) module"]
pub type PIDR7 = crate::Reg<u32, _PIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR7;
#[doc = "`read()` method returns [pidr7::R](pidr7::R) reader structure"]
impl crate::Readable for PIDR7 {}
#[doc = "GICD peripheral ID7 register"]
pub mod pidr7;
#[doc = "GICD peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr0](pidr0) module"]
pub type PIDR0 = crate::Reg<u32, _PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR0;
#[doc = "`read()` method returns [pidr0::R](pidr0::R) reader structure"]
impl crate::Readable for PIDR0 {}
#[doc = "GICD peripheral ID0 register"]
pub mod pidr0;
#[doc = "GICD peripheral ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr1](pidr1) module"]
pub type PIDR1 = crate::Reg<u32, _PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR1;
#[doc = "`read()` method returns [pidr1::R](pidr1::R) reader structure"]
impl crate::Readable for PIDR1 {}
#[doc = "GICD peripheral ID1 register"]
pub mod pidr1;
#[doc = "GICD peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr2](pidr2) module"]
pub type PIDR2 = crate::Reg<u32, _PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR2;
#[doc = "`read()` method returns [pidr2::R](pidr2::R) reader structure"]
impl crate::Readable for PIDR2 {}
#[doc = "GICD peripheral ID2 register"]
pub mod pidr2;
#[doc = "GICD peripheral ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr3](pidr3) module"]
pub type PIDR3 = crate::Reg<u32, _PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR3;
#[doc = "`read()` method returns [pidr3::R](pidr3::R) reader structure"]
impl crate::Readable for PIDR3 {}
#[doc = "GICD peripheral ID3 register"]
pub mod pidr3;
#[doc = "GICD component ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr0](cidr0) module"]
pub type CIDR0 = crate::Reg<u32, _CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR0;
#[doc = "`read()` method returns [cidr0::R](cidr0::R) reader structure"]
impl crate::Readable for CIDR0 {}
#[doc = "GICD component ID0 register"]
pub mod cidr0;
#[doc = "GICD component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr1](cidr1) module"]
pub type CIDR1 = crate::Reg<u32, _CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR1;
#[doc = "`read()` method returns [cidr1::R](cidr1::R) reader structure"]
impl crate::Readable for CIDR1 {}
#[doc = "GICD component ID1 register"]
pub mod cidr1;
#[doc = "GICD component ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr2](cidr2) module"]
pub type CIDR2 = crate::Reg<u32, _CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR2;
#[doc = "`read()` method returns [cidr2::R](cidr2::R) reader structure"]
impl crate::Readable for CIDR2 {}
#[doc = "GICD component ID2 register"]
pub mod cidr2;
#[doc = "GICD component ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr3](cidr3) module"]
pub type CIDR3 = crate::Reg<u32, _CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR3;
#[doc = "`read()` method returns [cidr3::R](cidr3::R) reader structure"]
impl crate::Readable for CIDR3 {}
#[doc = "GICD component ID3 register"]
pub mod cidr3;
