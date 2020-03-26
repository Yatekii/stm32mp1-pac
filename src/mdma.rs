#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDMA Global Interrupt/Status Register"]
    pub mdma_gisr0: MDMA_GISR0,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - MDMA secure global interrupt/status register"]
    pub mdma_sgisr0: MDMA_SGISR0,
    _reserved2: [u8; 52usize],
    #[doc = "0x40 - MDMA channel 0 interrupt/status register"]
    pub mdma_c0isr: MDMA_C0ISR,
    #[doc = "0x44 - MDMA channel 0 interrupt flag clear register"]
    pub mdma_c0ifcr: MDMA_C0IFCR,
    #[doc = "0x48 - MDMA Channel 0 error status register"]
    pub mdma_c0esr: MDMA_C0ESR,
    #[doc = "0x4c - This register is used to control the concerned channel."]
    pub mdma_c0cr: MDMA_C0CR,
    #[doc = "0x50 - This register is used to configure the concerned channel."]
    pub mdma_c0tcr: MDMA_C0TCR,
    #[doc = "0x54 - MDMA Channel 0 block number of data register"]
    pub mdma_c0bndtr: MDMA_C0BNDTR,
    #[doc = "0x58 - MDMA channel 0 source address register"]
    pub mdma_c0sar: MDMA_C0SAR,
    #[doc = "0x5c - MDMA channel 0 destination address register"]
    pub mdma_c0dar: MDMA_C0DAR,
    #[doc = "0x60 - MDMA channel 0 Block Repeat address Update register"]
    pub mdma_c0brur: MDMA_C0BRUR,
    #[doc = "0x64 - MDMA channel 0 Link Address register"]
    pub mdma_c0lar: MDMA_C0LAR,
    #[doc = "0x68 - MDMA channel 0 Trigger and Bus selection Register"]
    pub mdma_c0tbr: MDMA_C0TBR,
    _reserved13: [u8; 4usize],
    #[doc = "0x70 - MDMA channel 0 Mask address register"]
    pub mdma_c0mar: MDMA_C0MAR,
    #[doc = "0x74 - MDMA channel 0 Mask Data register"]
    pub mdma_c0mdr: MDMA_C0MDR,
    _reserved15: [u8; 8usize],
    #[doc = "0x80 - MDMA channel 1 interrupt/status register"]
    pub mdma_c1isr: MDMA_C1ISR,
    #[doc = "0x84 - MDMA channel 1 interrupt flag clear register"]
    pub mdma_c1ifcr: MDMA_C1IFCR,
    #[doc = "0x88 - MDMA Channel 1 error status register"]
    pub mdma_c1esr: MDMA_C1ESR,
    #[doc = "0x8c - This register is used to control the concerned channel."]
    pub mdma_c1cr: MDMA_C1CR,
    #[doc = "0x90 - This register is used to configure the concerned channel."]
    pub mdma_c1tcr: MDMA_C1TCR,
    #[doc = "0x94 - MDMA Channel 1 block number of data register"]
    pub mdma_c1bndtr: MDMA_C1BNDTR,
    #[doc = "0x98 - MDMA channel 1 source address register"]
    pub mdma_c1sar: MDMA_C1SAR,
    #[doc = "0x9c - MDMA channel 1 destination address register"]
    pub mdma_c1dar: MDMA_C1DAR,
    #[doc = "0xa0 - MDMA channel 1 Block Repeat address Update register"]
    pub mdma_c1brur: MDMA_C1BRUR,
    #[doc = "0xa4 - MDMA channel 1 Link Address register"]
    pub mdma_c1lar: MDMA_C1LAR,
    #[doc = "0xa8 - MDMA channel 1 Trigger and Bus selection Register"]
    pub mdma_c1tbr: MDMA_C1TBR,
    _reserved26: [u8; 4usize],
    #[doc = "0xb0 - MDMA channel 1 Mask address register"]
    pub mdma_c1mar: MDMA_C1MAR,
    #[doc = "0xb4 - MDMA channel 1 Mask Data register"]
    pub mdma_c1mdr: MDMA_C1MDR,
    _reserved28: [u8; 8usize],
    #[doc = "0xc0 - MDMA channel 2 interrupt/status register"]
    pub mdma_c2isr: MDMA_C2ISR,
    #[doc = "0xc4 - MDMA channel 2 interrupt flag clear register"]
    pub mdma_c2ifcr: MDMA_C2IFCR,
    #[doc = "0xc8 - MDMA Channel 2 error status register"]
    pub mdma_c2esr: MDMA_C2ESR,
    #[doc = "0xcc - This register is used to control the concerned channel."]
    pub mdma_c2cr: MDMA_C2CR,
    #[doc = "0xd0 - This register is used to configure the concerned channel."]
    pub mdma_c2tcr: MDMA_C2TCR,
    #[doc = "0xd4 - MDMA Channel 2 block number of data register"]
    pub mdma_c2bndtr: MDMA_C2BNDTR,
    #[doc = "0xd8 - MDMA channel 2 source address register"]
    pub mdma_c2sar: MDMA_C2SAR,
    #[doc = "0xdc - MDMA channel 2 destination address register"]
    pub mdma_c2dar: MDMA_C2DAR,
    #[doc = "0xe0 - MDMA channel 2 Block Repeat address Update register"]
    pub mdma_c2brur: MDMA_C2BRUR,
    #[doc = "0xe4 - MDMA channel 2 Link Address register"]
    pub mdma_c2lar: MDMA_C2LAR,
    #[doc = "0xe8 - MDMA channel 2 Trigger and Bus selection Register"]
    pub mdma_c2tbr: MDMA_C2TBR,
    _reserved39: [u8; 4usize],
    #[doc = "0xf0 - MDMA channel 2 Mask address register"]
    pub mdma_c2mar: MDMA_C2MAR,
    #[doc = "0xf4 - MDMA channel 2 Mask Data register"]
    pub mdma_c2mdr: MDMA_C2MDR,
    _reserved41: [u8; 8usize],
    #[doc = "0x100 - MDMA channel 3 interrupt/status register"]
    pub mdma_c3isr: MDMA_C3ISR,
    #[doc = "0x104 - MDMA channel 3 interrupt flag clear register"]
    pub mdma_c3ifcr: MDMA_C3IFCR,
    #[doc = "0x108 - MDMA Channel 3 error status register"]
    pub mdma_c3esr: MDMA_C3ESR,
    #[doc = "0x10c - This register is used to control the concerned channel."]
    pub mdma_c3cr: MDMA_C3CR,
    #[doc = "0x110 - This register is used to configure the concerned channel."]
    pub mdma_c3tcr: MDMA_C3TCR,
    #[doc = "0x114 - MDMA Channel 3 block number of data register"]
    pub mdma_c3bndtr: MDMA_C3BNDTR,
    #[doc = "0x118 - MDMA channel 3 source address register"]
    pub mdma_c3sar: MDMA_C3SAR,
    #[doc = "0x11c - MDMA channel 3 destination address register"]
    pub mdma_c3dar: MDMA_C3DAR,
    #[doc = "0x120 - MDMA channel 3 Block Repeat address Update register"]
    pub mdma_c3brur: MDMA_C3BRUR,
    #[doc = "0x124 - MDMA channel 3 Link Address register"]
    pub mdma_c3lar: MDMA_C3LAR,
    #[doc = "0x128 - MDMA channel 3 Trigger and Bus selection Register"]
    pub mdma_c3tbr: MDMA_C3TBR,
    _reserved52: [u8; 4usize],
    #[doc = "0x130 - MDMA channel 3 Mask address register"]
    pub mdma_c3mar: MDMA_C3MAR,
    #[doc = "0x134 - MDMA channel 3 Mask Data register"]
    pub mdma_c3mdr: MDMA_C3MDR,
    _reserved54: [u8; 8usize],
    #[doc = "0x140 - MDMA channel 4 interrupt/status register"]
    pub mdma_c4isr: MDMA_C4ISR,
    #[doc = "0x144 - MDMA channel 4 interrupt flag clear register"]
    pub mdma_c4ifcr: MDMA_C4IFCR,
    #[doc = "0x148 - MDMA Channel 4 error status register"]
    pub mdma_c4esr: MDMA_C4ESR,
    #[doc = "0x14c - This register is used to control the concerned channel."]
    pub mdma_c4cr: MDMA_C4CR,
    #[doc = "0x150 - This register is used to configure the concerned channel."]
    pub mdma_c4tcr: MDMA_C4TCR,
    #[doc = "0x154 - MDMA Channel 4 block number of data register"]
    pub mdma_c4bndtr: MDMA_C4BNDTR,
    #[doc = "0x158 - MDMA channel 4 source address register"]
    pub mdma_c4sar: MDMA_C4SAR,
    #[doc = "0x15c - MDMA channel 4 destination address register"]
    pub mdma_c4dar: MDMA_C4DAR,
    #[doc = "0x160 - MDMA channel 4 Block Repeat address Update register"]
    pub mdma_c4brur: MDMA_C4BRUR,
    #[doc = "0x164 - MDMA channel 4 Link Address register"]
    pub mdma_c4lar: MDMA_C4LAR,
    #[doc = "0x168 - MDMA channel 4 Trigger and Bus selection Register"]
    pub mdma_c4tbr: MDMA_C4TBR,
    _reserved65: [u8; 4usize],
    #[doc = "0x170 - MDMA channel 4 Mask address register"]
    pub mdma_c4mar: MDMA_C4MAR,
    #[doc = "0x174 - MDMA channel 4 Mask Data register"]
    pub mdma_c4mdr: MDMA_C4MDR,
    _reserved67: [u8; 8usize],
    #[doc = "0x180 - MDMA channel 5 interrupt/status register"]
    pub mdma_c5isr: MDMA_C5ISR,
    #[doc = "0x184 - MDMA channel 5 interrupt flag clear register"]
    pub mdma_c5ifcr: MDMA_C5IFCR,
    #[doc = "0x188 - MDMA Channel 5 error status register"]
    pub mdma_c5esr: MDMA_C5ESR,
    #[doc = "0x18c - This register is used to control the concerned channel."]
    pub mdma_c5cr: MDMA_C5CR,
    #[doc = "0x190 - This register is used to configure the concerned channel."]
    pub mdma_c5tcr: MDMA_C5TCR,
    #[doc = "0x194 - MDMA Channel 5 block number of data register"]
    pub mdma_c5bndtr: MDMA_C5BNDTR,
    #[doc = "0x198 - MDMA channel 5 source address register"]
    pub mdma_c5sar: MDMA_C5SAR,
    #[doc = "0x19c - MDMA channel 5 destination address register"]
    pub mdma_c5dar: MDMA_C5DAR,
    #[doc = "0x1a0 - MDMA channel 5 Block Repeat address Update register"]
    pub mdma_c5brur: MDMA_C5BRUR,
    #[doc = "0x1a4 - MDMA channel 5 Link Address register"]
    pub mdma_c5lar: MDMA_C5LAR,
    #[doc = "0x1a8 - MDMA channel 5 Trigger and Bus selection Register"]
    pub mdma_c5tbr: MDMA_C5TBR,
    _reserved78: [u8; 4usize],
    #[doc = "0x1b0 - MDMA channel 5 Mask address register"]
    pub mdma_c5mar: MDMA_C5MAR,
    #[doc = "0x1b4 - MDMA channel 5 Mask Data register"]
    pub mdma_c5mdr: MDMA_C5MDR,
    _reserved80: [u8; 8usize],
    #[doc = "0x1c0 - MDMA channel 6 interrupt/status register"]
    pub mdma_c6isr: MDMA_C6ISR,
    #[doc = "0x1c4 - MDMA channel 6 interrupt flag clear register"]
    pub mdma_c6ifcr: MDMA_C6IFCR,
    #[doc = "0x1c8 - MDMA Channel 6 error status register"]
    pub mdma_c6esr: MDMA_C6ESR,
    #[doc = "0x1cc - This register is used to control the concerned channel."]
    pub mdma_c6cr: MDMA_C6CR,
    #[doc = "0x1d0 - This register is used to configure the concerned channel."]
    pub mdma_c6tcr: MDMA_C6TCR,
    #[doc = "0x1d4 - MDMA Channel 6 block number of data register"]
    pub mdma_c6bndtr: MDMA_C6BNDTR,
    #[doc = "0x1d8 - MDMA channel 6 source address register"]
    pub mdma_c6sar: MDMA_C6SAR,
    #[doc = "0x1dc - MDMA channel 6 destination address register"]
    pub mdma_c6dar: MDMA_C6DAR,
    #[doc = "0x1e0 - MDMA channel 6 Block Repeat address Update register"]
    pub mdma_c6brur: MDMA_C6BRUR,
    #[doc = "0x1e4 - MDMA channel 6 Link Address register"]
    pub mdma_c6lar: MDMA_C6LAR,
    #[doc = "0x1e8 - MDMA channel 6 Trigger and Bus selection Register"]
    pub mdma_c6tbr: MDMA_C6TBR,
    _reserved91: [u8; 4usize],
    #[doc = "0x1f0 - MDMA channel 6 Mask address register"]
    pub mdma_c6mar: MDMA_C6MAR,
    #[doc = "0x1f4 - MDMA channel 6 Mask Data register"]
    pub mdma_c6mdr: MDMA_C6MDR,
    _reserved93: [u8; 8usize],
    #[doc = "0x200 - MDMA channel 7 interrupt/status register"]
    pub mdma_c7isr: MDMA_C7ISR,
    #[doc = "0x204 - MDMA channel 7 interrupt flag clear register"]
    pub mdma_c7ifcr: MDMA_C7IFCR,
    #[doc = "0x208 - MDMA Channel 7 error status register"]
    pub mdma_c7esr: MDMA_C7ESR,
    #[doc = "0x20c - This register is used to control the concerned channel."]
    pub mdma_c7cr: MDMA_C7CR,
    #[doc = "0x210 - This register is used to configure the concerned channel."]
    pub mdma_c7tcr: MDMA_C7TCR,
    #[doc = "0x214 - MDMA Channel 7 block number of data register"]
    pub mdma_c7bndtr: MDMA_C7BNDTR,
    #[doc = "0x218 - MDMA channel 7 source address register"]
    pub mdma_c7sar: MDMA_C7SAR,
    #[doc = "0x21c - MDMA channel 7 destination address register"]
    pub mdma_c7dar: MDMA_C7DAR,
    #[doc = "0x220 - MDMA channel 7 Block Repeat address Update register"]
    pub mdma_c7brur: MDMA_C7BRUR,
    #[doc = "0x224 - MDMA channel 7 Link Address register"]
    pub mdma_c7lar: MDMA_C7LAR,
    #[doc = "0x228 - MDMA channel 7 Trigger and Bus selection Register"]
    pub mdma_c7tbr: MDMA_C7TBR,
    _reserved104: [u8; 4usize],
    #[doc = "0x230 - MDMA channel 7 Mask address register"]
    pub mdma_c7mar: MDMA_C7MAR,
    #[doc = "0x234 - MDMA channel 7 Mask Data register"]
    pub mdma_c7mdr: MDMA_C7MDR,
    _reserved106: [u8; 8usize],
    #[doc = "0x240 - MDMA channel 8 interrupt/status register"]
    pub mdma_c8isr: MDMA_C8ISR,
    #[doc = "0x244 - MDMA channel 8 interrupt flag clear register"]
    pub mdma_c8ifcr: MDMA_C8IFCR,
    #[doc = "0x248 - MDMA Channel 8 error status register"]
    pub mdma_c8esr: MDMA_C8ESR,
    #[doc = "0x24c - This register is used to control the concerned channel."]
    pub mdma_c8cr: MDMA_C8CR,
    #[doc = "0x250 - This register is used to configure the concerned channel."]
    pub mdma_c8tcr: MDMA_C8TCR,
    #[doc = "0x254 - MDMA Channel 8 block number of data register"]
    pub mdma_c8bndtr: MDMA_C8BNDTR,
    #[doc = "0x258 - MDMA channel 8 source address register"]
    pub mdma_c8sar: MDMA_C8SAR,
    #[doc = "0x25c - MDMA channel 8 destination address register"]
    pub mdma_c8dar: MDMA_C8DAR,
    #[doc = "0x260 - MDMA channel 8 Block Repeat address Update register"]
    pub mdma_c8brur: MDMA_C8BRUR,
    #[doc = "0x264 - MDMA channel 8 Link Address register"]
    pub mdma_c8lar: MDMA_C8LAR,
    #[doc = "0x268 - MDMA channel 8 Trigger and Bus selection Register"]
    pub mdma_c8tbr: MDMA_C8TBR,
    _reserved117: [u8; 4usize],
    #[doc = "0x270 - MDMA channel 8 Mask address register"]
    pub mdma_c8mar: MDMA_C8MAR,
    #[doc = "0x274 - MDMA channel 8 Mask Data register"]
    pub mdma_c8mdr: MDMA_C8MDR,
    _reserved119: [u8; 8usize],
    #[doc = "0x280 - MDMA channel 9 interrupt/status register"]
    pub mdma_c9isr: MDMA_C9ISR,
    #[doc = "0x284 - MDMA channel 9 interrupt flag clear register"]
    pub mdma_c9ifcr: MDMA_C9IFCR,
    #[doc = "0x288 - MDMA Channel 9 error status register"]
    pub mdma_c9esr: MDMA_C9ESR,
    #[doc = "0x28c - This register is used to control the concerned channel."]
    pub mdma_c9cr: MDMA_C9CR,
    #[doc = "0x290 - This register is used to configure the concerned channel."]
    pub mdma_c9tcr: MDMA_C9TCR,
    #[doc = "0x294 - MDMA Channel 9 block number of data register"]
    pub mdma_c9bndtr: MDMA_C9BNDTR,
    #[doc = "0x298 - MDMA channel 9 source address register"]
    pub mdma_c9sar: MDMA_C9SAR,
    #[doc = "0x29c - MDMA channel 9 destination address register"]
    pub mdma_c9dar: MDMA_C9DAR,
    #[doc = "0x2a0 - MDMA channel 9 Block Repeat address Update register"]
    pub mdma_c9brur: MDMA_C9BRUR,
    #[doc = "0x2a4 - MDMA channel 9 Link Address register"]
    pub mdma_c9lar: MDMA_C9LAR,
    #[doc = "0x2a8 - MDMA channel 9 Trigger and Bus selection Register"]
    pub mdma_c9tbr: MDMA_C9TBR,
    _reserved130: [u8; 4usize],
    #[doc = "0x2b0 - MDMA channel 9 Mask address register"]
    pub mdma_c9mar: MDMA_C9MAR,
    #[doc = "0x2b4 - MDMA channel 9 Mask Data register"]
    pub mdma_c9mdr: MDMA_C9MDR,
    _reserved132: [u8; 8usize],
    #[doc = "0x2c0 - MDMA channel 10 interrupt/status register"]
    pub mdma_c10isr: MDMA_C10ISR,
    #[doc = "0x2c4 - MDMA channel 10 interrupt flag clear register"]
    pub mdma_c10ifcr: MDMA_C10IFCR,
    #[doc = "0x2c8 - MDMA Channel 10 error status register"]
    pub mdma_c10esr: MDMA_C10ESR,
    #[doc = "0x2cc - This register is used to control the concerned channel."]
    pub mdma_c10cr: MDMA_C10CR,
    #[doc = "0x2d0 - This register is used to configure the concerned channel."]
    pub mdma_c10tcr: MDMA_C10TCR,
    #[doc = "0x2d4 - MDMA Channel 10 block number of data register"]
    pub mdma_c10bndtr: MDMA_C10BNDTR,
    #[doc = "0x2d8 - MDMA channel 10 source address register"]
    pub mdma_c10sar: MDMA_C10SAR,
    #[doc = "0x2dc - MDMA channel 10 destination address register"]
    pub mdma_c10dar: MDMA_C10DAR,
    #[doc = "0x2e0 - MDMA channel 10 Block Repeat address Update register"]
    pub mdma_c10brur: MDMA_C10BRUR,
    #[doc = "0x2e4 - MDMA channel 10 Link Address register"]
    pub mdma_c10lar: MDMA_C10LAR,
    #[doc = "0x2e8 - MDMA channel 10 Trigger and Bus selection Register"]
    pub mdma_c10tbr: MDMA_C10TBR,
    _reserved143: [u8; 4usize],
    #[doc = "0x2f0 - MDMA channel 10 Mask address register"]
    pub mdma_c10mar: MDMA_C10MAR,
    #[doc = "0x2f4 - MDMA channel 10 Mask Data register"]
    pub mdma_c10mdr: MDMA_C10MDR,
    _reserved145: [u8; 8usize],
    #[doc = "0x300 - MDMA channel 11 interrupt/status register"]
    pub mdma_c11isr: MDMA_C11ISR,
    #[doc = "0x304 - MDMA channel 11 interrupt flag clear register"]
    pub mdma_c11ifcr: MDMA_C11IFCR,
    #[doc = "0x308 - MDMA Channel 11 error status register"]
    pub mdma_c11esr: MDMA_C11ESR,
    #[doc = "0x30c - This register is used to control the concerned channel."]
    pub mdma_c11cr: MDMA_C11CR,
    #[doc = "0x310 - This register is used to configure the concerned channel."]
    pub mdma_c11tcr: MDMA_C11TCR,
    #[doc = "0x314 - MDMA Channel 11 block number of data register"]
    pub mdma_c11bndtr: MDMA_C11BNDTR,
    #[doc = "0x318 - MDMA channel 11 source address register"]
    pub mdma_c11sar: MDMA_C11SAR,
    #[doc = "0x31c - MDMA channel 11 destination address register"]
    pub mdma_c11dar: MDMA_C11DAR,
    #[doc = "0x320 - MDMA channel 11 Block Repeat address Update register"]
    pub mdma_c11brur: MDMA_C11BRUR,
    #[doc = "0x324 - MDMA channel 11 Link Address register"]
    pub mdma_c11lar: MDMA_C11LAR,
    #[doc = "0x328 - MDMA channel 11 Trigger and Bus selection Register"]
    pub mdma_c11tbr: MDMA_C11TBR,
    _reserved156: [u8; 4usize],
    #[doc = "0x330 - MDMA channel 11 Mask address register"]
    pub mdma_c11mar: MDMA_C11MAR,
    #[doc = "0x334 - MDMA channel 11 Mask Data register"]
    pub mdma_c11mdr: MDMA_C11MDR,
    _reserved158: [u8; 8usize],
    #[doc = "0x340 - MDMA channel 12 interrupt/status register"]
    pub mdma_c12isr: MDMA_C12ISR,
    #[doc = "0x344 - MDMA channel 12 interrupt flag clear register"]
    pub mdma_c12ifcr: MDMA_C12IFCR,
    #[doc = "0x348 - MDMA Channel 12 error status register"]
    pub mdma_c12esr: MDMA_C12ESR,
    #[doc = "0x34c - This register is used to control the concerned channel."]
    pub mdma_c12cr: MDMA_C12CR,
    #[doc = "0x350 - This register is used to configure the concerned channel."]
    pub mdma_c12tcr: MDMA_C12TCR,
    #[doc = "0x354 - MDMA Channel 12 block number of data register"]
    pub mdma_c12bndtr: MDMA_C12BNDTR,
    #[doc = "0x358 - MDMA channel 12 source address register"]
    pub mdma_c12sar: MDMA_C12SAR,
    #[doc = "0x35c - MDMA channel 12 destination address register"]
    pub mdma_c12dar: MDMA_C12DAR,
    #[doc = "0x360 - MDMA channel 12 Block Repeat address Update register"]
    pub mdma_c12brur: MDMA_C12BRUR,
    #[doc = "0x364 - MDMA channel 12 Link Address register"]
    pub mdma_c12lar: MDMA_C12LAR,
    #[doc = "0x368 - MDMA channel 12 Trigger and Bus selection Register"]
    pub mdma_c12tbr: MDMA_C12TBR,
    _reserved169: [u8; 4usize],
    #[doc = "0x370 - MDMA channel 12 Mask address register"]
    pub mdma_c12mar: MDMA_C12MAR,
    #[doc = "0x374 - MDMA channel 12 Mask Data register"]
    pub mdma_c12mdr: MDMA_C12MDR,
    _reserved171: [u8; 8usize],
    #[doc = "0x380 - MDMA channel 13 interrupt/status register"]
    pub mdma_c13isr: MDMA_C13ISR,
    #[doc = "0x384 - MDMA channel 13 interrupt flag clear register"]
    pub mdma_c13ifcr: MDMA_C13IFCR,
    #[doc = "0x388 - MDMA Channel 13 error status register"]
    pub mdma_c13esr: MDMA_C13ESR,
    #[doc = "0x38c - This register is used to control the concerned channel."]
    pub mdma_c13cr: MDMA_C13CR,
    #[doc = "0x390 - This register is used to configure the concerned channel."]
    pub mdma_c13tcr: MDMA_C13TCR,
    #[doc = "0x394 - MDMA Channel 13 block number of data register"]
    pub mdma_c13bndtr: MDMA_C13BNDTR,
    #[doc = "0x398 - MDMA channel 13 source address register"]
    pub mdma_c13sar: MDMA_C13SAR,
    #[doc = "0x39c - MDMA channel 13 destination address register"]
    pub mdma_c13dar: MDMA_C13DAR,
    #[doc = "0x3a0 - MDMA channel 13 Block Repeat address Update register"]
    pub mdma_c13brur: MDMA_C13BRUR,
    #[doc = "0x3a4 - MDMA channel 13 Link Address register"]
    pub mdma_c13lar: MDMA_C13LAR,
    #[doc = "0x3a8 - MDMA channel 13 Trigger and Bus selection Register"]
    pub mdma_c13tbr: MDMA_C13TBR,
    _reserved182: [u8; 4usize],
    #[doc = "0x3b0 - MDMA channel 13 Mask address register"]
    pub mdma_c13mar: MDMA_C13MAR,
    #[doc = "0x3b4 - MDMA channel 13 Mask Data register"]
    pub mdma_c13mdr: MDMA_C13MDR,
    _reserved184: [u8; 8usize],
    #[doc = "0x3c0 - MDMA channel 14 interrupt/status register"]
    pub mdma_c14isr: MDMA_C14ISR,
    #[doc = "0x3c4 - MDMA channel 14 interrupt flag clear register"]
    pub mdma_c14ifcr: MDMA_C14IFCR,
    #[doc = "0x3c8 - MDMA Channel 14 error status register"]
    pub mdma_c14esr: MDMA_C14ESR,
    #[doc = "0x3cc - This register is used to control the concerned channel."]
    pub mdma_c14cr: MDMA_C14CR,
    #[doc = "0x3d0 - This register is used to configure the concerned channel."]
    pub mdma_c14tcr: MDMA_C14TCR,
    #[doc = "0x3d4 - MDMA Channel 14 block number of data register"]
    pub mdma_c14bndtr: MDMA_C14BNDTR,
    #[doc = "0x3d8 - MDMA channel 14 source address register"]
    pub mdma_c14sar: MDMA_C14SAR,
    #[doc = "0x3dc - MDMA channel 14 destination address register"]
    pub mdma_c14dar: MDMA_C14DAR,
    #[doc = "0x3e0 - MDMA channel 14 Block Repeat address Update register"]
    pub mdma_c14brur: MDMA_C14BRUR,
    #[doc = "0x3e4 - MDMA channel 14 Link Address register"]
    pub mdma_c14lar: MDMA_C14LAR,
    #[doc = "0x3e8 - MDMA channel 14 Trigger and Bus selection Register"]
    pub mdma_c14tbr: MDMA_C14TBR,
    _reserved195: [u8; 4usize],
    #[doc = "0x3f0 - MDMA channel 14 Mask address register"]
    pub mdma_c14mar: MDMA_C14MAR,
    #[doc = "0x3f4 - MDMA channel 14 Mask Data register"]
    pub mdma_c14mdr: MDMA_C14MDR,
    _reserved197: [u8; 8usize],
    #[doc = "0x400 - MDMA channel 15 interrupt/status register"]
    pub mdma_c15isr: MDMA_C15ISR,
    #[doc = "0x404 - MDMA channel 15 interrupt flag clear register"]
    pub mdma_c15ifcr: MDMA_C15IFCR,
    #[doc = "0x408 - MDMA Channel 15 error status register"]
    pub mdma_c15esr: MDMA_C15ESR,
    #[doc = "0x40c - This register is used to control the concerned channel."]
    pub mdma_c15cr: MDMA_C15CR,
    #[doc = "0x410 - This register is used to configure the concerned channel."]
    pub mdma_c15tcr: MDMA_C15TCR,
    #[doc = "0x414 - MDMA Channel 15 block number of data register"]
    pub mdma_c15bndtr: MDMA_C15BNDTR,
    #[doc = "0x418 - MDMA channel 15 source address register"]
    pub mdma_c15sar: MDMA_C15SAR,
    #[doc = "0x41c - MDMA channel 15 destination address register"]
    pub mdma_c15dar: MDMA_C15DAR,
    #[doc = "0x420 - MDMA channel 15 Block Repeat address Update register"]
    pub mdma_c15brur: MDMA_C15BRUR,
    #[doc = "0x424 - MDMA channel 15 Link Address register"]
    pub mdma_c15lar: MDMA_C15LAR,
    #[doc = "0x428 - MDMA channel 15 Trigger and Bus selection Register"]
    pub mdma_c15tbr: MDMA_C15TBR,
    _reserved208: [u8; 4usize],
    #[doc = "0x430 - MDMA channel 15 Mask address register"]
    pub mdma_c15mar: MDMA_C15MAR,
    #[doc = "0x434 - MDMA channel 15 Mask Data register"]
    pub mdma_c15mdr: MDMA_C15MDR,
    _reserved210: [u8; 8usize],
    #[doc = "0x440 - MDMA channel 16 interrupt/status register"]
    pub mdma_c16isr: MDMA_C16ISR,
    #[doc = "0x444 - MDMA channel 16 interrupt flag clear register"]
    pub mdma_c16ifcr: MDMA_C16IFCR,
    #[doc = "0x448 - MDMA Channel 16 error status register"]
    pub mdma_c16esr: MDMA_C16ESR,
    #[doc = "0x44c - This register is used to control the concerned channel."]
    pub mdma_c16cr: MDMA_C16CR,
    #[doc = "0x450 - This register is used to configure the concerned channel."]
    pub mdma_c16tcr: MDMA_C16TCR,
    #[doc = "0x454 - MDMA Channel block number of data register"]
    pub mdma_c16bndtr: MDMA_C16BNDTR,
    #[doc = "0x458 - MDMA channel source address register"]
    pub mdma_c16sar: MDMA_C16SAR,
    #[doc = "0x45c - MDMA channel destination address register"]
    pub mdma_c16dar: MDMA_C16DAR,
    #[doc = "0x460 - MDMA channel Block Repeat address Update register"]
    pub mdma_c16brur: MDMA_C16BRUR,
    #[doc = "0x464 - MDMA channel Link Address register"]
    pub mdma_c16lar: MDMA_C16LAR,
    #[doc = "0x468 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c16tbr: MDMA_C16TBR,
    _reserved221: [u8; 4usize],
    #[doc = "0x470 - MDMA channel Mask address register"]
    pub mdma_c16mar: MDMA_C16MAR,
    #[doc = "0x474 - MDMA channel Mask Data register"]
    pub mdma_c16mdr: MDMA_C16MDR,
    _reserved223: [u8; 8usize],
    #[doc = "0x480 - MDMA channel 17 interrupt/status register"]
    pub mdma_c17isr: MDMA_C17ISR,
    #[doc = "0x484 - MDMA channel 17 interrupt flag clear register"]
    pub mdma_c17ifcr: MDMA_C17IFCR,
    #[doc = "0x488 - MDMA Channel 17 error status register"]
    pub mdma_c17esr: MDMA_C17ESR,
    #[doc = "0x48c - This register is used to control the concerned channel."]
    pub mdma_c17cr: MDMA_C17CR,
    #[doc = "0x490 - This register is used to configure the concerned channel."]
    pub mdma_c17tcr: MDMA_C17TCR,
    #[doc = "0x494 - MDMA Channel block number of data register"]
    pub mdma_c17bndtr: MDMA_C17BNDTR,
    #[doc = "0x498 - MDMA channel source address register"]
    pub mdma_c17sar: MDMA_C17SAR,
    #[doc = "0x49c - MDMA channel destination address register"]
    pub mdma_c17dar: MDMA_C17DAR,
    #[doc = "0x4a0 - MDMA channel Block Repeat address Update register"]
    pub mdma_c17brur: MDMA_C17BRUR,
    #[doc = "0x4a4 - MDMA channel Link Address register"]
    pub mdma_c17lar: MDMA_C17LAR,
    #[doc = "0x4a8 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c17tbr: MDMA_C17TBR,
    _reserved234: [u8; 4usize],
    #[doc = "0x4b0 - MDMA channel Mask address register"]
    pub mdma_c17mar: MDMA_C17MAR,
    #[doc = "0x4b4 - MDMA channel Mask Data register"]
    pub mdma_c17mdr: MDMA_C17MDR,
    _reserved236: [u8; 8usize],
    #[doc = "0x4c0 - MDMA channel 18 interrupt/status register"]
    pub mdma_c18isr: MDMA_C18ISR,
    #[doc = "0x4c4 - MDMA channel 18 interrupt flag clear register"]
    pub mdma_c18ifcr: MDMA_C18IFCR,
    #[doc = "0x4c8 - MDMA Channel 18 error status register"]
    pub mdma_c18esr: MDMA_C18ESR,
    #[doc = "0x4cc - This register is used to control the concerned channel."]
    pub mdma_c18cr: MDMA_C18CR,
    #[doc = "0x4d0 - This register is used to configure the concerned channel."]
    pub mdma_c18tcr: MDMA_C18TCR,
    #[doc = "0x4d4 - MDMA Channel block number of data register"]
    pub mdma_c18bndtr: MDMA_C18BNDTR,
    #[doc = "0x4d8 - MDMA channel source address register"]
    pub mdma_c18sar: MDMA_C18SAR,
    #[doc = "0x4dc - MDMA channel destination address register"]
    pub mdma_c18dar: MDMA_C18DAR,
    #[doc = "0x4e0 - MDMA channel Block Repeat address Update register"]
    pub mdma_c18brur: MDMA_C18BRUR,
    #[doc = "0x4e4 - MDMA channel Link Address register"]
    pub mdma_c18lar: MDMA_C18LAR,
    #[doc = "0x4e8 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c18tbr: MDMA_C18TBR,
    _reserved247: [u8; 4usize],
    #[doc = "0x4f0 - MDMA channel Mask address register"]
    pub mdma_c18mar: MDMA_C18MAR,
    #[doc = "0x4f4 - MDMA channel Mask Data register"]
    pub mdma_c18mdr: MDMA_C18MDR,
    _reserved249: [u8; 8usize],
    #[doc = "0x500 - MDMA channel 19 interrupt/status register"]
    pub mdma_c19isr: MDMA_C19ISR,
    #[doc = "0x504 - MDMA channel 19 interrupt flag clear register"]
    pub mdma_c19ifcr: MDMA_C19IFCR,
    #[doc = "0x508 - MDMA Channel 19 error status register"]
    pub mdma_c19esr: MDMA_C19ESR,
    #[doc = "0x50c - This register is used to control the concerned channel."]
    pub mdma_c19cr: MDMA_C19CR,
    #[doc = "0x510 - This register is used to configure the concerned channel."]
    pub mdma_c19tcr: MDMA_C19TCR,
    #[doc = "0x514 - MDMA Channel block number of data register"]
    pub mdma_c19bndtr: MDMA_C19BNDTR,
    #[doc = "0x518 - MDMA channel source address register"]
    pub mdma_c19sar: MDMA_C19SAR,
    #[doc = "0x51c - MDMA channel destination address register"]
    pub mdma_c19dar: MDMA_C19DAR,
    #[doc = "0x520 - MDMA channel Block Repeat address Update register"]
    pub mdma_c19brur: MDMA_C19BRUR,
    #[doc = "0x524 - MDMA channel Link Address register"]
    pub mdma_c19lar: MDMA_C19LAR,
    #[doc = "0x528 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c19tbr: MDMA_C19TBR,
    _reserved260: [u8; 4usize],
    #[doc = "0x530 - MDMA channel Mask address register"]
    pub mdma_c19mar: MDMA_C19MAR,
    #[doc = "0x534 - MDMA channel Mask Data register"]
    pub mdma_c19mdr: MDMA_C19MDR,
    _reserved262: [u8; 8usize],
    #[doc = "0x540 - MDMA channel 20 interrupt/status register"]
    pub mdma_c20isr: MDMA_C20ISR,
    #[doc = "0x544 - MDMA channel 20 interrupt flag clear register"]
    pub mdma_c20ifcr: MDMA_C20IFCR,
    #[doc = "0x548 - MDMA Channel 20 error status register"]
    pub mdma_c20esr: MDMA_C20ESR,
    #[doc = "0x54c - This register is used to control the concerned channel."]
    pub mdma_c20cr: MDMA_C20CR,
    #[doc = "0x550 - This register is used to configure the concerned channel."]
    pub mdma_c20tcr: MDMA_C20TCR,
    #[doc = "0x554 - MDMA Channel block number of data register"]
    pub mdma_c20bndtr: MDMA_C20BNDTR,
    #[doc = "0x558 - MDMA channel source address register"]
    pub mdma_c20sar: MDMA_C20SAR,
    #[doc = "0x55c - MDMA channel destination address register"]
    pub mdma_c20dar: MDMA_C20DAR,
    #[doc = "0x560 - MDMA channel Block Repeat address Update register"]
    pub mdma_c20brur: MDMA_C20BRUR,
    #[doc = "0x564 - MDMA channel Link Address register"]
    pub mdma_c20lar: MDMA_C20LAR,
    #[doc = "0x568 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c20tbr: MDMA_C20TBR,
    _reserved273: [u8; 4usize],
    #[doc = "0x570 - MDMA channel Mask address register"]
    pub mdma_c20mar: MDMA_C20MAR,
    #[doc = "0x574 - MDMA channel Mask Data register"]
    pub mdma_c20mdr: MDMA_C20MDR,
    _reserved275: [u8; 8usize],
    #[doc = "0x580 - MDMA channel 21 interrupt/status register"]
    pub mdma_c21isr: MDMA_C21ISR,
    #[doc = "0x584 - MDMA channel 21 interrupt flag clear register"]
    pub mdma_c21ifcr: MDMA_C21IFCR,
    #[doc = "0x588 - MDMA Channel 21 error status register"]
    pub mdma_c21esr: MDMA_C21ESR,
    #[doc = "0x58c - This register is used to control the concerned channel."]
    pub mdma_c21cr: MDMA_C21CR,
    #[doc = "0x590 - This register is used to configure the concerned channel."]
    pub mdma_c21tcr: MDMA_C21TCR,
    #[doc = "0x594 - MDMA Channel block number of data register"]
    pub mdma_c21bndtr: MDMA_C21BNDTR,
    #[doc = "0x598 - MDMA channel source address register"]
    pub mdma_c21sar: MDMA_C21SAR,
    #[doc = "0x59c - MDMA channel destination address register"]
    pub mdma_c21dar: MDMA_C21DAR,
    #[doc = "0x5a0 - MDMA channel Block Repeat address Update register"]
    pub mdma_c21brur: MDMA_C21BRUR,
    #[doc = "0x5a4 - MDMA channel Link Address register"]
    pub mdma_c21lar: MDMA_C21LAR,
    #[doc = "0x5a8 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c21tbr: MDMA_C21TBR,
    _reserved286: [u8; 4usize],
    #[doc = "0x5b0 - MDMA channel Mask address register"]
    pub mdma_c21mar: MDMA_C21MAR,
    #[doc = "0x5b4 - MDMA channel Mask Data register"]
    pub mdma_c21mdr: MDMA_C21MDR,
    _reserved288: [u8; 8usize],
    #[doc = "0x5c0 - MDMA channel 22 interrupt/status register"]
    pub mdma_c22isr: MDMA_C22ISR,
    #[doc = "0x5c4 - MDMA channel 22 interrupt flag clear register"]
    pub mdma_c22ifcr: MDMA_C22IFCR,
    #[doc = "0x5c8 - MDMA Channel 22 error status register"]
    pub mdma_c22esr: MDMA_C22ESR,
    #[doc = "0x5cc - This register is used to control the concerned channel."]
    pub mdma_c22cr: MDMA_C22CR,
    #[doc = "0x5d0 - This register is used to configure the concerned channel."]
    pub mdma_c22tcr: MDMA_C22TCR,
    #[doc = "0x5d4 - MDMA Channel block number of data register"]
    pub mdma_c22bndtr: MDMA_C22BNDTR,
    #[doc = "0x5d8 - MDMA channel source address register"]
    pub mdma_c22sar: MDMA_C22SAR,
    #[doc = "0x5dc - MDMA channel destination address register"]
    pub mdma_c22dar: MDMA_C22DAR,
    #[doc = "0x5e0 - MDMA channel Block Repeat address Update register"]
    pub mdma_c22brur: MDMA_C22BRUR,
    #[doc = "0x5e4 - MDMA channel Link Address register"]
    pub mdma_c22lar: MDMA_C22LAR,
    #[doc = "0x5e8 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c22tbr: MDMA_C22TBR,
    _reserved299: [u8; 4usize],
    #[doc = "0x5f0 - MDMA channel Mask address register"]
    pub mdma_c22mar: MDMA_C22MAR,
    #[doc = "0x5f4 - MDMA channel Mask Data register"]
    pub mdma_c22mdr: MDMA_C22MDR,
    _reserved301: [u8; 8usize],
    #[doc = "0x600 - MDMA channel 23 interrupt/status register"]
    pub mdma_c23isr: MDMA_C23ISR,
    #[doc = "0x604 - MDMA channel 23 interrupt flag clear register"]
    pub mdma_c23ifcr: MDMA_C23IFCR,
    #[doc = "0x608 - MDMA Channel 23 error status register"]
    pub mdma_c23esr: MDMA_C23ESR,
    #[doc = "0x60c - This register is used to control the concerned channel."]
    pub mdma_c23cr: MDMA_C23CR,
    #[doc = "0x610 - This register is used to configure the concerned channel."]
    pub mdma_c23tcr: MDMA_C23TCR,
    #[doc = "0x614 - MDMA Channel block number of data register"]
    pub mdma_c23bndtr: MDMA_C23BNDTR,
    #[doc = "0x618 - MDMA channel source address register"]
    pub mdma_c23sar: MDMA_C23SAR,
    #[doc = "0x61c - MDMA channel destination address register"]
    pub mdma_c23dar: MDMA_C23DAR,
    #[doc = "0x620 - MDMA channel Block Repeat address Update register"]
    pub mdma_c23brur: MDMA_C23BRUR,
    #[doc = "0x624 - MDMA channel Link Address register"]
    pub mdma_c23lar: MDMA_C23LAR,
    #[doc = "0x628 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c23tbr: MDMA_C23TBR,
    _reserved312: [u8; 4usize],
    #[doc = "0x630 - MDMA channel Mask address register"]
    pub mdma_c23mar: MDMA_C23MAR,
    #[doc = "0x634 - MDMA channel Mask Data register"]
    pub mdma_c23mdr: MDMA_C23MDR,
    _reserved314: [u8; 8usize],
    #[doc = "0x640 - MDMA channel 24 interrupt/status register"]
    pub mdma_c24isr: MDMA_C24ISR,
    #[doc = "0x644 - MDMA channel 24 interrupt flag clear register"]
    pub mdma_c24ifcr: MDMA_C24IFCR,
    #[doc = "0x648 - MDMA Channel 24 error status register"]
    pub mdma_c24esr: MDMA_C24ESR,
    #[doc = "0x64c - This register is used to control the concerned channel."]
    pub mdma_c24cr: MDMA_C24CR,
    #[doc = "0x650 - This register is used to configure the concerned channel."]
    pub mdma_c24tcr: MDMA_C24TCR,
    #[doc = "0x654 - MDMA Channel block number of data register"]
    pub mdma_c24bndtr: MDMA_C24BNDTR,
    #[doc = "0x658 - MDMA channel source address register"]
    pub mdma_c24sar: MDMA_C24SAR,
    #[doc = "0x65c - MDMA channel destination address register"]
    pub mdma_c24dar: MDMA_C24DAR,
    #[doc = "0x660 - MDMA channel Block Repeat address Update register"]
    pub mdma_c24brur: MDMA_C24BRUR,
    #[doc = "0x664 - MDMA channel Link Address register"]
    pub mdma_c24lar: MDMA_C24LAR,
    #[doc = "0x668 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c24tbr: MDMA_C24TBR,
    _reserved325: [u8; 4usize],
    #[doc = "0x670 - MDMA channel Mask address register"]
    pub mdma_c24mar: MDMA_C24MAR,
    #[doc = "0x674 - MDMA channel Mask Data register"]
    pub mdma_c24mdr: MDMA_C24MDR,
    _reserved327: [u8; 8usize],
    #[doc = "0x680 - MDMA channel 25 interrupt/status register"]
    pub mdma_c25isr: MDMA_C25ISR,
    #[doc = "0x684 - MDMA channel 25 interrupt flag clear register"]
    pub mdma_c25ifcr: MDMA_C25IFCR,
    #[doc = "0x688 - MDMA Channel 25 error status register"]
    pub mdma_c25esr: MDMA_C25ESR,
    #[doc = "0x68c - This register is used to control the concerned channel."]
    pub mdma_c25cr: MDMA_C25CR,
    #[doc = "0x690 - This register is used to configure the concerned channel."]
    pub mdma_c25tcr: MDMA_C25TCR,
    #[doc = "0x694 - MDMA Channel block number of data register"]
    pub mdma_c25bndtr: MDMA_C25BNDTR,
    #[doc = "0x698 - MDMA channel source address register"]
    pub mdma_c25sar: MDMA_C25SAR,
    #[doc = "0x69c - MDMA channel destination address register"]
    pub mdma_c25dar: MDMA_C25DAR,
    #[doc = "0x6a0 - MDMA channel Block Repeat address Update register"]
    pub mdma_c25brur: MDMA_C25BRUR,
    #[doc = "0x6a4 - MDMA channel Link Address register"]
    pub mdma_c25lar: MDMA_C25LAR,
    #[doc = "0x6a8 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c25tbr: MDMA_C25TBR,
    _reserved338: [u8; 4usize],
    #[doc = "0x6b0 - MDMA channel Mask address register"]
    pub mdma_c25mar: MDMA_C25MAR,
    #[doc = "0x6b4 - MDMA channel Mask Data register"]
    pub mdma_c25mdr: MDMA_C25MDR,
    _reserved340: [u8; 8usize],
    #[doc = "0x6c0 - MDMA channel 26 interrupt/status register"]
    pub mdma_c26isr: MDMA_C26ISR,
    #[doc = "0x6c4 - MDMA channel 26 interrupt flag clear register"]
    pub mdma_c26ifcr: MDMA_C26IFCR,
    #[doc = "0x6c8 - MDMA Channel 26 error status register"]
    pub mdma_c26esr: MDMA_C26ESR,
    #[doc = "0x6cc - This register is used to control the concerned channel."]
    pub mdma_c26cr: MDMA_C26CR,
    #[doc = "0x6d0 - This register is used to configure the concerned channel."]
    pub mdma_c26tcr: MDMA_C26TCR,
    #[doc = "0x6d4 - MDMA Channel block number of data register"]
    pub mdma_c26bndtr: MDMA_C26BNDTR,
    #[doc = "0x6d8 - MDMA channel source address register"]
    pub mdma_c26sar: MDMA_C26SAR,
    #[doc = "0x6dc - MDMA channel destination address register"]
    pub mdma_c26dar: MDMA_C26DAR,
    #[doc = "0x6e0 - MDMA channel Block Repeat address Update register"]
    pub mdma_c26brur: MDMA_C26BRUR,
    #[doc = "0x6e4 - MDMA channel Link Address register"]
    pub mdma_c26lar: MDMA_C26LAR,
    #[doc = "0x6e8 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c26tbr: MDMA_C26TBR,
    _reserved351: [u8; 4usize],
    #[doc = "0x6f0 - MDMA channel Mask address register"]
    pub mdma_c26mar: MDMA_C26MAR,
    #[doc = "0x6f4 - MDMA channel Mask Data register"]
    pub mdma_c26mdr: MDMA_C26MDR,
    _reserved353: [u8; 8usize],
    #[doc = "0x700 - MDMA channel 27 interrupt/status register"]
    pub mdma_c27isr: MDMA_C27ISR,
    #[doc = "0x704 - MDMA channel 27 interrupt flag clear register"]
    pub mdma_c27ifcr: MDMA_C27IFCR,
    #[doc = "0x708 - MDMA Channel 27 error status register"]
    pub mdma_c27esr: MDMA_C27ESR,
    #[doc = "0x70c - This register is used to control the concerned channel."]
    pub mdma_c27cr: MDMA_C27CR,
    #[doc = "0x710 - This register is used to configure the concerned channel."]
    pub mdma_c27tcr: MDMA_C27TCR,
    #[doc = "0x714 - MDMA Channel block number of data register"]
    pub mdma_c27bndtr: MDMA_C27BNDTR,
    #[doc = "0x718 - MDMA channel source address register"]
    pub mdma_c27sar: MDMA_C27SAR,
    #[doc = "0x71c - MDMA channel destination address register"]
    pub mdma_c27dar: MDMA_C27DAR,
    #[doc = "0x720 - MDMA channel Block Repeat address Update register"]
    pub mdma_c27brur: MDMA_C27BRUR,
    #[doc = "0x724 - MDMA channel Link Address register"]
    pub mdma_c27lar: MDMA_C27LAR,
    #[doc = "0x728 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c27tbr: MDMA_C27TBR,
    _reserved364: [u8; 4usize],
    #[doc = "0x730 - MDMA channel Mask address register"]
    pub mdma_c27mar: MDMA_C27MAR,
    #[doc = "0x734 - MDMA channel Mask Data register"]
    pub mdma_c27mdr: MDMA_C27MDR,
    _reserved366: [u8; 8usize],
    #[doc = "0x740 - MDMA channel 28 interrupt/status register"]
    pub mdma_c28isr: MDMA_C28ISR,
    #[doc = "0x744 - MDMA channel 28 interrupt flag clear register"]
    pub mdma_c28ifcr: MDMA_C28IFCR,
    #[doc = "0x748 - MDMA Channel 28 error status register"]
    pub mdma_c28esr: MDMA_C28ESR,
    #[doc = "0x74c - This register is used to control the concerned channel."]
    pub mdma_c28cr: MDMA_C28CR,
    #[doc = "0x750 - This register is used to configure the concerned channel."]
    pub mdma_c28tcr: MDMA_C28TCR,
    #[doc = "0x754 - MDMA Channel block number of data register"]
    pub mdma_c28bndtr: MDMA_C28BNDTR,
    #[doc = "0x758 - MDMA channel source address register"]
    pub mdma_c28sar: MDMA_C28SAR,
    #[doc = "0x75c - MDMA channel destination address register"]
    pub mdma_c28dar: MDMA_C28DAR,
    #[doc = "0x760 - MDMA channel Block Repeat address Update register"]
    pub mdma_c28brur: MDMA_C28BRUR,
    #[doc = "0x764 - MDMA channel Link Address register"]
    pub mdma_c28lar: MDMA_C28LAR,
    #[doc = "0x768 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c28tbr: MDMA_C28TBR,
    _reserved377: [u8; 4usize],
    #[doc = "0x770 - MDMA channel Mask address register"]
    pub mdma_c28mar: MDMA_C28MAR,
    #[doc = "0x774 - MDMA channel Mask Data register"]
    pub mdma_c28mdr: MDMA_C28MDR,
    _reserved379: [u8; 8usize],
    #[doc = "0x780 - MDMA channel 29 interrupt/status register"]
    pub mdma_c29isr: MDMA_C29ISR,
    #[doc = "0x784 - MDMA channel 29 interrupt flag clear register"]
    pub mdma_c29ifcr: MDMA_C29IFCR,
    #[doc = "0x788 - MDMA Channel 29 error status register"]
    pub mdma_c29esr: MDMA_C29ESR,
    #[doc = "0x78c - This register is used to control the concerned channel."]
    pub mdma_c29cr: MDMA_C29CR,
    #[doc = "0x790 - This register is used to configure the concerned channel."]
    pub mdma_c29tcr: MDMA_C29TCR,
    #[doc = "0x794 - MDMA Channel block number of data register"]
    pub mdma_c29bndtr: MDMA_C29BNDTR,
    #[doc = "0x798 - MDMA channel source address register"]
    pub mdma_c29sar: MDMA_C29SAR,
    #[doc = "0x79c - MDMA channel destination address register"]
    pub mdma_c29dar: MDMA_C29DAR,
    #[doc = "0x7a0 - MDMA channel Block Repeat address Update register"]
    pub mdma_c29brur: MDMA_C29BRUR,
    #[doc = "0x7a4 - MDMA channel Link Address register"]
    pub mdma_c29lar: MDMA_C29LAR,
    #[doc = "0x7a8 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c29tbr: MDMA_C29TBR,
    _reserved390: [u8; 4usize],
    #[doc = "0x7b0 - MDMA channel Mask address register"]
    pub mdma_c29mar: MDMA_C29MAR,
    #[doc = "0x7b4 - MDMA channel Mask Data register"]
    pub mdma_c29mdr: MDMA_C29MDR,
    _reserved392: [u8; 8usize],
    #[doc = "0x7c0 - MDMA channel 30 interrupt/status register"]
    pub mdma_c30isr: MDMA_C30ISR,
    #[doc = "0x7c4 - MDMA channel 30 interrupt flag clear register"]
    pub mdma_c30ifcr: MDMA_C30IFCR,
    #[doc = "0x7c8 - MDMA Channel 30 error status register"]
    pub mdma_c30esr: MDMA_C30ESR,
    #[doc = "0x7cc - This register is used to control the concerned channel."]
    pub mdma_c30cr: MDMA_C30CR,
    #[doc = "0x7d0 - This register is used to configure the concerned channel."]
    pub mdma_c30tcr: MDMA_C30TCR,
    #[doc = "0x7d4 - MDMA Channel block number of data register"]
    pub mdma_c30bndtr: MDMA_C30BNDTR,
    #[doc = "0x7d8 - MDMA channel source address register"]
    pub mdma_c30sar: MDMA_C30SAR,
    #[doc = "0x7dc - MDMA channel destination address register"]
    pub mdma_c30dar: MDMA_C30DAR,
    #[doc = "0x7e0 - MDMA channel Block Repeat address Update register"]
    pub mdma_c30brur: MDMA_C30BRUR,
    #[doc = "0x7e4 - MDMA channel Link Address register"]
    pub mdma_c30lar: MDMA_C30LAR,
    #[doc = "0x7e8 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c30tbr: MDMA_C30TBR,
    _reserved403: [u8; 4usize],
    #[doc = "0x7f0 - MDMA channel Mask address register"]
    pub mdma_c30mar: MDMA_C30MAR,
    #[doc = "0x7f4 - MDMA channel Mask Data register"]
    pub mdma_c30mdr: MDMA_C30MDR,
    _reserved405: [u8; 8usize],
    #[doc = "0x800 - MDMA channel 31 interrupt/status register"]
    pub mdma_c31isr: MDMA_C31ISR,
    #[doc = "0x804 - MDMA channel 31 interrupt flag clear register"]
    pub mdma_c31ifcr: MDMA_C31IFCR,
    #[doc = "0x808 - MDMA Channel 31 error status register"]
    pub mdma_c31esr: MDMA_C31ESR,
    #[doc = "0x80c - This register is used to control the concerned channel."]
    pub mdma_c31cr: MDMA_C31CR,
    #[doc = "0x810 - This register is used to configure the concerned channel."]
    pub mdma_c31tcr: MDMA_C31TCR,
    #[doc = "0x814 - MDMA Channel block number of data register"]
    pub mdma_c31bndtr: MDMA_C31BNDTR,
    #[doc = "0x818 - MDMA channel source address register"]
    pub mdma_c31sar: MDMA_C31SAR,
    #[doc = "0x81c - MDMA channel destination address register"]
    pub mdma_c31dar: MDMA_C31DAR,
    #[doc = "0x820 - MDMA channel Block Repeat address Update register"]
    pub mdma_c31brur: MDMA_C31BRUR,
    #[doc = "0x824 - MDMA channel Link Address register"]
    pub mdma_c31lar: MDMA_C31LAR,
    #[doc = "0x828 - MDMA channel Trigger and Bus selection Register"]
    pub mdma_c31tbr: MDMA_C31TBR,
    _reserved416: [u8; 4usize],
    #[doc = "0x830 - MDMA channel Mask address register"]
    pub mdma_c31mar: MDMA_C31MAR,
    #[doc = "0x834 - MDMA channel Mask Data register"]
    pub mdma_c31mdr: MDMA_C31MDR,
}
#[doc = "MDMA Global Interrupt/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_gisr0](mdma_gisr0) module"]
pub type MDMA_GISR0 = crate::Reg<u32, _MDMA_GISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_GISR0;
#[doc = "`read()` method returns [mdma_gisr0::R](mdma_gisr0::R) reader structure"]
impl crate::Readable for MDMA_GISR0 {}
#[doc = "MDMA Global Interrupt/Status Register"]
pub mod mdma_gisr0;
#[doc = "MDMA secure global interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_sgisr0](mdma_sgisr0) module"]
pub type MDMA_SGISR0 = crate::Reg<u32, _MDMA_SGISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_SGISR0;
#[doc = "`read()` method returns [mdma_sgisr0::R](mdma_sgisr0::R) reader structure"]
impl crate::Readable for MDMA_SGISR0 {}
#[doc = "MDMA secure global interrupt/status register"]
pub mod mdma_sgisr0;
#[doc = "MDMA channel 0 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0isr](mdma_c0isr) module"]
pub type MDMA_C0ISR = crate::Reg<u32, _MDMA_C0ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0ISR;
#[doc = "`read()` method returns [mdma_c0isr::R](mdma_c0isr::R) reader structure"]
impl crate::Readable for MDMA_C0ISR {}
#[doc = "MDMA channel 0 interrupt/status register"]
pub mod mdma_c0isr;
#[doc = "MDMA channel 0 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0ifcr](mdma_c0ifcr) module"]
pub type MDMA_C0IFCR = crate::Reg<u32, _MDMA_C0IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0IFCR;
#[doc = "`read()` method returns [mdma_c0ifcr::R](mdma_c0ifcr::R) reader structure"]
impl crate::Readable for MDMA_C0IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0ifcr::W](mdma_c0ifcr::W) writer structure"]
impl crate::Writable for MDMA_C0IFCR {}
#[doc = "MDMA channel 0 interrupt flag clear register"]
pub mod mdma_c0ifcr;
#[doc = "MDMA Channel 0 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0esr](mdma_c0esr) module"]
pub type MDMA_C0ESR = crate::Reg<u32, _MDMA_C0ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0ESR;
#[doc = "`read()` method returns [mdma_c0esr::R](mdma_c0esr::R) reader structure"]
impl crate::Readable for MDMA_C0ESR {}
#[doc = "MDMA Channel 0 error status register"]
pub mod mdma_c0esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0cr](mdma_c0cr) module"]
pub type MDMA_C0CR = crate::Reg<u32, _MDMA_C0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0CR;
#[doc = "`read()` method returns [mdma_c0cr::R](mdma_c0cr::R) reader structure"]
impl crate::Readable for MDMA_C0CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0cr::W](mdma_c0cr::W) writer structure"]
impl crate::Writable for MDMA_C0CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c0cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0tcr](mdma_c0tcr) module"]
pub type MDMA_C0TCR = crate::Reg<u32, _MDMA_C0TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0TCR;
#[doc = "`read()` method returns [mdma_c0tcr::R](mdma_c0tcr::R) reader structure"]
impl crate::Readable for MDMA_C0TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0tcr::W](mdma_c0tcr::W) writer structure"]
impl crate::Writable for MDMA_C0TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c0tcr;
#[doc = "MDMA Channel 0 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0bndtr](mdma_c0bndtr) module"]
pub type MDMA_C0BNDTR = crate::Reg<u32, _MDMA_C0BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0BNDTR;
#[doc = "`read()` method returns [mdma_c0bndtr::R](mdma_c0bndtr::R) reader structure"]
impl crate::Readable for MDMA_C0BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0bndtr::W](mdma_c0bndtr::W) writer structure"]
impl crate::Writable for MDMA_C0BNDTR {}
#[doc = "MDMA Channel 0 block number of data register"]
pub mod mdma_c0bndtr;
#[doc = "MDMA channel 0 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0sar](mdma_c0sar) module"]
pub type MDMA_C0SAR = crate::Reg<u32, _MDMA_C0SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0SAR;
#[doc = "`read()` method returns [mdma_c0sar::R](mdma_c0sar::R) reader structure"]
impl crate::Readable for MDMA_C0SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0sar::W](mdma_c0sar::W) writer structure"]
impl crate::Writable for MDMA_C0SAR {}
#[doc = "MDMA channel 0 source address register"]
pub mod mdma_c0sar;
#[doc = "MDMA channel 0 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0dar](mdma_c0dar) module"]
pub type MDMA_C0DAR = crate::Reg<u32, _MDMA_C0DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0DAR;
#[doc = "`read()` method returns [mdma_c0dar::R](mdma_c0dar::R) reader structure"]
impl crate::Readable for MDMA_C0DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0dar::W](mdma_c0dar::W) writer structure"]
impl crate::Writable for MDMA_C0DAR {}
#[doc = "MDMA channel 0 destination address register"]
pub mod mdma_c0dar;
#[doc = "MDMA channel 0 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0brur](mdma_c0brur) module"]
pub type MDMA_C0BRUR = crate::Reg<u32, _MDMA_C0BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0BRUR;
#[doc = "`read()` method returns [mdma_c0brur::R](mdma_c0brur::R) reader structure"]
impl crate::Readable for MDMA_C0BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0brur::W](mdma_c0brur::W) writer structure"]
impl crate::Writable for MDMA_C0BRUR {}
#[doc = "MDMA channel 0 Block Repeat address Update register"]
pub mod mdma_c0brur;
#[doc = "MDMA channel 0 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0lar](mdma_c0lar) module"]
pub type MDMA_C0LAR = crate::Reg<u32, _MDMA_C0LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0LAR;
#[doc = "`read()` method returns [mdma_c0lar::R](mdma_c0lar::R) reader structure"]
impl crate::Readable for MDMA_C0LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0lar::W](mdma_c0lar::W) writer structure"]
impl crate::Writable for MDMA_C0LAR {}
#[doc = "MDMA channel 0 Link Address register"]
pub mod mdma_c0lar;
#[doc = "MDMA channel 0 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0tbr](mdma_c0tbr) module"]
pub type MDMA_C0TBR = crate::Reg<u32, _MDMA_C0TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0TBR;
#[doc = "`read()` method returns [mdma_c0tbr::R](mdma_c0tbr::R) reader structure"]
impl crate::Readable for MDMA_C0TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0tbr::W](mdma_c0tbr::W) writer structure"]
impl crate::Writable for MDMA_C0TBR {}
#[doc = "MDMA channel 0 Trigger and Bus selection Register"]
pub mod mdma_c0tbr;
#[doc = "MDMA channel 0 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0mar](mdma_c0mar) module"]
pub type MDMA_C0MAR = crate::Reg<u32, _MDMA_C0MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0MAR;
#[doc = "`read()` method returns [mdma_c0mar::R](mdma_c0mar::R) reader structure"]
impl crate::Readable for MDMA_C0MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0mar::W](mdma_c0mar::W) writer structure"]
impl crate::Writable for MDMA_C0MAR {}
#[doc = "MDMA channel 0 Mask address register"]
pub mod mdma_c0mar;
#[doc = "MDMA channel 0 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c0mdr](mdma_c0mdr) module"]
pub type MDMA_C0MDR = crate::Reg<u32, _MDMA_C0MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C0MDR;
#[doc = "`read()` method returns [mdma_c0mdr::R](mdma_c0mdr::R) reader structure"]
impl crate::Readable for MDMA_C0MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c0mdr::W](mdma_c0mdr::W) writer structure"]
impl crate::Writable for MDMA_C0MDR {}
#[doc = "MDMA channel 0 Mask Data register"]
pub mod mdma_c0mdr;
#[doc = "MDMA channel 1 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1isr](mdma_c1isr) module"]
pub type MDMA_C1ISR = crate::Reg<u32, _MDMA_C1ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1ISR;
#[doc = "`read()` method returns [mdma_c1isr::R](mdma_c1isr::R) reader structure"]
impl crate::Readable for MDMA_C1ISR {}
#[doc = "MDMA channel 1 interrupt/status register"]
pub mod mdma_c1isr;
#[doc = "MDMA channel 1 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1ifcr](mdma_c1ifcr) module"]
pub type MDMA_C1IFCR = crate::Reg<u32, _MDMA_C1IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1IFCR;
#[doc = "`read()` method returns [mdma_c1ifcr::R](mdma_c1ifcr::R) reader structure"]
impl crate::Readable for MDMA_C1IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1ifcr::W](mdma_c1ifcr::W) writer structure"]
impl crate::Writable for MDMA_C1IFCR {}
#[doc = "MDMA channel 1 interrupt flag clear register"]
pub mod mdma_c1ifcr;
#[doc = "MDMA Channel 1 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1esr](mdma_c1esr) module"]
pub type MDMA_C1ESR = crate::Reg<u32, _MDMA_C1ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1ESR;
#[doc = "`read()` method returns [mdma_c1esr::R](mdma_c1esr::R) reader structure"]
impl crate::Readable for MDMA_C1ESR {}
#[doc = "MDMA Channel 1 error status register"]
pub mod mdma_c1esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1cr](mdma_c1cr) module"]
pub type MDMA_C1CR = crate::Reg<u32, _MDMA_C1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1CR;
#[doc = "`read()` method returns [mdma_c1cr::R](mdma_c1cr::R) reader structure"]
impl crate::Readable for MDMA_C1CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1cr::W](mdma_c1cr::W) writer structure"]
impl crate::Writable for MDMA_C1CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c1cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1tcr](mdma_c1tcr) module"]
pub type MDMA_C1TCR = crate::Reg<u32, _MDMA_C1TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1TCR;
#[doc = "`read()` method returns [mdma_c1tcr::R](mdma_c1tcr::R) reader structure"]
impl crate::Readable for MDMA_C1TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1tcr::W](mdma_c1tcr::W) writer structure"]
impl crate::Writable for MDMA_C1TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c1tcr;
#[doc = "MDMA Channel 1 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1bndtr](mdma_c1bndtr) module"]
pub type MDMA_C1BNDTR = crate::Reg<u32, _MDMA_C1BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1BNDTR;
#[doc = "`read()` method returns [mdma_c1bndtr::R](mdma_c1bndtr::R) reader structure"]
impl crate::Readable for MDMA_C1BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1bndtr::W](mdma_c1bndtr::W) writer structure"]
impl crate::Writable for MDMA_C1BNDTR {}
#[doc = "MDMA Channel 1 block number of data register"]
pub mod mdma_c1bndtr;
#[doc = "MDMA channel 1 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1sar](mdma_c1sar) module"]
pub type MDMA_C1SAR = crate::Reg<u32, _MDMA_C1SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1SAR;
#[doc = "`read()` method returns [mdma_c1sar::R](mdma_c1sar::R) reader structure"]
impl crate::Readable for MDMA_C1SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1sar::W](mdma_c1sar::W) writer structure"]
impl crate::Writable for MDMA_C1SAR {}
#[doc = "MDMA channel 1 source address register"]
pub mod mdma_c1sar;
#[doc = "MDMA channel 1 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1dar](mdma_c1dar) module"]
pub type MDMA_C1DAR = crate::Reg<u32, _MDMA_C1DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1DAR;
#[doc = "`read()` method returns [mdma_c1dar::R](mdma_c1dar::R) reader structure"]
impl crate::Readable for MDMA_C1DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1dar::W](mdma_c1dar::W) writer structure"]
impl crate::Writable for MDMA_C1DAR {}
#[doc = "MDMA channel 1 destination address register"]
pub mod mdma_c1dar;
#[doc = "MDMA channel 1 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1brur](mdma_c1brur) module"]
pub type MDMA_C1BRUR = crate::Reg<u32, _MDMA_C1BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1BRUR;
#[doc = "`read()` method returns [mdma_c1brur::R](mdma_c1brur::R) reader structure"]
impl crate::Readable for MDMA_C1BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1brur::W](mdma_c1brur::W) writer structure"]
impl crate::Writable for MDMA_C1BRUR {}
#[doc = "MDMA channel 1 Block Repeat address Update register"]
pub mod mdma_c1brur;
#[doc = "MDMA channel 1 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1lar](mdma_c1lar) module"]
pub type MDMA_C1LAR = crate::Reg<u32, _MDMA_C1LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1LAR;
#[doc = "`read()` method returns [mdma_c1lar::R](mdma_c1lar::R) reader structure"]
impl crate::Readable for MDMA_C1LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1lar::W](mdma_c1lar::W) writer structure"]
impl crate::Writable for MDMA_C1LAR {}
#[doc = "MDMA channel 1 Link Address register"]
pub mod mdma_c1lar;
#[doc = "MDMA channel 1 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1tbr](mdma_c1tbr) module"]
pub type MDMA_C1TBR = crate::Reg<u32, _MDMA_C1TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1TBR;
#[doc = "`read()` method returns [mdma_c1tbr::R](mdma_c1tbr::R) reader structure"]
impl crate::Readable for MDMA_C1TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1tbr::W](mdma_c1tbr::W) writer structure"]
impl crate::Writable for MDMA_C1TBR {}
#[doc = "MDMA channel 1 Trigger and Bus selection Register"]
pub mod mdma_c1tbr;
#[doc = "MDMA channel 1 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1mar](mdma_c1mar) module"]
pub type MDMA_C1MAR = crate::Reg<u32, _MDMA_C1MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1MAR;
#[doc = "`read()` method returns [mdma_c1mar::R](mdma_c1mar::R) reader structure"]
impl crate::Readable for MDMA_C1MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1mar::W](mdma_c1mar::W) writer structure"]
impl crate::Writable for MDMA_C1MAR {}
#[doc = "MDMA channel 1 Mask address register"]
pub mod mdma_c1mar;
#[doc = "MDMA channel 1 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c1mdr](mdma_c1mdr) module"]
pub type MDMA_C1MDR = crate::Reg<u32, _MDMA_C1MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C1MDR;
#[doc = "`read()` method returns [mdma_c1mdr::R](mdma_c1mdr::R) reader structure"]
impl crate::Readable for MDMA_C1MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c1mdr::W](mdma_c1mdr::W) writer structure"]
impl crate::Writable for MDMA_C1MDR {}
#[doc = "MDMA channel 1 Mask Data register"]
pub mod mdma_c1mdr;
#[doc = "MDMA channel 2 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2isr](mdma_c2isr) module"]
pub type MDMA_C2ISR = crate::Reg<u32, _MDMA_C2ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2ISR;
#[doc = "`read()` method returns [mdma_c2isr::R](mdma_c2isr::R) reader structure"]
impl crate::Readable for MDMA_C2ISR {}
#[doc = "MDMA channel 2 interrupt/status register"]
pub mod mdma_c2isr;
#[doc = "MDMA channel 2 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2ifcr](mdma_c2ifcr) module"]
pub type MDMA_C2IFCR = crate::Reg<u32, _MDMA_C2IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2IFCR;
#[doc = "`read()` method returns [mdma_c2ifcr::R](mdma_c2ifcr::R) reader structure"]
impl crate::Readable for MDMA_C2IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2ifcr::W](mdma_c2ifcr::W) writer structure"]
impl crate::Writable for MDMA_C2IFCR {}
#[doc = "MDMA channel 2 interrupt flag clear register"]
pub mod mdma_c2ifcr;
#[doc = "MDMA Channel 2 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2esr](mdma_c2esr) module"]
pub type MDMA_C2ESR = crate::Reg<u32, _MDMA_C2ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2ESR;
#[doc = "`read()` method returns [mdma_c2esr::R](mdma_c2esr::R) reader structure"]
impl crate::Readable for MDMA_C2ESR {}
#[doc = "MDMA Channel 2 error status register"]
pub mod mdma_c2esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2cr](mdma_c2cr) module"]
pub type MDMA_C2CR = crate::Reg<u32, _MDMA_C2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2CR;
#[doc = "`read()` method returns [mdma_c2cr::R](mdma_c2cr::R) reader structure"]
impl crate::Readable for MDMA_C2CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2cr::W](mdma_c2cr::W) writer structure"]
impl crate::Writable for MDMA_C2CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c2cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2tcr](mdma_c2tcr) module"]
pub type MDMA_C2TCR = crate::Reg<u32, _MDMA_C2TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2TCR;
#[doc = "`read()` method returns [mdma_c2tcr::R](mdma_c2tcr::R) reader structure"]
impl crate::Readable for MDMA_C2TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2tcr::W](mdma_c2tcr::W) writer structure"]
impl crate::Writable for MDMA_C2TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c2tcr;
#[doc = "MDMA Channel 2 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2bndtr](mdma_c2bndtr) module"]
pub type MDMA_C2BNDTR = crate::Reg<u32, _MDMA_C2BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2BNDTR;
#[doc = "`read()` method returns [mdma_c2bndtr::R](mdma_c2bndtr::R) reader structure"]
impl crate::Readable for MDMA_C2BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2bndtr::W](mdma_c2bndtr::W) writer structure"]
impl crate::Writable for MDMA_C2BNDTR {}
#[doc = "MDMA Channel 2 block number of data register"]
pub mod mdma_c2bndtr;
#[doc = "MDMA channel 2 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2sar](mdma_c2sar) module"]
pub type MDMA_C2SAR = crate::Reg<u32, _MDMA_C2SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2SAR;
#[doc = "`read()` method returns [mdma_c2sar::R](mdma_c2sar::R) reader structure"]
impl crate::Readable for MDMA_C2SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2sar::W](mdma_c2sar::W) writer structure"]
impl crate::Writable for MDMA_C2SAR {}
#[doc = "MDMA channel 2 source address register"]
pub mod mdma_c2sar;
#[doc = "MDMA channel 2 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2dar](mdma_c2dar) module"]
pub type MDMA_C2DAR = crate::Reg<u32, _MDMA_C2DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2DAR;
#[doc = "`read()` method returns [mdma_c2dar::R](mdma_c2dar::R) reader structure"]
impl crate::Readable for MDMA_C2DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2dar::W](mdma_c2dar::W) writer structure"]
impl crate::Writable for MDMA_C2DAR {}
#[doc = "MDMA channel 2 destination address register"]
pub mod mdma_c2dar;
#[doc = "MDMA channel 2 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2brur](mdma_c2brur) module"]
pub type MDMA_C2BRUR = crate::Reg<u32, _MDMA_C2BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2BRUR;
#[doc = "`read()` method returns [mdma_c2brur::R](mdma_c2brur::R) reader structure"]
impl crate::Readable for MDMA_C2BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2brur::W](mdma_c2brur::W) writer structure"]
impl crate::Writable for MDMA_C2BRUR {}
#[doc = "MDMA channel 2 Block Repeat address Update register"]
pub mod mdma_c2brur;
#[doc = "MDMA channel 2 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2lar](mdma_c2lar) module"]
pub type MDMA_C2LAR = crate::Reg<u32, _MDMA_C2LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2LAR;
#[doc = "`read()` method returns [mdma_c2lar::R](mdma_c2lar::R) reader structure"]
impl crate::Readable for MDMA_C2LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2lar::W](mdma_c2lar::W) writer structure"]
impl crate::Writable for MDMA_C2LAR {}
#[doc = "MDMA channel 2 Link Address register"]
pub mod mdma_c2lar;
#[doc = "MDMA channel 2 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2tbr](mdma_c2tbr) module"]
pub type MDMA_C2TBR = crate::Reg<u32, _MDMA_C2TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2TBR;
#[doc = "`read()` method returns [mdma_c2tbr::R](mdma_c2tbr::R) reader structure"]
impl crate::Readable for MDMA_C2TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2tbr::W](mdma_c2tbr::W) writer structure"]
impl crate::Writable for MDMA_C2TBR {}
#[doc = "MDMA channel 2 Trigger and Bus selection Register"]
pub mod mdma_c2tbr;
#[doc = "MDMA channel 2 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2mar](mdma_c2mar) module"]
pub type MDMA_C2MAR = crate::Reg<u32, _MDMA_C2MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2MAR;
#[doc = "`read()` method returns [mdma_c2mar::R](mdma_c2mar::R) reader structure"]
impl crate::Readable for MDMA_C2MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2mar::W](mdma_c2mar::W) writer structure"]
impl crate::Writable for MDMA_C2MAR {}
#[doc = "MDMA channel 2 Mask address register"]
pub mod mdma_c2mar;
#[doc = "MDMA channel 2 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c2mdr](mdma_c2mdr) module"]
pub type MDMA_C2MDR = crate::Reg<u32, _MDMA_C2MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C2MDR;
#[doc = "`read()` method returns [mdma_c2mdr::R](mdma_c2mdr::R) reader structure"]
impl crate::Readable for MDMA_C2MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c2mdr::W](mdma_c2mdr::W) writer structure"]
impl crate::Writable for MDMA_C2MDR {}
#[doc = "MDMA channel 2 Mask Data register"]
pub mod mdma_c2mdr;
#[doc = "MDMA channel 3 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3isr](mdma_c3isr) module"]
pub type MDMA_C3ISR = crate::Reg<u32, _MDMA_C3ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3ISR;
#[doc = "`read()` method returns [mdma_c3isr::R](mdma_c3isr::R) reader structure"]
impl crate::Readable for MDMA_C3ISR {}
#[doc = "MDMA channel 3 interrupt/status register"]
pub mod mdma_c3isr;
#[doc = "MDMA channel 3 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3ifcr](mdma_c3ifcr) module"]
pub type MDMA_C3IFCR = crate::Reg<u32, _MDMA_C3IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3IFCR;
#[doc = "`read()` method returns [mdma_c3ifcr::R](mdma_c3ifcr::R) reader structure"]
impl crate::Readable for MDMA_C3IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3ifcr::W](mdma_c3ifcr::W) writer structure"]
impl crate::Writable for MDMA_C3IFCR {}
#[doc = "MDMA channel 3 interrupt flag clear register"]
pub mod mdma_c3ifcr;
#[doc = "MDMA Channel 3 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3esr](mdma_c3esr) module"]
pub type MDMA_C3ESR = crate::Reg<u32, _MDMA_C3ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3ESR;
#[doc = "`read()` method returns [mdma_c3esr::R](mdma_c3esr::R) reader structure"]
impl crate::Readable for MDMA_C3ESR {}
#[doc = "MDMA Channel 3 error status register"]
pub mod mdma_c3esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3cr](mdma_c3cr) module"]
pub type MDMA_C3CR = crate::Reg<u32, _MDMA_C3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3CR;
#[doc = "`read()` method returns [mdma_c3cr::R](mdma_c3cr::R) reader structure"]
impl crate::Readable for MDMA_C3CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3cr::W](mdma_c3cr::W) writer structure"]
impl crate::Writable for MDMA_C3CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c3cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3tcr](mdma_c3tcr) module"]
pub type MDMA_C3TCR = crate::Reg<u32, _MDMA_C3TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3TCR;
#[doc = "`read()` method returns [mdma_c3tcr::R](mdma_c3tcr::R) reader structure"]
impl crate::Readable for MDMA_C3TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3tcr::W](mdma_c3tcr::W) writer structure"]
impl crate::Writable for MDMA_C3TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c3tcr;
#[doc = "MDMA Channel 3 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3bndtr](mdma_c3bndtr) module"]
pub type MDMA_C3BNDTR = crate::Reg<u32, _MDMA_C3BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3BNDTR;
#[doc = "`read()` method returns [mdma_c3bndtr::R](mdma_c3bndtr::R) reader structure"]
impl crate::Readable for MDMA_C3BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3bndtr::W](mdma_c3bndtr::W) writer structure"]
impl crate::Writable for MDMA_C3BNDTR {}
#[doc = "MDMA Channel 3 block number of data register"]
pub mod mdma_c3bndtr;
#[doc = "MDMA channel 3 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3sar](mdma_c3sar) module"]
pub type MDMA_C3SAR = crate::Reg<u32, _MDMA_C3SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3SAR;
#[doc = "`read()` method returns [mdma_c3sar::R](mdma_c3sar::R) reader structure"]
impl crate::Readable for MDMA_C3SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3sar::W](mdma_c3sar::W) writer structure"]
impl crate::Writable for MDMA_C3SAR {}
#[doc = "MDMA channel 3 source address register"]
pub mod mdma_c3sar;
#[doc = "MDMA channel 3 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3dar](mdma_c3dar) module"]
pub type MDMA_C3DAR = crate::Reg<u32, _MDMA_C3DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3DAR;
#[doc = "`read()` method returns [mdma_c3dar::R](mdma_c3dar::R) reader structure"]
impl crate::Readable for MDMA_C3DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3dar::W](mdma_c3dar::W) writer structure"]
impl crate::Writable for MDMA_C3DAR {}
#[doc = "MDMA channel 3 destination address register"]
pub mod mdma_c3dar;
#[doc = "MDMA channel 3 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3brur](mdma_c3brur) module"]
pub type MDMA_C3BRUR = crate::Reg<u32, _MDMA_C3BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3BRUR;
#[doc = "`read()` method returns [mdma_c3brur::R](mdma_c3brur::R) reader structure"]
impl crate::Readable for MDMA_C3BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3brur::W](mdma_c3brur::W) writer structure"]
impl crate::Writable for MDMA_C3BRUR {}
#[doc = "MDMA channel 3 Block Repeat address Update register"]
pub mod mdma_c3brur;
#[doc = "MDMA channel 3 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3lar](mdma_c3lar) module"]
pub type MDMA_C3LAR = crate::Reg<u32, _MDMA_C3LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3LAR;
#[doc = "`read()` method returns [mdma_c3lar::R](mdma_c3lar::R) reader structure"]
impl crate::Readable for MDMA_C3LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3lar::W](mdma_c3lar::W) writer structure"]
impl crate::Writable for MDMA_C3LAR {}
#[doc = "MDMA channel 3 Link Address register"]
pub mod mdma_c3lar;
#[doc = "MDMA channel 3 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3tbr](mdma_c3tbr) module"]
pub type MDMA_C3TBR = crate::Reg<u32, _MDMA_C3TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3TBR;
#[doc = "`read()` method returns [mdma_c3tbr::R](mdma_c3tbr::R) reader structure"]
impl crate::Readable for MDMA_C3TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3tbr::W](mdma_c3tbr::W) writer structure"]
impl crate::Writable for MDMA_C3TBR {}
#[doc = "MDMA channel 3 Trigger and Bus selection Register"]
pub mod mdma_c3tbr;
#[doc = "MDMA channel 3 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3mar](mdma_c3mar) module"]
pub type MDMA_C3MAR = crate::Reg<u32, _MDMA_C3MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3MAR;
#[doc = "`read()` method returns [mdma_c3mar::R](mdma_c3mar::R) reader structure"]
impl crate::Readable for MDMA_C3MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3mar::W](mdma_c3mar::W) writer structure"]
impl crate::Writable for MDMA_C3MAR {}
#[doc = "MDMA channel 3 Mask address register"]
pub mod mdma_c3mar;
#[doc = "MDMA channel 3 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c3mdr](mdma_c3mdr) module"]
pub type MDMA_C3MDR = crate::Reg<u32, _MDMA_C3MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C3MDR;
#[doc = "`read()` method returns [mdma_c3mdr::R](mdma_c3mdr::R) reader structure"]
impl crate::Readable for MDMA_C3MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c3mdr::W](mdma_c3mdr::W) writer structure"]
impl crate::Writable for MDMA_C3MDR {}
#[doc = "MDMA channel 3 Mask Data register"]
pub mod mdma_c3mdr;
#[doc = "MDMA channel 4 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4isr](mdma_c4isr) module"]
pub type MDMA_C4ISR = crate::Reg<u32, _MDMA_C4ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4ISR;
#[doc = "`read()` method returns [mdma_c4isr::R](mdma_c4isr::R) reader structure"]
impl crate::Readable for MDMA_C4ISR {}
#[doc = "MDMA channel 4 interrupt/status register"]
pub mod mdma_c4isr;
#[doc = "MDMA channel 4 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4ifcr](mdma_c4ifcr) module"]
pub type MDMA_C4IFCR = crate::Reg<u32, _MDMA_C4IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4IFCR;
#[doc = "`read()` method returns [mdma_c4ifcr::R](mdma_c4ifcr::R) reader structure"]
impl crate::Readable for MDMA_C4IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4ifcr::W](mdma_c4ifcr::W) writer structure"]
impl crate::Writable for MDMA_C4IFCR {}
#[doc = "MDMA channel 4 interrupt flag clear register"]
pub mod mdma_c4ifcr;
#[doc = "MDMA Channel 4 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4esr](mdma_c4esr) module"]
pub type MDMA_C4ESR = crate::Reg<u32, _MDMA_C4ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4ESR;
#[doc = "`read()` method returns [mdma_c4esr::R](mdma_c4esr::R) reader structure"]
impl crate::Readable for MDMA_C4ESR {}
#[doc = "MDMA Channel 4 error status register"]
pub mod mdma_c4esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4cr](mdma_c4cr) module"]
pub type MDMA_C4CR = crate::Reg<u32, _MDMA_C4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4CR;
#[doc = "`read()` method returns [mdma_c4cr::R](mdma_c4cr::R) reader structure"]
impl crate::Readable for MDMA_C4CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4cr::W](mdma_c4cr::W) writer structure"]
impl crate::Writable for MDMA_C4CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c4cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4tcr](mdma_c4tcr) module"]
pub type MDMA_C4TCR = crate::Reg<u32, _MDMA_C4TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4TCR;
#[doc = "`read()` method returns [mdma_c4tcr::R](mdma_c4tcr::R) reader structure"]
impl crate::Readable for MDMA_C4TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4tcr::W](mdma_c4tcr::W) writer structure"]
impl crate::Writable for MDMA_C4TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c4tcr;
#[doc = "MDMA Channel 4 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4bndtr](mdma_c4bndtr) module"]
pub type MDMA_C4BNDTR = crate::Reg<u32, _MDMA_C4BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4BNDTR;
#[doc = "`read()` method returns [mdma_c4bndtr::R](mdma_c4bndtr::R) reader structure"]
impl crate::Readable for MDMA_C4BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4bndtr::W](mdma_c4bndtr::W) writer structure"]
impl crate::Writable for MDMA_C4BNDTR {}
#[doc = "MDMA Channel 4 block number of data register"]
pub mod mdma_c4bndtr;
#[doc = "MDMA channel 4 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4sar](mdma_c4sar) module"]
pub type MDMA_C4SAR = crate::Reg<u32, _MDMA_C4SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4SAR;
#[doc = "`read()` method returns [mdma_c4sar::R](mdma_c4sar::R) reader structure"]
impl crate::Readable for MDMA_C4SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4sar::W](mdma_c4sar::W) writer structure"]
impl crate::Writable for MDMA_C4SAR {}
#[doc = "MDMA channel 4 source address register"]
pub mod mdma_c4sar;
#[doc = "MDMA channel 4 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4dar](mdma_c4dar) module"]
pub type MDMA_C4DAR = crate::Reg<u32, _MDMA_C4DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4DAR;
#[doc = "`read()` method returns [mdma_c4dar::R](mdma_c4dar::R) reader structure"]
impl crate::Readable for MDMA_C4DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4dar::W](mdma_c4dar::W) writer structure"]
impl crate::Writable for MDMA_C4DAR {}
#[doc = "MDMA channel 4 destination address register"]
pub mod mdma_c4dar;
#[doc = "MDMA channel 4 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4brur](mdma_c4brur) module"]
pub type MDMA_C4BRUR = crate::Reg<u32, _MDMA_C4BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4BRUR;
#[doc = "`read()` method returns [mdma_c4brur::R](mdma_c4brur::R) reader structure"]
impl crate::Readable for MDMA_C4BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4brur::W](mdma_c4brur::W) writer structure"]
impl crate::Writable for MDMA_C4BRUR {}
#[doc = "MDMA channel 4 Block Repeat address Update register"]
pub mod mdma_c4brur;
#[doc = "MDMA channel 4 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4lar](mdma_c4lar) module"]
pub type MDMA_C4LAR = crate::Reg<u32, _MDMA_C4LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4LAR;
#[doc = "`read()` method returns [mdma_c4lar::R](mdma_c4lar::R) reader structure"]
impl crate::Readable for MDMA_C4LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4lar::W](mdma_c4lar::W) writer structure"]
impl crate::Writable for MDMA_C4LAR {}
#[doc = "MDMA channel 4 Link Address register"]
pub mod mdma_c4lar;
#[doc = "MDMA channel 4 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4tbr](mdma_c4tbr) module"]
pub type MDMA_C4TBR = crate::Reg<u32, _MDMA_C4TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4TBR;
#[doc = "`read()` method returns [mdma_c4tbr::R](mdma_c4tbr::R) reader structure"]
impl crate::Readable for MDMA_C4TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4tbr::W](mdma_c4tbr::W) writer structure"]
impl crate::Writable for MDMA_C4TBR {}
#[doc = "MDMA channel 4 Trigger and Bus selection Register"]
pub mod mdma_c4tbr;
#[doc = "MDMA channel 4 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4mar](mdma_c4mar) module"]
pub type MDMA_C4MAR = crate::Reg<u32, _MDMA_C4MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4MAR;
#[doc = "`read()` method returns [mdma_c4mar::R](mdma_c4mar::R) reader structure"]
impl crate::Readable for MDMA_C4MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4mar::W](mdma_c4mar::W) writer structure"]
impl crate::Writable for MDMA_C4MAR {}
#[doc = "MDMA channel 4 Mask address register"]
pub mod mdma_c4mar;
#[doc = "MDMA channel 4 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4mdr](mdma_c4mdr) module"]
pub type MDMA_C4MDR = crate::Reg<u32, _MDMA_C4MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C4MDR;
#[doc = "`read()` method returns [mdma_c4mdr::R](mdma_c4mdr::R) reader structure"]
impl crate::Readable for MDMA_C4MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c4mdr::W](mdma_c4mdr::W) writer structure"]
impl crate::Writable for MDMA_C4MDR {}
#[doc = "MDMA channel 4 Mask Data register"]
pub mod mdma_c4mdr;
#[doc = "MDMA channel 5 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5isr](mdma_c5isr) module"]
pub type MDMA_C5ISR = crate::Reg<u32, _MDMA_C5ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5ISR;
#[doc = "`read()` method returns [mdma_c5isr::R](mdma_c5isr::R) reader structure"]
impl crate::Readable for MDMA_C5ISR {}
#[doc = "MDMA channel 5 interrupt/status register"]
pub mod mdma_c5isr;
#[doc = "MDMA channel 5 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5ifcr](mdma_c5ifcr) module"]
pub type MDMA_C5IFCR = crate::Reg<u32, _MDMA_C5IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5IFCR;
#[doc = "`read()` method returns [mdma_c5ifcr::R](mdma_c5ifcr::R) reader structure"]
impl crate::Readable for MDMA_C5IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5ifcr::W](mdma_c5ifcr::W) writer structure"]
impl crate::Writable for MDMA_C5IFCR {}
#[doc = "MDMA channel 5 interrupt flag clear register"]
pub mod mdma_c5ifcr;
#[doc = "MDMA Channel 5 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5esr](mdma_c5esr) module"]
pub type MDMA_C5ESR = crate::Reg<u32, _MDMA_C5ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5ESR;
#[doc = "`read()` method returns [mdma_c5esr::R](mdma_c5esr::R) reader structure"]
impl crate::Readable for MDMA_C5ESR {}
#[doc = "MDMA Channel 5 error status register"]
pub mod mdma_c5esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5cr](mdma_c5cr) module"]
pub type MDMA_C5CR = crate::Reg<u32, _MDMA_C5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5CR;
#[doc = "`read()` method returns [mdma_c5cr::R](mdma_c5cr::R) reader structure"]
impl crate::Readable for MDMA_C5CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5cr::W](mdma_c5cr::W) writer structure"]
impl crate::Writable for MDMA_C5CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c5cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5tcr](mdma_c5tcr) module"]
pub type MDMA_C5TCR = crate::Reg<u32, _MDMA_C5TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5TCR;
#[doc = "`read()` method returns [mdma_c5tcr::R](mdma_c5tcr::R) reader structure"]
impl crate::Readable for MDMA_C5TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5tcr::W](mdma_c5tcr::W) writer structure"]
impl crate::Writable for MDMA_C5TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c5tcr;
#[doc = "MDMA Channel 5 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5bndtr](mdma_c5bndtr) module"]
pub type MDMA_C5BNDTR = crate::Reg<u32, _MDMA_C5BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5BNDTR;
#[doc = "`read()` method returns [mdma_c5bndtr::R](mdma_c5bndtr::R) reader structure"]
impl crate::Readable for MDMA_C5BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5bndtr::W](mdma_c5bndtr::W) writer structure"]
impl crate::Writable for MDMA_C5BNDTR {}
#[doc = "MDMA Channel 5 block number of data register"]
pub mod mdma_c5bndtr;
#[doc = "MDMA channel 5 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5sar](mdma_c5sar) module"]
pub type MDMA_C5SAR = crate::Reg<u32, _MDMA_C5SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5SAR;
#[doc = "`read()` method returns [mdma_c5sar::R](mdma_c5sar::R) reader structure"]
impl crate::Readable for MDMA_C5SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5sar::W](mdma_c5sar::W) writer structure"]
impl crate::Writable for MDMA_C5SAR {}
#[doc = "MDMA channel 5 source address register"]
pub mod mdma_c5sar;
#[doc = "MDMA channel 5 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5dar](mdma_c5dar) module"]
pub type MDMA_C5DAR = crate::Reg<u32, _MDMA_C5DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5DAR;
#[doc = "`read()` method returns [mdma_c5dar::R](mdma_c5dar::R) reader structure"]
impl crate::Readable for MDMA_C5DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5dar::W](mdma_c5dar::W) writer structure"]
impl crate::Writable for MDMA_C5DAR {}
#[doc = "MDMA channel 5 destination address register"]
pub mod mdma_c5dar;
#[doc = "MDMA channel 5 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5brur](mdma_c5brur) module"]
pub type MDMA_C5BRUR = crate::Reg<u32, _MDMA_C5BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5BRUR;
#[doc = "`read()` method returns [mdma_c5brur::R](mdma_c5brur::R) reader structure"]
impl crate::Readable for MDMA_C5BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5brur::W](mdma_c5brur::W) writer structure"]
impl crate::Writable for MDMA_C5BRUR {}
#[doc = "MDMA channel 5 Block Repeat address Update register"]
pub mod mdma_c5brur;
#[doc = "MDMA channel 5 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5lar](mdma_c5lar) module"]
pub type MDMA_C5LAR = crate::Reg<u32, _MDMA_C5LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5LAR;
#[doc = "`read()` method returns [mdma_c5lar::R](mdma_c5lar::R) reader structure"]
impl crate::Readable for MDMA_C5LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5lar::W](mdma_c5lar::W) writer structure"]
impl crate::Writable for MDMA_C5LAR {}
#[doc = "MDMA channel 5 Link Address register"]
pub mod mdma_c5lar;
#[doc = "MDMA channel 5 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5tbr](mdma_c5tbr) module"]
pub type MDMA_C5TBR = crate::Reg<u32, _MDMA_C5TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5TBR;
#[doc = "`read()` method returns [mdma_c5tbr::R](mdma_c5tbr::R) reader structure"]
impl crate::Readable for MDMA_C5TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5tbr::W](mdma_c5tbr::W) writer structure"]
impl crate::Writable for MDMA_C5TBR {}
#[doc = "MDMA channel 5 Trigger and Bus selection Register"]
pub mod mdma_c5tbr;
#[doc = "MDMA channel 5 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5mar](mdma_c5mar) module"]
pub type MDMA_C5MAR = crate::Reg<u32, _MDMA_C5MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5MAR;
#[doc = "`read()` method returns [mdma_c5mar::R](mdma_c5mar::R) reader structure"]
impl crate::Readable for MDMA_C5MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5mar::W](mdma_c5mar::W) writer structure"]
impl crate::Writable for MDMA_C5MAR {}
#[doc = "MDMA channel 5 Mask address register"]
pub mod mdma_c5mar;
#[doc = "MDMA channel 5 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c5mdr](mdma_c5mdr) module"]
pub type MDMA_C5MDR = crate::Reg<u32, _MDMA_C5MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C5MDR;
#[doc = "`read()` method returns [mdma_c5mdr::R](mdma_c5mdr::R) reader structure"]
impl crate::Readable for MDMA_C5MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c5mdr::W](mdma_c5mdr::W) writer structure"]
impl crate::Writable for MDMA_C5MDR {}
#[doc = "MDMA channel 5 Mask Data register"]
pub mod mdma_c5mdr;
#[doc = "MDMA channel 6 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6isr](mdma_c6isr) module"]
pub type MDMA_C6ISR = crate::Reg<u32, _MDMA_C6ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6ISR;
#[doc = "`read()` method returns [mdma_c6isr::R](mdma_c6isr::R) reader structure"]
impl crate::Readable for MDMA_C6ISR {}
#[doc = "MDMA channel 6 interrupt/status register"]
pub mod mdma_c6isr;
#[doc = "MDMA channel 6 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6ifcr](mdma_c6ifcr) module"]
pub type MDMA_C6IFCR = crate::Reg<u32, _MDMA_C6IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6IFCR;
#[doc = "`read()` method returns [mdma_c6ifcr::R](mdma_c6ifcr::R) reader structure"]
impl crate::Readable for MDMA_C6IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6ifcr::W](mdma_c6ifcr::W) writer structure"]
impl crate::Writable for MDMA_C6IFCR {}
#[doc = "MDMA channel 6 interrupt flag clear register"]
pub mod mdma_c6ifcr;
#[doc = "MDMA Channel 6 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6esr](mdma_c6esr) module"]
pub type MDMA_C6ESR = crate::Reg<u32, _MDMA_C6ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6ESR;
#[doc = "`read()` method returns [mdma_c6esr::R](mdma_c6esr::R) reader structure"]
impl crate::Readable for MDMA_C6ESR {}
#[doc = "MDMA Channel 6 error status register"]
pub mod mdma_c6esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6cr](mdma_c6cr) module"]
pub type MDMA_C6CR = crate::Reg<u32, _MDMA_C6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6CR;
#[doc = "`read()` method returns [mdma_c6cr::R](mdma_c6cr::R) reader structure"]
impl crate::Readable for MDMA_C6CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6cr::W](mdma_c6cr::W) writer structure"]
impl crate::Writable for MDMA_C6CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c6cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6tcr](mdma_c6tcr) module"]
pub type MDMA_C6TCR = crate::Reg<u32, _MDMA_C6TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6TCR;
#[doc = "`read()` method returns [mdma_c6tcr::R](mdma_c6tcr::R) reader structure"]
impl crate::Readable for MDMA_C6TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6tcr::W](mdma_c6tcr::W) writer structure"]
impl crate::Writable for MDMA_C6TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c6tcr;
#[doc = "MDMA Channel 6 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6bndtr](mdma_c6bndtr) module"]
pub type MDMA_C6BNDTR = crate::Reg<u32, _MDMA_C6BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6BNDTR;
#[doc = "`read()` method returns [mdma_c6bndtr::R](mdma_c6bndtr::R) reader structure"]
impl crate::Readable for MDMA_C6BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6bndtr::W](mdma_c6bndtr::W) writer structure"]
impl crate::Writable for MDMA_C6BNDTR {}
#[doc = "MDMA Channel 6 block number of data register"]
pub mod mdma_c6bndtr;
#[doc = "MDMA channel 6 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6sar](mdma_c6sar) module"]
pub type MDMA_C6SAR = crate::Reg<u32, _MDMA_C6SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6SAR;
#[doc = "`read()` method returns [mdma_c6sar::R](mdma_c6sar::R) reader structure"]
impl crate::Readable for MDMA_C6SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6sar::W](mdma_c6sar::W) writer structure"]
impl crate::Writable for MDMA_C6SAR {}
#[doc = "MDMA channel 6 source address register"]
pub mod mdma_c6sar;
#[doc = "MDMA channel 6 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6dar](mdma_c6dar) module"]
pub type MDMA_C6DAR = crate::Reg<u32, _MDMA_C6DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6DAR;
#[doc = "`read()` method returns [mdma_c6dar::R](mdma_c6dar::R) reader structure"]
impl crate::Readable for MDMA_C6DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6dar::W](mdma_c6dar::W) writer structure"]
impl crate::Writable for MDMA_C6DAR {}
#[doc = "MDMA channel 6 destination address register"]
pub mod mdma_c6dar;
#[doc = "MDMA channel 6 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6brur](mdma_c6brur) module"]
pub type MDMA_C6BRUR = crate::Reg<u32, _MDMA_C6BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6BRUR;
#[doc = "`read()` method returns [mdma_c6brur::R](mdma_c6brur::R) reader structure"]
impl crate::Readable for MDMA_C6BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6brur::W](mdma_c6brur::W) writer structure"]
impl crate::Writable for MDMA_C6BRUR {}
#[doc = "MDMA channel 6 Block Repeat address Update register"]
pub mod mdma_c6brur;
#[doc = "MDMA channel 6 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6lar](mdma_c6lar) module"]
pub type MDMA_C6LAR = crate::Reg<u32, _MDMA_C6LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6LAR;
#[doc = "`read()` method returns [mdma_c6lar::R](mdma_c6lar::R) reader structure"]
impl crate::Readable for MDMA_C6LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6lar::W](mdma_c6lar::W) writer structure"]
impl crate::Writable for MDMA_C6LAR {}
#[doc = "MDMA channel 6 Link Address register"]
pub mod mdma_c6lar;
#[doc = "MDMA channel 6 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6tbr](mdma_c6tbr) module"]
pub type MDMA_C6TBR = crate::Reg<u32, _MDMA_C6TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6TBR;
#[doc = "`read()` method returns [mdma_c6tbr::R](mdma_c6tbr::R) reader structure"]
impl crate::Readable for MDMA_C6TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6tbr::W](mdma_c6tbr::W) writer structure"]
impl crate::Writable for MDMA_C6TBR {}
#[doc = "MDMA channel 6 Trigger and Bus selection Register"]
pub mod mdma_c6tbr;
#[doc = "MDMA channel 6 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6mar](mdma_c6mar) module"]
pub type MDMA_C6MAR = crate::Reg<u32, _MDMA_C6MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6MAR;
#[doc = "`read()` method returns [mdma_c6mar::R](mdma_c6mar::R) reader structure"]
impl crate::Readable for MDMA_C6MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6mar::W](mdma_c6mar::W) writer structure"]
impl crate::Writable for MDMA_C6MAR {}
#[doc = "MDMA channel 6 Mask address register"]
pub mod mdma_c6mar;
#[doc = "MDMA channel 6 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6mdr](mdma_c6mdr) module"]
pub type MDMA_C6MDR = crate::Reg<u32, _MDMA_C6MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C6MDR;
#[doc = "`read()` method returns [mdma_c6mdr::R](mdma_c6mdr::R) reader structure"]
impl crate::Readable for MDMA_C6MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c6mdr::W](mdma_c6mdr::W) writer structure"]
impl crate::Writable for MDMA_C6MDR {}
#[doc = "MDMA channel 6 Mask Data register"]
pub mod mdma_c6mdr;
#[doc = "MDMA channel 7 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7isr](mdma_c7isr) module"]
pub type MDMA_C7ISR = crate::Reg<u32, _MDMA_C7ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7ISR;
#[doc = "`read()` method returns [mdma_c7isr::R](mdma_c7isr::R) reader structure"]
impl crate::Readable for MDMA_C7ISR {}
#[doc = "MDMA channel 7 interrupt/status register"]
pub mod mdma_c7isr;
#[doc = "MDMA channel 7 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7ifcr](mdma_c7ifcr) module"]
pub type MDMA_C7IFCR = crate::Reg<u32, _MDMA_C7IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7IFCR;
#[doc = "`read()` method returns [mdma_c7ifcr::R](mdma_c7ifcr::R) reader structure"]
impl crate::Readable for MDMA_C7IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7ifcr::W](mdma_c7ifcr::W) writer structure"]
impl crate::Writable for MDMA_C7IFCR {}
#[doc = "MDMA channel 7 interrupt flag clear register"]
pub mod mdma_c7ifcr;
#[doc = "MDMA Channel 7 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7esr](mdma_c7esr) module"]
pub type MDMA_C7ESR = crate::Reg<u32, _MDMA_C7ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7ESR;
#[doc = "`read()` method returns [mdma_c7esr::R](mdma_c7esr::R) reader structure"]
impl crate::Readable for MDMA_C7ESR {}
#[doc = "MDMA Channel 7 error status register"]
pub mod mdma_c7esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7cr](mdma_c7cr) module"]
pub type MDMA_C7CR = crate::Reg<u32, _MDMA_C7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7CR;
#[doc = "`read()` method returns [mdma_c7cr::R](mdma_c7cr::R) reader structure"]
impl crate::Readable for MDMA_C7CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7cr::W](mdma_c7cr::W) writer structure"]
impl crate::Writable for MDMA_C7CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c7cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7tcr](mdma_c7tcr) module"]
pub type MDMA_C7TCR = crate::Reg<u32, _MDMA_C7TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7TCR;
#[doc = "`read()` method returns [mdma_c7tcr::R](mdma_c7tcr::R) reader structure"]
impl crate::Readable for MDMA_C7TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7tcr::W](mdma_c7tcr::W) writer structure"]
impl crate::Writable for MDMA_C7TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c7tcr;
#[doc = "MDMA Channel 7 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7bndtr](mdma_c7bndtr) module"]
pub type MDMA_C7BNDTR = crate::Reg<u32, _MDMA_C7BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7BNDTR;
#[doc = "`read()` method returns [mdma_c7bndtr::R](mdma_c7bndtr::R) reader structure"]
impl crate::Readable for MDMA_C7BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7bndtr::W](mdma_c7bndtr::W) writer structure"]
impl crate::Writable for MDMA_C7BNDTR {}
#[doc = "MDMA Channel 7 block number of data register"]
pub mod mdma_c7bndtr;
#[doc = "MDMA channel 7 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7sar](mdma_c7sar) module"]
pub type MDMA_C7SAR = crate::Reg<u32, _MDMA_C7SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7SAR;
#[doc = "`read()` method returns [mdma_c7sar::R](mdma_c7sar::R) reader structure"]
impl crate::Readable for MDMA_C7SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7sar::W](mdma_c7sar::W) writer structure"]
impl crate::Writable for MDMA_C7SAR {}
#[doc = "MDMA channel 7 source address register"]
pub mod mdma_c7sar;
#[doc = "MDMA channel 7 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7dar](mdma_c7dar) module"]
pub type MDMA_C7DAR = crate::Reg<u32, _MDMA_C7DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7DAR;
#[doc = "`read()` method returns [mdma_c7dar::R](mdma_c7dar::R) reader structure"]
impl crate::Readable for MDMA_C7DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7dar::W](mdma_c7dar::W) writer structure"]
impl crate::Writable for MDMA_C7DAR {}
#[doc = "MDMA channel 7 destination address register"]
pub mod mdma_c7dar;
#[doc = "MDMA channel 7 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7brur](mdma_c7brur) module"]
pub type MDMA_C7BRUR = crate::Reg<u32, _MDMA_C7BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7BRUR;
#[doc = "`read()` method returns [mdma_c7brur::R](mdma_c7brur::R) reader structure"]
impl crate::Readable for MDMA_C7BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7brur::W](mdma_c7brur::W) writer structure"]
impl crate::Writable for MDMA_C7BRUR {}
#[doc = "MDMA channel 7 Block Repeat address Update register"]
pub mod mdma_c7brur;
#[doc = "MDMA channel 7 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7lar](mdma_c7lar) module"]
pub type MDMA_C7LAR = crate::Reg<u32, _MDMA_C7LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7LAR;
#[doc = "`read()` method returns [mdma_c7lar::R](mdma_c7lar::R) reader structure"]
impl crate::Readable for MDMA_C7LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7lar::W](mdma_c7lar::W) writer structure"]
impl crate::Writable for MDMA_C7LAR {}
#[doc = "MDMA channel 7 Link Address register"]
pub mod mdma_c7lar;
#[doc = "MDMA channel 7 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7tbr](mdma_c7tbr) module"]
pub type MDMA_C7TBR = crate::Reg<u32, _MDMA_C7TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7TBR;
#[doc = "`read()` method returns [mdma_c7tbr::R](mdma_c7tbr::R) reader structure"]
impl crate::Readable for MDMA_C7TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7tbr::W](mdma_c7tbr::W) writer structure"]
impl crate::Writable for MDMA_C7TBR {}
#[doc = "MDMA channel 7 Trigger and Bus selection Register"]
pub mod mdma_c7tbr;
#[doc = "MDMA channel 7 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7mar](mdma_c7mar) module"]
pub type MDMA_C7MAR = crate::Reg<u32, _MDMA_C7MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7MAR;
#[doc = "`read()` method returns [mdma_c7mar::R](mdma_c7mar::R) reader structure"]
impl crate::Readable for MDMA_C7MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7mar::W](mdma_c7mar::W) writer structure"]
impl crate::Writable for MDMA_C7MAR {}
#[doc = "MDMA channel 7 Mask address register"]
pub mod mdma_c7mar;
#[doc = "MDMA channel 7 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c7mdr](mdma_c7mdr) module"]
pub type MDMA_C7MDR = crate::Reg<u32, _MDMA_C7MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C7MDR;
#[doc = "`read()` method returns [mdma_c7mdr::R](mdma_c7mdr::R) reader structure"]
impl crate::Readable for MDMA_C7MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c7mdr::W](mdma_c7mdr::W) writer structure"]
impl crate::Writable for MDMA_C7MDR {}
#[doc = "MDMA channel 7 Mask Data register"]
pub mod mdma_c7mdr;
#[doc = "MDMA channel 8 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8isr](mdma_c8isr) module"]
pub type MDMA_C8ISR = crate::Reg<u32, _MDMA_C8ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8ISR;
#[doc = "`read()` method returns [mdma_c8isr::R](mdma_c8isr::R) reader structure"]
impl crate::Readable for MDMA_C8ISR {}
#[doc = "MDMA channel 8 interrupt/status register"]
pub mod mdma_c8isr;
#[doc = "MDMA channel 8 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8ifcr](mdma_c8ifcr) module"]
pub type MDMA_C8IFCR = crate::Reg<u32, _MDMA_C8IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8IFCR;
#[doc = "`read()` method returns [mdma_c8ifcr::R](mdma_c8ifcr::R) reader structure"]
impl crate::Readable for MDMA_C8IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8ifcr::W](mdma_c8ifcr::W) writer structure"]
impl crate::Writable for MDMA_C8IFCR {}
#[doc = "MDMA channel 8 interrupt flag clear register"]
pub mod mdma_c8ifcr;
#[doc = "MDMA Channel 8 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8esr](mdma_c8esr) module"]
pub type MDMA_C8ESR = crate::Reg<u32, _MDMA_C8ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8ESR;
#[doc = "`read()` method returns [mdma_c8esr::R](mdma_c8esr::R) reader structure"]
impl crate::Readable for MDMA_C8ESR {}
#[doc = "MDMA Channel 8 error status register"]
pub mod mdma_c8esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8cr](mdma_c8cr) module"]
pub type MDMA_C8CR = crate::Reg<u32, _MDMA_C8CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8CR;
#[doc = "`read()` method returns [mdma_c8cr::R](mdma_c8cr::R) reader structure"]
impl crate::Readable for MDMA_C8CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8cr::W](mdma_c8cr::W) writer structure"]
impl crate::Writable for MDMA_C8CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c8cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8tcr](mdma_c8tcr) module"]
pub type MDMA_C8TCR = crate::Reg<u32, _MDMA_C8TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8TCR;
#[doc = "`read()` method returns [mdma_c8tcr::R](mdma_c8tcr::R) reader structure"]
impl crate::Readable for MDMA_C8TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8tcr::W](mdma_c8tcr::W) writer structure"]
impl crate::Writable for MDMA_C8TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c8tcr;
#[doc = "MDMA Channel 8 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8bndtr](mdma_c8bndtr) module"]
pub type MDMA_C8BNDTR = crate::Reg<u32, _MDMA_C8BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8BNDTR;
#[doc = "`read()` method returns [mdma_c8bndtr::R](mdma_c8bndtr::R) reader structure"]
impl crate::Readable for MDMA_C8BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8bndtr::W](mdma_c8bndtr::W) writer structure"]
impl crate::Writable for MDMA_C8BNDTR {}
#[doc = "MDMA Channel 8 block number of data register"]
pub mod mdma_c8bndtr;
#[doc = "MDMA channel 8 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8sar](mdma_c8sar) module"]
pub type MDMA_C8SAR = crate::Reg<u32, _MDMA_C8SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8SAR;
#[doc = "`read()` method returns [mdma_c8sar::R](mdma_c8sar::R) reader structure"]
impl crate::Readable for MDMA_C8SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8sar::W](mdma_c8sar::W) writer structure"]
impl crate::Writable for MDMA_C8SAR {}
#[doc = "MDMA channel 8 source address register"]
pub mod mdma_c8sar;
#[doc = "MDMA channel 8 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8dar](mdma_c8dar) module"]
pub type MDMA_C8DAR = crate::Reg<u32, _MDMA_C8DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8DAR;
#[doc = "`read()` method returns [mdma_c8dar::R](mdma_c8dar::R) reader structure"]
impl crate::Readable for MDMA_C8DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8dar::W](mdma_c8dar::W) writer structure"]
impl crate::Writable for MDMA_C8DAR {}
#[doc = "MDMA channel 8 destination address register"]
pub mod mdma_c8dar;
#[doc = "MDMA channel 8 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8brur](mdma_c8brur) module"]
pub type MDMA_C8BRUR = crate::Reg<u32, _MDMA_C8BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8BRUR;
#[doc = "`read()` method returns [mdma_c8brur::R](mdma_c8brur::R) reader structure"]
impl crate::Readable for MDMA_C8BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8brur::W](mdma_c8brur::W) writer structure"]
impl crate::Writable for MDMA_C8BRUR {}
#[doc = "MDMA channel 8 Block Repeat address Update register"]
pub mod mdma_c8brur;
#[doc = "MDMA channel 8 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8lar](mdma_c8lar) module"]
pub type MDMA_C8LAR = crate::Reg<u32, _MDMA_C8LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8LAR;
#[doc = "`read()` method returns [mdma_c8lar::R](mdma_c8lar::R) reader structure"]
impl crate::Readable for MDMA_C8LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8lar::W](mdma_c8lar::W) writer structure"]
impl crate::Writable for MDMA_C8LAR {}
#[doc = "MDMA channel 8 Link Address register"]
pub mod mdma_c8lar;
#[doc = "MDMA channel 8 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8tbr](mdma_c8tbr) module"]
pub type MDMA_C8TBR = crate::Reg<u32, _MDMA_C8TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8TBR;
#[doc = "`read()` method returns [mdma_c8tbr::R](mdma_c8tbr::R) reader structure"]
impl crate::Readable for MDMA_C8TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8tbr::W](mdma_c8tbr::W) writer structure"]
impl crate::Writable for MDMA_C8TBR {}
#[doc = "MDMA channel 8 Trigger and Bus selection Register"]
pub mod mdma_c8tbr;
#[doc = "MDMA channel 8 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8mar](mdma_c8mar) module"]
pub type MDMA_C8MAR = crate::Reg<u32, _MDMA_C8MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8MAR;
#[doc = "`read()` method returns [mdma_c8mar::R](mdma_c8mar::R) reader structure"]
impl crate::Readable for MDMA_C8MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8mar::W](mdma_c8mar::W) writer structure"]
impl crate::Writable for MDMA_C8MAR {}
#[doc = "MDMA channel 8 Mask address register"]
pub mod mdma_c8mar;
#[doc = "MDMA channel 8 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c8mdr](mdma_c8mdr) module"]
pub type MDMA_C8MDR = crate::Reg<u32, _MDMA_C8MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C8MDR;
#[doc = "`read()` method returns [mdma_c8mdr::R](mdma_c8mdr::R) reader structure"]
impl crate::Readable for MDMA_C8MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c8mdr::W](mdma_c8mdr::W) writer structure"]
impl crate::Writable for MDMA_C8MDR {}
#[doc = "MDMA channel 8 Mask Data register"]
pub mod mdma_c8mdr;
#[doc = "MDMA channel 9 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9isr](mdma_c9isr) module"]
pub type MDMA_C9ISR = crate::Reg<u32, _MDMA_C9ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9ISR;
#[doc = "`read()` method returns [mdma_c9isr::R](mdma_c9isr::R) reader structure"]
impl crate::Readable for MDMA_C9ISR {}
#[doc = "MDMA channel 9 interrupt/status register"]
pub mod mdma_c9isr;
#[doc = "MDMA channel 9 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9ifcr](mdma_c9ifcr) module"]
pub type MDMA_C9IFCR = crate::Reg<u32, _MDMA_C9IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9IFCR;
#[doc = "`read()` method returns [mdma_c9ifcr::R](mdma_c9ifcr::R) reader structure"]
impl crate::Readable for MDMA_C9IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9ifcr::W](mdma_c9ifcr::W) writer structure"]
impl crate::Writable for MDMA_C9IFCR {}
#[doc = "MDMA channel 9 interrupt flag clear register"]
pub mod mdma_c9ifcr;
#[doc = "MDMA Channel 9 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9esr](mdma_c9esr) module"]
pub type MDMA_C9ESR = crate::Reg<u32, _MDMA_C9ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9ESR;
#[doc = "`read()` method returns [mdma_c9esr::R](mdma_c9esr::R) reader structure"]
impl crate::Readable for MDMA_C9ESR {}
#[doc = "MDMA Channel 9 error status register"]
pub mod mdma_c9esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9cr](mdma_c9cr) module"]
pub type MDMA_C9CR = crate::Reg<u32, _MDMA_C9CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9CR;
#[doc = "`read()` method returns [mdma_c9cr::R](mdma_c9cr::R) reader structure"]
impl crate::Readable for MDMA_C9CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9cr::W](mdma_c9cr::W) writer structure"]
impl crate::Writable for MDMA_C9CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c9cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9tcr](mdma_c9tcr) module"]
pub type MDMA_C9TCR = crate::Reg<u32, _MDMA_C9TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9TCR;
#[doc = "`read()` method returns [mdma_c9tcr::R](mdma_c9tcr::R) reader structure"]
impl crate::Readable for MDMA_C9TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9tcr::W](mdma_c9tcr::W) writer structure"]
impl crate::Writable for MDMA_C9TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c9tcr;
#[doc = "MDMA Channel 9 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9bndtr](mdma_c9bndtr) module"]
pub type MDMA_C9BNDTR = crate::Reg<u32, _MDMA_C9BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9BNDTR;
#[doc = "`read()` method returns [mdma_c9bndtr::R](mdma_c9bndtr::R) reader structure"]
impl crate::Readable for MDMA_C9BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9bndtr::W](mdma_c9bndtr::W) writer structure"]
impl crate::Writable for MDMA_C9BNDTR {}
#[doc = "MDMA Channel 9 block number of data register"]
pub mod mdma_c9bndtr;
#[doc = "MDMA channel 9 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9sar](mdma_c9sar) module"]
pub type MDMA_C9SAR = crate::Reg<u32, _MDMA_C9SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9SAR;
#[doc = "`read()` method returns [mdma_c9sar::R](mdma_c9sar::R) reader structure"]
impl crate::Readable for MDMA_C9SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9sar::W](mdma_c9sar::W) writer structure"]
impl crate::Writable for MDMA_C9SAR {}
#[doc = "MDMA channel 9 source address register"]
pub mod mdma_c9sar;
#[doc = "MDMA channel 9 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9dar](mdma_c9dar) module"]
pub type MDMA_C9DAR = crate::Reg<u32, _MDMA_C9DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9DAR;
#[doc = "`read()` method returns [mdma_c9dar::R](mdma_c9dar::R) reader structure"]
impl crate::Readable for MDMA_C9DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9dar::W](mdma_c9dar::W) writer structure"]
impl crate::Writable for MDMA_C9DAR {}
#[doc = "MDMA channel 9 destination address register"]
pub mod mdma_c9dar;
#[doc = "MDMA channel 9 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9brur](mdma_c9brur) module"]
pub type MDMA_C9BRUR = crate::Reg<u32, _MDMA_C9BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9BRUR;
#[doc = "`read()` method returns [mdma_c9brur::R](mdma_c9brur::R) reader structure"]
impl crate::Readable for MDMA_C9BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9brur::W](mdma_c9brur::W) writer structure"]
impl crate::Writable for MDMA_C9BRUR {}
#[doc = "MDMA channel 9 Block Repeat address Update register"]
pub mod mdma_c9brur;
#[doc = "MDMA channel 9 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9lar](mdma_c9lar) module"]
pub type MDMA_C9LAR = crate::Reg<u32, _MDMA_C9LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9LAR;
#[doc = "`read()` method returns [mdma_c9lar::R](mdma_c9lar::R) reader structure"]
impl crate::Readable for MDMA_C9LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9lar::W](mdma_c9lar::W) writer structure"]
impl crate::Writable for MDMA_C9LAR {}
#[doc = "MDMA channel 9 Link Address register"]
pub mod mdma_c9lar;
#[doc = "MDMA channel 9 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9tbr](mdma_c9tbr) module"]
pub type MDMA_C9TBR = crate::Reg<u32, _MDMA_C9TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9TBR;
#[doc = "`read()` method returns [mdma_c9tbr::R](mdma_c9tbr::R) reader structure"]
impl crate::Readable for MDMA_C9TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9tbr::W](mdma_c9tbr::W) writer structure"]
impl crate::Writable for MDMA_C9TBR {}
#[doc = "MDMA channel 9 Trigger and Bus selection Register"]
pub mod mdma_c9tbr;
#[doc = "MDMA channel 9 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9mar](mdma_c9mar) module"]
pub type MDMA_C9MAR = crate::Reg<u32, _MDMA_C9MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9MAR;
#[doc = "`read()` method returns [mdma_c9mar::R](mdma_c9mar::R) reader structure"]
impl crate::Readable for MDMA_C9MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9mar::W](mdma_c9mar::W) writer structure"]
impl crate::Writable for MDMA_C9MAR {}
#[doc = "MDMA channel 9 Mask address register"]
pub mod mdma_c9mar;
#[doc = "MDMA channel 9 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9mdr](mdma_c9mdr) module"]
pub type MDMA_C9MDR = crate::Reg<u32, _MDMA_C9MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C9MDR;
#[doc = "`read()` method returns [mdma_c9mdr::R](mdma_c9mdr::R) reader structure"]
impl crate::Readable for MDMA_C9MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c9mdr::W](mdma_c9mdr::W) writer structure"]
impl crate::Writable for MDMA_C9MDR {}
#[doc = "MDMA channel 9 Mask Data register"]
pub mod mdma_c9mdr;
#[doc = "MDMA channel 10 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10isr](mdma_c10isr) module"]
pub type MDMA_C10ISR = crate::Reg<u32, _MDMA_C10ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10ISR;
#[doc = "`read()` method returns [mdma_c10isr::R](mdma_c10isr::R) reader structure"]
impl crate::Readable for MDMA_C10ISR {}
#[doc = "MDMA channel 10 interrupt/status register"]
pub mod mdma_c10isr;
#[doc = "MDMA channel 10 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10ifcr](mdma_c10ifcr) module"]
pub type MDMA_C10IFCR = crate::Reg<u32, _MDMA_C10IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10IFCR;
#[doc = "`read()` method returns [mdma_c10ifcr::R](mdma_c10ifcr::R) reader structure"]
impl crate::Readable for MDMA_C10IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10ifcr::W](mdma_c10ifcr::W) writer structure"]
impl crate::Writable for MDMA_C10IFCR {}
#[doc = "MDMA channel 10 interrupt flag clear register"]
pub mod mdma_c10ifcr;
#[doc = "MDMA Channel 10 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10esr](mdma_c10esr) module"]
pub type MDMA_C10ESR = crate::Reg<u32, _MDMA_C10ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10ESR;
#[doc = "`read()` method returns [mdma_c10esr::R](mdma_c10esr::R) reader structure"]
impl crate::Readable for MDMA_C10ESR {}
#[doc = "MDMA Channel 10 error status register"]
pub mod mdma_c10esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10cr](mdma_c10cr) module"]
pub type MDMA_C10CR = crate::Reg<u32, _MDMA_C10CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10CR;
#[doc = "`read()` method returns [mdma_c10cr::R](mdma_c10cr::R) reader structure"]
impl crate::Readable for MDMA_C10CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10cr::W](mdma_c10cr::W) writer structure"]
impl crate::Writable for MDMA_C10CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c10cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10tcr](mdma_c10tcr) module"]
pub type MDMA_C10TCR = crate::Reg<u32, _MDMA_C10TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10TCR;
#[doc = "`read()` method returns [mdma_c10tcr::R](mdma_c10tcr::R) reader structure"]
impl crate::Readable for MDMA_C10TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10tcr::W](mdma_c10tcr::W) writer structure"]
impl crate::Writable for MDMA_C10TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c10tcr;
#[doc = "MDMA Channel 10 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10bndtr](mdma_c10bndtr) module"]
pub type MDMA_C10BNDTR = crate::Reg<u32, _MDMA_C10BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10BNDTR;
#[doc = "`read()` method returns [mdma_c10bndtr::R](mdma_c10bndtr::R) reader structure"]
impl crate::Readable for MDMA_C10BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10bndtr::W](mdma_c10bndtr::W) writer structure"]
impl crate::Writable for MDMA_C10BNDTR {}
#[doc = "MDMA Channel 10 block number of data register"]
pub mod mdma_c10bndtr;
#[doc = "MDMA channel 10 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10sar](mdma_c10sar) module"]
pub type MDMA_C10SAR = crate::Reg<u32, _MDMA_C10SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10SAR;
#[doc = "`read()` method returns [mdma_c10sar::R](mdma_c10sar::R) reader structure"]
impl crate::Readable for MDMA_C10SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10sar::W](mdma_c10sar::W) writer structure"]
impl crate::Writable for MDMA_C10SAR {}
#[doc = "MDMA channel 10 source address register"]
pub mod mdma_c10sar;
#[doc = "MDMA channel 10 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10dar](mdma_c10dar) module"]
pub type MDMA_C10DAR = crate::Reg<u32, _MDMA_C10DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10DAR;
#[doc = "`read()` method returns [mdma_c10dar::R](mdma_c10dar::R) reader structure"]
impl crate::Readable for MDMA_C10DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10dar::W](mdma_c10dar::W) writer structure"]
impl crate::Writable for MDMA_C10DAR {}
#[doc = "MDMA channel 10 destination address register"]
pub mod mdma_c10dar;
#[doc = "MDMA channel 10 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10brur](mdma_c10brur) module"]
pub type MDMA_C10BRUR = crate::Reg<u32, _MDMA_C10BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10BRUR;
#[doc = "`read()` method returns [mdma_c10brur::R](mdma_c10brur::R) reader structure"]
impl crate::Readable for MDMA_C10BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10brur::W](mdma_c10brur::W) writer structure"]
impl crate::Writable for MDMA_C10BRUR {}
#[doc = "MDMA channel 10 Block Repeat address Update register"]
pub mod mdma_c10brur;
#[doc = "MDMA channel 10 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10lar](mdma_c10lar) module"]
pub type MDMA_C10LAR = crate::Reg<u32, _MDMA_C10LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10LAR;
#[doc = "`read()` method returns [mdma_c10lar::R](mdma_c10lar::R) reader structure"]
impl crate::Readable for MDMA_C10LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10lar::W](mdma_c10lar::W) writer structure"]
impl crate::Writable for MDMA_C10LAR {}
#[doc = "MDMA channel 10 Link Address register"]
pub mod mdma_c10lar;
#[doc = "MDMA channel 10 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10tbr](mdma_c10tbr) module"]
pub type MDMA_C10TBR = crate::Reg<u32, _MDMA_C10TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10TBR;
#[doc = "`read()` method returns [mdma_c10tbr::R](mdma_c10tbr::R) reader structure"]
impl crate::Readable for MDMA_C10TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10tbr::W](mdma_c10tbr::W) writer structure"]
impl crate::Writable for MDMA_C10TBR {}
#[doc = "MDMA channel 10 Trigger and Bus selection Register"]
pub mod mdma_c10tbr;
#[doc = "MDMA channel 10 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10mar](mdma_c10mar) module"]
pub type MDMA_C10MAR = crate::Reg<u32, _MDMA_C10MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10MAR;
#[doc = "`read()` method returns [mdma_c10mar::R](mdma_c10mar::R) reader structure"]
impl crate::Readable for MDMA_C10MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10mar::W](mdma_c10mar::W) writer structure"]
impl crate::Writable for MDMA_C10MAR {}
#[doc = "MDMA channel 10 Mask address register"]
pub mod mdma_c10mar;
#[doc = "MDMA channel 10 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c10mdr](mdma_c10mdr) module"]
pub type MDMA_C10MDR = crate::Reg<u32, _MDMA_C10MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C10MDR;
#[doc = "`read()` method returns [mdma_c10mdr::R](mdma_c10mdr::R) reader structure"]
impl crate::Readable for MDMA_C10MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c10mdr::W](mdma_c10mdr::W) writer structure"]
impl crate::Writable for MDMA_C10MDR {}
#[doc = "MDMA channel 10 Mask Data register"]
pub mod mdma_c10mdr;
#[doc = "MDMA channel 11 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11isr](mdma_c11isr) module"]
pub type MDMA_C11ISR = crate::Reg<u32, _MDMA_C11ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11ISR;
#[doc = "`read()` method returns [mdma_c11isr::R](mdma_c11isr::R) reader structure"]
impl crate::Readable for MDMA_C11ISR {}
#[doc = "MDMA channel 11 interrupt/status register"]
pub mod mdma_c11isr;
#[doc = "MDMA channel 11 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11ifcr](mdma_c11ifcr) module"]
pub type MDMA_C11IFCR = crate::Reg<u32, _MDMA_C11IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11IFCR;
#[doc = "`read()` method returns [mdma_c11ifcr::R](mdma_c11ifcr::R) reader structure"]
impl crate::Readable for MDMA_C11IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11ifcr::W](mdma_c11ifcr::W) writer structure"]
impl crate::Writable for MDMA_C11IFCR {}
#[doc = "MDMA channel 11 interrupt flag clear register"]
pub mod mdma_c11ifcr;
#[doc = "MDMA Channel 11 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11esr](mdma_c11esr) module"]
pub type MDMA_C11ESR = crate::Reg<u32, _MDMA_C11ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11ESR;
#[doc = "`read()` method returns [mdma_c11esr::R](mdma_c11esr::R) reader structure"]
impl crate::Readable for MDMA_C11ESR {}
#[doc = "MDMA Channel 11 error status register"]
pub mod mdma_c11esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11cr](mdma_c11cr) module"]
pub type MDMA_C11CR = crate::Reg<u32, _MDMA_C11CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11CR;
#[doc = "`read()` method returns [mdma_c11cr::R](mdma_c11cr::R) reader structure"]
impl crate::Readable for MDMA_C11CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11cr::W](mdma_c11cr::W) writer structure"]
impl crate::Writable for MDMA_C11CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c11cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11tcr](mdma_c11tcr) module"]
pub type MDMA_C11TCR = crate::Reg<u32, _MDMA_C11TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11TCR;
#[doc = "`read()` method returns [mdma_c11tcr::R](mdma_c11tcr::R) reader structure"]
impl crate::Readable for MDMA_C11TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11tcr::W](mdma_c11tcr::W) writer structure"]
impl crate::Writable for MDMA_C11TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c11tcr;
#[doc = "MDMA Channel 11 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11bndtr](mdma_c11bndtr) module"]
pub type MDMA_C11BNDTR = crate::Reg<u32, _MDMA_C11BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11BNDTR;
#[doc = "`read()` method returns [mdma_c11bndtr::R](mdma_c11bndtr::R) reader structure"]
impl crate::Readable for MDMA_C11BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11bndtr::W](mdma_c11bndtr::W) writer structure"]
impl crate::Writable for MDMA_C11BNDTR {}
#[doc = "MDMA Channel 11 block number of data register"]
pub mod mdma_c11bndtr;
#[doc = "MDMA channel 11 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11sar](mdma_c11sar) module"]
pub type MDMA_C11SAR = crate::Reg<u32, _MDMA_C11SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11SAR;
#[doc = "`read()` method returns [mdma_c11sar::R](mdma_c11sar::R) reader structure"]
impl crate::Readable for MDMA_C11SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11sar::W](mdma_c11sar::W) writer structure"]
impl crate::Writable for MDMA_C11SAR {}
#[doc = "MDMA channel 11 source address register"]
pub mod mdma_c11sar;
#[doc = "MDMA channel 11 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11dar](mdma_c11dar) module"]
pub type MDMA_C11DAR = crate::Reg<u32, _MDMA_C11DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11DAR;
#[doc = "`read()` method returns [mdma_c11dar::R](mdma_c11dar::R) reader structure"]
impl crate::Readable for MDMA_C11DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11dar::W](mdma_c11dar::W) writer structure"]
impl crate::Writable for MDMA_C11DAR {}
#[doc = "MDMA channel 11 destination address register"]
pub mod mdma_c11dar;
#[doc = "MDMA channel 11 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11brur](mdma_c11brur) module"]
pub type MDMA_C11BRUR = crate::Reg<u32, _MDMA_C11BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11BRUR;
#[doc = "`read()` method returns [mdma_c11brur::R](mdma_c11brur::R) reader structure"]
impl crate::Readable for MDMA_C11BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11brur::W](mdma_c11brur::W) writer structure"]
impl crate::Writable for MDMA_C11BRUR {}
#[doc = "MDMA channel 11 Block Repeat address Update register"]
pub mod mdma_c11brur;
#[doc = "MDMA channel 11 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11lar](mdma_c11lar) module"]
pub type MDMA_C11LAR = crate::Reg<u32, _MDMA_C11LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11LAR;
#[doc = "`read()` method returns [mdma_c11lar::R](mdma_c11lar::R) reader structure"]
impl crate::Readable for MDMA_C11LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11lar::W](mdma_c11lar::W) writer structure"]
impl crate::Writable for MDMA_C11LAR {}
#[doc = "MDMA channel 11 Link Address register"]
pub mod mdma_c11lar;
#[doc = "MDMA channel 11 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11tbr](mdma_c11tbr) module"]
pub type MDMA_C11TBR = crate::Reg<u32, _MDMA_C11TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11TBR;
#[doc = "`read()` method returns [mdma_c11tbr::R](mdma_c11tbr::R) reader structure"]
impl crate::Readable for MDMA_C11TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11tbr::W](mdma_c11tbr::W) writer structure"]
impl crate::Writable for MDMA_C11TBR {}
#[doc = "MDMA channel 11 Trigger and Bus selection Register"]
pub mod mdma_c11tbr;
#[doc = "MDMA channel 11 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11mar](mdma_c11mar) module"]
pub type MDMA_C11MAR = crate::Reg<u32, _MDMA_C11MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11MAR;
#[doc = "`read()` method returns [mdma_c11mar::R](mdma_c11mar::R) reader structure"]
impl crate::Readable for MDMA_C11MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11mar::W](mdma_c11mar::W) writer structure"]
impl crate::Writable for MDMA_C11MAR {}
#[doc = "MDMA channel 11 Mask address register"]
pub mod mdma_c11mar;
#[doc = "MDMA channel 11 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c11mdr](mdma_c11mdr) module"]
pub type MDMA_C11MDR = crate::Reg<u32, _MDMA_C11MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C11MDR;
#[doc = "`read()` method returns [mdma_c11mdr::R](mdma_c11mdr::R) reader structure"]
impl crate::Readable for MDMA_C11MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c11mdr::W](mdma_c11mdr::W) writer structure"]
impl crate::Writable for MDMA_C11MDR {}
#[doc = "MDMA channel 11 Mask Data register"]
pub mod mdma_c11mdr;
#[doc = "MDMA channel 12 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12isr](mdma_c12isr) module"]
pub type MDMA_C12ISR = crate::Reg<u32, _MDMA_C12ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12ISR;
#[doc = "`read()` method returns [mdma_c12isr::R](mdma_c12isr::R) reader structure"]
impl crate::Readable for MDMA_C12ISR {}
#[doc = "MDMA channel 12 interrupt/status register"]
pub mod mdma_c12isr;
#[doc = "MDMA channel 12 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12ifcr](mdma_c12ifcr) module"]
pub type MDMA_C12IFCR = crate::Reg<u32, _MDMA_C12IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12IFCR;
#[doc = "`read()` method returns [mdma_c12ifcr::R](mdma_c12ifcr::R) reader structure"]
impl crate::Readable for MDMA_C12IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12ifcr::W](mdma_c12ifcr::W) writer structure"]
impl crate::Writable for MDMA_C12IFCR {}
#[doc = "MDMA channel 12 interrupt flag clear register"]
pub mod mdma_c12ifcr;
#[doc = "MDMA Channel 12 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12esr](mdma_c12esr) module"]
pub type MDMA_C12ESR = crate::Reg<u32, _MDMA_C12ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12ESR;
#[doc = "`read()` method returns [mdma_c12esr::R](mdma_c12esr::R) reader structure"]
impl crate::Readable for MDMA_C12ESR {}
#[doc = "MDMA Channel 12 error status register"]
pub mod mdma_c12esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12cr](mdma_c12cr) module"]
pub type MDMA_C12CR = crate::Reg<u32, _MDMA_C12CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12CR;
#[doc = "`read()` method returns [mdma_c12cr::R](mdma_c12cr::R) reader structure"]
impl crate::Readable for MDMA_C12CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12cr::W](mdma_c12cr::W) writer structure"]
impl crate::Writable for MDMA_C12CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c12cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12tcr](mdma_c12tcr) module"]
pub type MDMA_C12TCR = crate::Reg<u32, _MDMA_C12TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12TCR;
#[doc = "`read()` method returns [mdma_c12tcr::R](mdma_c12tcr::R) reader structure"]
impl crate::Readable for MDMA_C12TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12tcr::W](mdma_c12tcr::W) writer structure"]
impl crate::Writable for MDMA_C12TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c12tcr;
#[doc = "MDMA Channel 12 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12bndtr](mdma_c12bndtr) module"]
pub type MDMA_C12BNDTR = crate::Reg<u32, _MDMA_C12BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12BNDTR;
#[doc = "`read()` method returns [mdma_c12bndtr::R](mdma_c12bndtr::R) reader structure"]
impl crate::Readable for MDMA_C12BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12bndtr::W](mdma_c12bndtr::W) writer structure"]
impl crate::Writable for MDMA_C12BNDTR {}
#[doc = "MDMA Channel 12 block number of data register"]
pub mod mdma_c12bndtr;
#[doc = "MDMA channel 12 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12sar](mdma_c12sar) module"]
pub type MDMA_C12SAR = crate::Reg<u32, _MDMA_C12SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12SAR;
#[doc = "`read()` method returns [mdma_c12sar::R](mdma_c12sar::R) reader structure"]
impl crate::Readable for MDMA_C12SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12sar::W](mdma_c12sar::W) writer structure"]
impl crate::Writable for MDMA_C12SAR {}
#[doc = "MDMA channel 12 source address register"]
pub mod mdma_c12sar;
#[doc = "MDMA channel 12 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12dar](mdma_c12dar) module"]
pub type MDMA_C12DAR = crate::Reg<u32, _MDMA_C12DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12DAR;
#[doc = "`read()` method returns [mdma_c12dar::R](mdma_c12dar::R) reader structure"]
impl crate::Readable for MDMA_C12DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12dar::W](mdma_c12dar::W) writer structure"]
impl crate::Writable for MDMA_C12DAR {}
#[doc = "MDMA channel 12 destination address register"]
pub mod mdma_c12dar;
#[doc = "MDMA channel 12 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12brur](mdma_c12brur) module"]
pub type MDMA_C12BRUR = crate::Reg<u32, _MDMA_C12BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12BRUR;
#[doc = "`read()` method returns [mdma_c12brur::R](mdma_c12brur::R) reader structure"]
impl crate::Readable for MDMA_C12BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12brur::W](mdma_c12brur::W) writer structure"]
impl crate::Writable for MDMA_C12BRUR {}
#[doc = "MDMA channel 12 Block Repeat address Update register"]
pub mod mdma_c12brur;
#[doc = "MDMA channel 12 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12lar](mdma_c12lar) module"]
pub type MDMA_C12LAR = crate::Reg<u32, _MDMA_C12LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12LAR;
#[doc = "`read()` method returns [mdma_c12lar::R](mdma_c12lar::R) reader structure"]
impl crate::Readable for MDMA_C12LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12lar::W](mdma_c12lar::W) writer structure"]
impl crate::Writable for MDMA_C12LAR {}
#[doc = "MDMA channel 12 Link Address register"]
pub mod mdma_c12lar;
#[doc = "MDMA channel 12 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12tbr](mdma_c12tbr) module"]
pub type MDMA_C12TBR = crate::Reg<u32, _MDMA_C12TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12TBR;
#[doc = "`read()` method returns [mdma_c12tbr::R](mdma_c12tbr::R) reader structure"]
impl crate::Readable for MDMA_C12TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12tbr::W](mdma_c12tbr::W) writer structure"]
impl crate::Writable for MDMA_C12TBR {}
#[doc = "MDMA channel 12 Trigger and Bus selection Register"]
pub mod mdma_c12tbr;
#[doc = "MDMA channel 12 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12mar](mdma_c12mar) module"]
pub type MDMA_C12MAR = crate::Reg<u32, _MDMA_C12MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12MAR;
#[doc = "`read()` method returns [mdma_c12mar::R](mdma_c12mar::R) reader structure"]
impl crate::Readable for MDMA_C12MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12mar::W](mdma_c12mar::W) writer structure"]
impl crate::Writable for MDMA_C12MAR {}
#[doc = "MDMA channel 12 Mask address register"]
pub mod mdma_c12mar;
#[doc = "MDMA channel 12 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12mdr](mdma_c12mdr) module"]
pub type MDMA_C12MDR = crate::Reg<u32, _MDMA_C12MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C12MDR;
#[doc = "`read()` method returns [mdma_c12mdr::R](mdma_c12mdr::R) reader structure"]
impl crate::Readable for MDMA_C12MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c12mdr::W](mdma_c12mdr::W) writer structure"]
impl crate::Writable for MDMA_C12MDR {}
#[doc = "MDMA channel 12 Mask Data register"]
pub mod mdma_c12mdr;
#[doc = "MDMA channel 13 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13isr](mdma_c13isr) module"]
pub type MDMA_C13ISR = crate::Reg<u32, _MDMA_C13ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13ISR;
#[doc = "`read()` method returns [mdma_c13isr::R](mdma_c13isr::R) reader structure"]
impl crate::Readable for MDMA_C13ISR {}
#[doc = "MDMA channel 13 interrupt/status register"]
pub mod mdma_c13isr;
#[doc = "MDMA channel 13 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13ifcr](mdma_c13ifcr) module"]
pub type MDMA_C13IFCR = crate::Reg<u32, _MDMA_C13IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13IFCR;
#[doc = "`read()` method returns [mdma_c13ifcr::R](mdma_c13ifcr::R) reader structure"]
impl crate::Readable for MDMA_C13IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13ifcr::W](mdma_c13ifcr::W) writer structure"]
impl crate::Writable for MDMA_C13IFCR {}
#[doc = "MDMA channel 13 interrupt flag clear register"]
pub mod mdma_c13ifcr;
#[doc = "MDMA Channel 13 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13esr](mdma_c13esr) module"]
pub type MDMA_C13ESR = crate::Reg<u32, _MDMA_C13ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13ESR;
#[doc = "`read()` method returns [mdma_c13esr::R](mdma_c13esr::R) reader structure"]
impl crate::Readable for MDMA_C13ESR {}
#[doc = "MDMA Channel 13 error status register"]
pub mod mdma_c13esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13cr](mdma_c13cr) module"]
pub type MDMA_C13CR = crate::Reg<u32, _MDMA_C13CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13CR;
#[doc = "`read()` method returns [mdma_c13cr::R](mdma_c13cr::R) reader structure"]
impl crate::Readable for MDMA_C13CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13cr::W](mdma_c13cr::W) writer structure"]
impl crate::Writable for MDMA_C13CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c13cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13tcr](mdma_c13tcr) module"]
pub type MDMA_C13TCR = crate::Reg<u32, _MDMA_C13TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13TCR;
#[doc = "`read()` method returns [mdma_c13tcr::R](mdma_c13tcr::R) reader structure"]
impl crate::Readable for MDMA_C13TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13tcr::W](mdma_c13tcr::W) writer structure"]
impl crate::Writable for MDMA_C13TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c13tcr;
#[doc = "MDMA Channel 13 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13bndtr](mdma_c13bndtr) module"]
pub type MDMA_C13BNDTR = crate::Reg<u32, _MDMA_C13BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13BNDTR;
#[doc = "`read()` method returns [mdma_c13bndtr::R](mdma_c13bndtr::R) reader structure"]
impl crate::Readable for MDMA_C13BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13bndtr::W](mdma_c13bndtr::W) writer structure"]
impl crate::Writable for MDMA_C13BNDTR {}
#[doc = "MDMA Channel 13 block number of data register"]
pub mod mdma_c13bndtr;
#[doc = "MDMA channel 13 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13sar](mdma_c13sar) module"]
pub type MDMA_C13SAR = crate::Reg<u32, _MDMA_C13SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13SAR;
#[doc = "`read()` method returns [mdma_c13sar::R](mdma_c13sar::R) reader structure"]
impl crate::Readable for MDMA_C13SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13sar::W](mdma_c13sar::W) writer structure"]
impl crate::Writable for MDMA_C13SAR {}
#[doc = "MDMA channel 13 source address register"]
pub mod mdma_c13sar;
#[doc = "MDMA channel 13 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13dar](mdma_c13dar) module"]
pub type MDMA_C13DAR = crate::Reg<u32, _MDMA_C13DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13DAR;
#[doc = "`read()` method returns [mdma_c13dar::R](mdma_c13dar::R) reader structure"]
impl crate::Readable for MDMA_C13DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13dar::W](mdma_c13dar::W) writer structure"]
impl crate::Writable for MDMA_C13DAR {}
#[doc = "MDMA channel 13 destination address register"]
pub mod mdma_c13dar;
#[doc = "MDMA channel 13 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13brur](mdma_c13brur) module"]
pub type MDMA_C13BRUR = crate::Reg<u32, _MDMA_C13BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13BRUR;
#[doc = "`read()` method returns [mdma_c13brur::R](mdma_c13brur::R) reader structure"]
impl crate::Readable for MDMA_C13BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13brur::W](mdma_c13brur::W) writer structure"]
impl crate::Writable for MDMA_C13BRUR {}
#[doc = "MDMA channel 13 Block Repeat address Update register"]
pub mod mdma_c13brur;
#[doc = "MDMA channel 13 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13lar](mdma_c13lar) module"]
pub type MDMA_C13LAR = crate::Reg<u32, _MDMA_C13LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13LAR;
#[doc = "`read()` method returns [mdma_c13lar::R](mdma_c13lar::R) reader structure"]
impl crate::Readable for MDMA_C13LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13lar::W](mdma_c13lar::W) writer structure"]
impl crate::Writable for MDMA_C13LAR {}
#[doc = "MDMA channel 13 Link Address register"]
pub mod mdma_c13lar;
#[doc = "MDMA channel 13 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13tbr](mdma_c13tbr) module"]
pub type MDMA_C13TBR = crate::Reg<u32, _MDMA_C13TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13TBR;
#[doc = "`read()` method returns [mdma_c13tbr::R](mdma_c13tbr::R) reader structure"]
impl crate::Readable for MDMA_C13TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13tbr::W](mdma_c13tbr::W) writer structure"]
impl crate::Writable for MDMA_C13TBR {}
#[doc = "MDMA channel 13 Trigger and Bus selection Register"]
pub mod mdma_c13tbr;
#[doc = "MDMA channel 13 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13mar](mdma_c13mar) module"]
pub type MDMA_C13MAR = crate::Reg<u32, _MDMA_C13MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13MAR;
#[doc = "`read()` method returns [mdma_c13mar::R](mdma_c13mar::R) reader structure"]
impl crate::Readable for MDMA_C13MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13mar::W](mdma_c13mar::W) writer structure"]
impl crate::Writable for MDMA_C13MAR {}
#[doc = "MDMA channel 13 Mask address register"]
pub mod mdma_c13mar;
#[doc = "MDMA channel 13 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c13mdr](mdma_c13mdr) module"]
pub type MDMA_C13MDR = crate::Reg<u32, _MDMA_C13MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C13MDR;
#[doc = "`read()` method returns [mdma_c13mdr::R](mdma_c13mdr::R) reader structure"]
impl crate::Readable for MDMA_C13MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c13mdr::W](mdma_c13mdr::W) writer structure"]
impl crate::Writable for MDMA_C13MDR {}
#[doc = "MDMA channel 13 Mask Data register"]
pub mod mdma_c13mdr;
#[doc = "MDMA channel 14 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14isr](mdma_c14isr) module"]
pub type MDMA_C14ISR = crate::Reg<u32, _MDMA_C14ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14ISR;
#[doc = "`read()` method returns [mdma_c14isr::R](mdma_c14isr::R) reader structure"]
impl crate::Readable for MDMA_C14ISR {}
#[doc = "MDMA channel 14 interrupt/status register"]
pub mod mdma_c14isr;
#[doc = "MDMA channel 14 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14ifcr](mdma_c14ifcr) module"]
pub type MDMA_C14IFCR = crate::Reg<u32, _MDMA_C14IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14IFCR;
#[doc = "`read()` method returns [mdma_c14ifcr::R](mdma_c14ifcr::R) reader structure"]
impl crate::Readable for MDMA_C14IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14ifcr::W](mdma_c14ifcr::W) writer structure"]
impl crate::Writable for MDMA_C14IFCR {}
#[doc = "MDMA channel 14 interrupt flag clear register"]
pub mod mdma_c14ifcr;
#[doc = "MDMA Channel 14 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14esr](mdma_c14esr) module"]
pub type MDMA_C14ESR = crate::Reg<u32, _MDMA_C14ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14ESR;
#[doc = "`read()` method returns [mdma_c14esr::R](mdma_c14esr::R) reader structure"]
impl crate::Readable for MDMA_C14ESR {}
#[doc = "MDMA Channel 14 error status register"]
pub mod mdma_c14esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14cr](mdma_c14cr) module"]
pub type MDMA_C14CR = crate::Reg<u32, _MDMA_C14CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14CR;
#[doc = "`read()` method returns [mdma_c14cr::R](mdma_c14cr::R) reader structure"]
impl crate::Readable for MDMA_C14CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14cr::W](mdma_c14cr::W) writer structure"]
impl crate::Writable for MDMA_C14CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c14cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14tcr](mdma_c14tcr) module"]
pub type MDMA_C14TCR = crate::Reg<u32, _MDMA_C14TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14TCR;
#[doc = "`read()` method returns [mdma_c14tcr::R](mdma_c14tcr::R) reader structure"]
impl crate::Readable for MDMA_C14TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14tcr::W](mdma_c14tcr::W) writer structure"]
impl crate::Writable for MDMA_C14TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c14tcr;
#[doc = "MDMA Channel 14 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14bndtr](mdma_c14bndtr) module"]
pub type MDMA_C14BNDTR = crate::Reg<u32, _MDMA_C14BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14BNDTR;
#[doc = "`read()` method returns [mdma_c14bndtr::R](mdma_c14bndtr::R) reader structure"]
impl crate::Readable for MDMA_C14BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14bndtr::W](mdma_c14bndtr::W) writer structure"]
impl crate::Writable for MDMA_C14BNDTR {}
#[doc = "MDMA Channel 14 block number of data register"]
pub mod mdma_c14bndtr;
#[doc = "MDMA channel 14 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14sar](mdma_c14sar) module"]
pub type MDMA_C14SAR = crate::Reg<u32, _MDMA_C14SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14SAR;
#[doc = "`read()` method returns [mdma_c14sar::R](mdma_c14sar::R) reader structure"]
impl crate::Readable for MDMA_C14SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14sar::W](mdma_c14sar::W) writer structure"]
impl crate::Writable for MDMA_C14SAR {}
#[doc = "MDMA channel 14 source address register"]
pub mod mdma_c14sar;
#[doc = "MDMA channel 14 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14dar](mdma_c14dar) module"]
pub type MDMA_C14DAR = crate::Reg<u32, _MDMA_C14DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14DAR;
#[doc = "`read()` method returns [mdma_c14dar::R](mdma_c14dar::R) reader structure"]
impl crate::Readable for MDMA_C14DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14dar::W](mdma_c14dar::W) writer structure"]
impl crate::Writable for MDMA_C14DAR {}
#[doc = "MDMA channel 14 destination address register"]
pub mod mdma_c14dar;
#[doc = "MDMA channel 14 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14brur](mdma_c14brur) module"]
pub type MDMA_C14BRUR = crate::Reg<u32, _MDMA_C14BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14BRUR;
#[doc = "`read()` method returns [mdma_c14brur::R](mdma_c14brur::R) reader structure"]
impl crate::Readable for MDMA_C14BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14brur::W](mdma_c14brur::W) writer structure"]
impl crate::Writable for MDMA_C14BRUR {}
#[doc = "MDMA channel 14 Block Repeat address Update register"]
pub mod mdma_c14brur;
#[doc = "MDMA channel 14 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14lar](mdma_c14lar) module"]
pub type MDMA_C14LAR = crate::Reg<u32, _MDMA_C14LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14LAR;
#[doc = "`read()` method returns [mdma_c14lar::R](mdma_c14lar::R) reader structure"]
impl crate::Readable for MDMA_C14LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14lar::W](mdma_c14lar::W) writer structure"]
impl crate::Writable for MDMA_C14LAR {}
#[doc = "MDMA channel 14 Link Address register"]
pub mod mdma_c14lar;
#[doc = "MDMA channel 14 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14tbr](mdma_c14tbr) module"]
pub type MDMA_C14TBR = crate::Reg<u32, _MDMA_C14TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14TBR;
#[doc = "`read()` method returns [mdma_c14tbr::R](mdma_c14tbr::R) reader structure"]
impl crate::Readable for MDMA_C14TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14tbr::W](mdma_c14tbr::W) writer structure"]
impl crate::Writable for MDMA_C14TBR {}
#[doc = "MDMA channel 14 Trigger and Bus selection Register"]
pub mod mdma_c14tbr;
#[doc = "MDMA channel 14 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14mar](mdma_c14mar) module"]
pub type MDMA_C14MAR = crate::Reg<u32, _MDMA_C14MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14MAR;
#[doc = "`read()` method returns [mdma_c14mar::R](mdma_c14mar::R) reader structure"]
impl crate::Readable for MDMA_C14MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14mar::W](mdma_c14mar::W) writer structure"]
impl crate::Writable for MDMA_C14MAR {}
#[doc = "MDMA channel 14 Mask address register"]
pub mod mdma_c14mar;
#[doc = "MDMA channel 14 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c14mdr](mdma_c14mdr) module"]
pub type MDMA_C14MDR = crate::Reg<u32, _MDMA_C14MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C14MDR;
#[doc = "`read()` method returns [mdma_c14mdr::R](mdma_c14mdr::R) reader structure"]
impl crate::Readable for MDMA_C14MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c14mdr::W](mdma_c14mdr::W) writer structure"]
impl crate::Writable for MDMA_C14MDR {}
#[doc = "MDMA channel 14 Mask Data register"]
pub mod mdma_c14mdr;
#[doc = "MDMA channel 15 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15isr](mdma_c15isr) module"]
pub type MDMA_C15ISR = crate::Reg<u32, _MDMA_C15ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15ISR;
#[doc = "`read()` method returns [mdma_c15isr::R](mdma_c15isr::R) reader structure"]
impl crate::Readable for MDMA_C15ISR {}
#[doc = "MDMA channel 15 interrupt/status register"]
pub mod mdma_c15isr;
#[doc = "MDMA channel 15 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15ifcr](mdma_c15ifcr) module"]
pub type MDMA_C15IFCR = crate::Reg<u32, _MDMA_C15IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15IFCR;
#[doc = "`read()` method returns [mdma_c15ifcr::R](mdma_c15ifcr::R) reader structure"]
impl crate::Readable for MDMA_C15IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15ifcr::W](mdma_c15ifcr::W) writer structure"]
impl crate::Writable for MDMA_C15IFCR {}
#[doc = "MDMA channel 15 interrupt flag clear register"]
pub mod mdma_c15ifcr;
#[doc = "MDMA Channel 15 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15esr](mdma_c15esr) module"]
pub type MDMA_C15ESR = crate::Reg<u32, _MDMA_C15ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15ESR;
#[doc = "`read()` method returns [mdma_c15esr::R](mdma_c15esr::R) reader structure"]
impl crate::Readable for MDMA_C15ESR {}
#[doc = "MDMA Channel 15 error status register"]
pub mod mdma_c15esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15cr](mdma_c15cr) module"]
pub type MDMA_C15CR = crate::Reg<u32, _MDMA_C15CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15CR;
#[doc = "`read()` method returns [mdma_c15cr::R](mdma_c15cr::R) reader structure"]
impl crate::Readable for MDMA_C15CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15cr::W](mdma_c15cr::W) writer structure"]
impl crate::Writable for MDMA_C15CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c15cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15tcr](mdma_c15tcr) module"]
pub type MDMA_C15TCR = crate::Reg<u32, _MDMA_C15TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15TCR;
#[doc = "`read()` method returns [mdma_c15tcr::R](mdma_c15tcr::R) reader structure"]
impl crate::Readable for MDMA_C15TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15tcr::W](mdma_c15tcr::W) writer structure"]
impl crate::Writable for MDMA_C15TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c15tcr;
#[doc = "MDMA Channel 15 block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15bndtr](mdma_c15bndtr) module"]
pub type MDMA_C15BNDTR = crate::Reg<u32, _MDMA_C15BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15BNDTR;
#[doc = "`read()` method returns [mdma_c15bndtr::R](mdma_c15bndtr::R) reader structure"]
impl crate::Readable for MDMA_C15BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15bndtr::W](mdma_c15bndtr::W) writer structure"]
impl crate::Writable for MDMA_C15BNDTR {}
#[doc = "MDMA Channel 15 block number of data register"]
pub mod mdma_c15bndtr;
#[doc = "MDMA channel 15 source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15sar](mdma_c15sar) module"]
pub type MDMA_C15SAR = crate::Reg<u32, _MDMA_C15SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15SAR;
#[doc = "`read()` method returns [mdma_c15sar::R](mdma_c15sar::R) reader structure"]
impl crate::Readable for MDMA_C15SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15sar::W](mdma_c15sar::W) writer structure"]
impl crate::Writable for MDMA_C15SAR {}
#[doc = "MDMA channel 15 source address register"]
pub mod mdma_c15sar;
#[doc = "MDMA channel 15 destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15dar](mdma_c15dar) module"]
pub type MDMA_C15DAR = crate::Reg<u32, _MDMA_C15DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15DAR;
#[doc = "`read()` method returns [mdma_c15dar::R](mdma_c15dar::R) reader structure"]
impl crate::Readable for MDMA_C15DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15dar::W](mdma_c15dar::W) writer structure"]
impl crate::Writable for MDMA_C15DAR {}
#[doc = "MDMA channel 15 destination address register"]
pub mod mdma_c15dar;
#[doc = "MDMA channel 15 Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15brur](mdma_c15brur) module"]
pub type MDMA_C15BRUR = crate::Reg<u32, _MDMA_C15BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15BRUR;
#[doc = "`read()` method returns [mdma_c15brur::R](mdma_c15brur::R) reader structure"]
impl crate::Readable for MDMA_C15BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15brur::W](mdma_c15brur::W) writer structure"]
impl crate::Writable for MDMA_C15BRUR {}
#[doc = "MDMA channel 15 Block Repeat address Update register"]
pub mod mdma_c15brur;
#[doc = "MDMA channel 15 Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15lar](mdma_c15lar) module"]
pub type MDMA_C15LAR = crate::Reg<u32, _MDMA_C15LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15LAR;
#[doc = "`read()` method returns [mdma_c15lar::R](mdma_c15lar::R) reader structure"]
impl crate::Readable for MDMA_C15LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15lar::W](mdma_c15lar::W) writer structure"]
impl crate::Writable for MDMA_C15LAR {}
#[doc = "MDMA channel 15 Link Address register"]
pub mod mdma_c15lar;
#[doc = "MDMA channel 15 Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15tbr](mdma_c15tbr) module"]
pub type MDMA_C15TBR = crate::Reg<u32, _MDMA_C15TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15TBR;
#[doc = "`read()` method returns [mdma_c15tbr::R](mdma_c15tbr::R) reader structure"]
impl crate::Readable for MDMA_C15TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15tbr::W](mdma_c15tbr::W) writer structure"]
impl crate::Writable for MDMA_C15TBR {}
#[doc = "MDMA channel 15 Trigger and Bus selection Register"]
pub mod mdma_c15tbr;
#[doc = "MDMA channel 15 Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15mar](mdma_c15mar) module"]
pub type MDMA_C15MAR = crate::Reg<u32, _MDMA_C15MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15MAR;
#[doc = "`read()` method returns [mdma_c15mar::R](mdma_c15mar::R) reader structure"]
impl crate::Readable for MDMA_C15MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15mar::W](mdma_c15mar::W) writer structure"]
impl crate::Writable for MDMA_C15MAR {}
#[doc = "MDMA channel 15 Mask address register"]
pub mod mdma_c15mar;
#[doc = "MDMA channel 15 Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c15mdr](mdma_c15mdr) module"]
pub type MDMA_C15MDR = crate::Reg<u32, _MDMA_C15MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C15MDR;
#[doc = "`read()` method returns [mdma_c15mdr::R](mdma_c15mdr::R) reader structure"]
impl crate::Readable for MDMA_C15MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c15mdr::W](mdma_c15mdr::W) writer structure"]
impl crate::Writable for MDMA_C15MDR {}
#[doc = "MDMA channel 15 Mask Data register"]
pub mod mdma_c15mdr;
#[doc = "MDMA channel 16 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16isr](mdma_c16isr) module"]
pub type MDMA_C16ISR = crate::Reg<u32, _MDMA_C16ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16ISR;
#[doc = "`read()` method returns [mdma_c16isr::R](mdma_c16isr::R) reader structure"]
impl crate::Readable for MDMA_C16ISR {}
#[doc = "MDMA channel 16 interrupt/status register"]
pub mod mdma_c16isr;
#[doc = "MDMA channel 16 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16ifcr](mdma_c16ifcr) module"]
pub type MDMA_C16IFCR = crate::Reg<u32, _MDMA_C16IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16IFCR;
#[doc = "`read()` method returns [mdma_c16ifcr::R](mdma_c16ifcr::R) reader structure"]
impl crate::Readable for MDMA_C16IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16ifcr::W](mdma_c16ifcr::W) writer structure"]
impl crate::Writable for MDMA_C16IFCR {}
#[doc = "MDMA channel 16 interrupt flag clear register"]
pub mod mdma_c16ifcr;
#[doc = "MDMA Channel 16 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16esr](mdma_c16esr) module"]
pub type MDMA_C16ESR = crate::Reg<u32, _MDMA_C16ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16ESR;
#[doc = "`read()` method returns [mdma_c16esr::R](mdma_c16esr::R) reader structure"]
impl crate::Readable for MDMA_C16ESR {}
#[doc = "MDMA Channel 16 error status register"]
pub mod mdma_c16esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16cr](mdma_c16cr) module"]
pub type MDMA_C16CR = crate::Reg<u32, _MDMA_C16CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16CR;
#[doc = "`read()` method returns [mdma_c16cr::R](mdma_c16cr::R) reader structure"]
impl crate::Readable for MDMA_C16CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16cr::W](mdma_c16cr::W) writer structure"]
impl crate::Writable for MDMA_C16CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c16cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16tcr](mdma_c16tcr) module"]
pub type MDMA_C16TCR = crate::Reg<u32, _MDMA_C16TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16TCR;
#[doc = "`read()` method returns [mdma_c16tcr::R](mdma_c16tcr::R) reader structure"]
impl crate::Readable for MDMA_C16TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16tcr::W](mdma_c16tcr::W) writer structure"]
impl crate::Writable for MDMA_C16TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c16tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16bndtr](mdma_c16bndtr) module"]
pub type MDMA_C16BNDTR = crate::Reg<u32, _MDMA_C16BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16BNDTR;
#[doc = "`read()` method returns [mdma_c16bndtr::R](mdma_c16bndtr::R) reader structure"]
impl crate::Readable for MDMA_C16BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16bndtr::W](mdma_c16bndtr::W) writer structure"]
impl crate::Writable for MDMA_C16BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c16bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16sar](mdma_c16sar) module"]
pub type MDMA_C16SAR = crate::Reg<u32, _MDMA_C16SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16SAR;
#[doc = "`read()` method returns [mdma_c16sar::R](mdma_c16sar::R) reader structure"]
impl crate::Readable for MDMA_C16SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16sar::W](mdma_c16sar::W) writer structure"]
impl crate::Writable for MDMA_C16SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c16sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16dar](mdma_c16dar) module"]
pub type MDMA_C16DAR = crate::Reg<u32, _MDMA_C16DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16DAR;
#[doc = "`read()` method returns [mdma_c16dar::R](mdma_c16dar::R) reader structure"]
impl crate::Readable for MDMA_C16DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16dar::W](mdma_c16dar::W) writer structure"]
impl crate::Writable for MDMA_C16DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c16dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16brur](mdma_c16brur) module"]
pub type MDMA_C16BRUR = crate::Reg<u32, _MDMA_C16BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16BRUR;
#[doc = "`read()` method returns [mdma_c16brur::R](mdma_c16brur::R) reader structure"]
impl crate::Readable for MDMA_C16BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16brur::W](mdma_c16brur::W) writer structure"]
impl crate::Writable for MDMA_C16BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c16brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16lar](mdma_c16lar) module"]
pub type MDMA_C16LAR = crate::Reg<u32, _MDMA_C16LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16LAR;
#[doc = "`read()` method returns [mdma_c16lar::R](mdma_c16lar::R) reader structure"]
impl crate::Readable for MDMA_C16LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16lar::W](mdma_c16lar::W) writer structure"]
impl crate::Writable for MDMA_C16LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c16lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16tbr](mdma_c16tbr) module"]
pub type MDMA_C16TBR = crate::Reg<u32, _MDMA_C16TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16TBR;
#[doc = "`read()` method returns [mdma_c16tbr::R](mdma_c16tbr::R) reader structure"]
impl crate::Readable for MDMA_C16TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16tbr::W](mdma_c16tbr::W) writer structure"]
impl crate::Writable for MDMA_C16TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c16tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16mar](mdma_c16mar) module"]
pub type MDMA_C16MAR = crate::Reg<u32, _MDMA_C16MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16MAR;
#[doc = "`read()` method returns [mdma_c16mar::R](mdma_c16mar::R) reader structure"]
impl crate::Readable for MDMA_C16MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16mar::W](mdma_c16mar::W) writer structure"]
impl crate::Writable for MDMA_C16MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c16mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16mdr](mdma_c16mdr) module"]
pub type MDMA_C16MDR = crate::Reg<u32, _MDMA_C16MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C16MDR;
#[doc = "`read()` method returns [mdma_c16mdr::R](mdma_c16mdr::R) reader structure"]
impl crate::Readable for MDMA_C16MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c16mdr::W](mdma_c16mdr::W) writer structure"]
impl crate::Writable for MDMA_C16MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c16mdr;
#[doc = "MDMA channel 17 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17isr](mdma_c17isr) module"]
pub type MDMA_C17ISR = crate::Reg<u32, _MDMA_C17ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17ISR;
#[doc = "`read()` method returns [mdma_c17isr::R](mdma_c17isr::R) reader structure"]
impl crate::Readable for MDMA_C17ISR {}
#[doc = "MDMA channel 17 interrupt/status register"]
pub mod mdma_c17isr;
#[doc = "MDMA channel 17 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17ifcr](mdma_c17ifcr) module"]
pub type MDMA_C17IFCR = crate::Reg<u32, _MDMA_C17IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17IFCR;
#[doc = "`read()` method returns [mdma_c17ifcr::R](mdma_c17ifcr::R) reader structure"]
impl crate::Readable for MDMA_C17IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17ifcr::W](mdma_c17ifcr::W) writer structure"]
impl crate::Writable for MDMA_C17IFCR {}
#[doc = "MDMA channel 17 interrupt flag clear register"]
pub mod mdma_c17ifcr;
#[doc = "MDMA Channel 17 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17esr](mdma_c17esr) module"]
pub type MDMA_C17ESR = crate::Reg<u32, _MDMA_C17ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17ESR;
#[doc = "`read()` method returns [mdma_c17esr::R](mdma_c17esr::R) reader structure"]
impl crate::Readable for MDMA_C17ESR {}
#[doc = "MDMA Channel 17 error status register"]
pub mod mdma_c17esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17cr](mdma_c17cr) module"]
pub type MDMA_C17CR = crate::Reg<u32, _MDMA_C17CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17CR;
#[doc = "`read()` method returns [mdma_c17cr::R](mdma_c17cr::R) reader structure"]
impl crate::Readable for MDMA_C17CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17cr::W](mdma_c17cr::W) writer structure"]
impl crate::Writable for MDMA_C17CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c17cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17tcr](mdma_c17tcr) module"]
pub type MDMA_C17TCR = crate::Reg<u32, _MDMA_C17TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17TCR;
#[doc = "`read()` method returns [mdma_c17tcr::R](mdma_c17tcr::R) reader structure"]
impl crate::Readable for MDMA_C17TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17tcr::W](mdma_c17tcr::W) writer structure"]
impl crate::Writable for MDMA_C17TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c17tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17bndtr](mdma_c17bndtr) module"]
pub type MDMA_C17BNDTR = crate::Reg<u32, _MDMA_C17BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17BNDTR;
#[doc = "`read()` method returns [mdma_c17bndtr::R](mdma_c17bndtr::R) reader structure"]
impl crate::Readable for MDMA_C17BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17bndtr::W](mdma_c17bndtr::W) writer structure"]
impl crate::Writable for MDMA_C17BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c17bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17sar](mdma_c17sar) module"]
pub type MDMA_C17SAR = crate::Reg<u32, _MDMA_C17SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17SAR;
#[doc = "`read()` method returns [mdma_c17sar::R](mdma_c17sar::R) reader structure"]
impl crate::Readable for MDMA_C17SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17sar::W](mdma_c17sar::W) writer structure"]
impl crate::Writable for MDMA_C17SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c17sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17dar](mdma_c17dar) module"]
pub type MDMA_C17DAR = crate::Reg<u32, _MDMA_C17DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17DAR;
#[doc = "`read()` method returns [mdma_c17dar::R](mdma_c17dar::R) reader structure"]
impl crate::Readable for MDMA_C17DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17dar::W](mdma_c17dar::W) writer structure"]
impl crate::Writable for MDMA_C17DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c17dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17brur](mdma_c17brur) module"]
pub type MDMA_C17BRUR = crate::Reg<u32, _MDMA_C17BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17BRUR;
#[doc = "`read()` method returns [mdma_c17brur::R](mdma_c17brur::R) reader structure"]
impl crate::Readable for MDMA_C17BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17brur::W](mdma_c17brur::W) writer structure"]
impl crate::Writable for MDMA_C17BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c17brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17lar](mdma_c17lar) module"]
pub type MDMA_C17LAR = crate::Reg<u32, _MDMA_C17LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17LAR;
#[doc = "`read()` method returns [mdma_c17lar::R](mdma_c17lar::R) reader structure"]
impl crate::Readable for MDMA_C17LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17lar::W](mdma_c17lar::W) writer structure"]
impl crate::Writable for MDMA_C17LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c17lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17tbr](mdma_c17tbr) module"]
pub type MDMA_C17TBR = crate::Reg<u32, _MDMA_C17TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17TBR;
#[doc = "`read()` method returns [mdma_c17tbr::R](mdma_c17tbr::R) reader structure"]
impl crate::Readable for MDMA_C17TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17tbr::W](mdma_c17tbr::W) writer structure"]
impl crate::Writable for MDMA_C17TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c17tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17mar](mdma_c17mar) module"]
pub type MDMA_C17MAR = crate::Reg<u32, _MDMA_C17MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17MAR;
#[doc = "`read()` method returns [mdma_c17mar::R](mdma_c17mar::R) reader structure"]
impl crate::Readable for MDMA_C17MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17mar::W](mdma_c17mar::W) writer structure"]
impl crate::Writable for MDMA_C17MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c17mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c17mdr](mdma_c17mdr) module"]
pub type MDMA_C17MDR = crate::Reg<u32, _MDMA_C17MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C17MDR;
#[doc = "`read()` method returns [mdma_c17mdr::R](mdma_c17mdr::R) reader structure"]
impl crate::Readable for MDMA_C17MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c17mdr::W](mdma_c17mdr::W) writer structure"]
impl crate::Writable for MDMA_C17MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c17mdr;
#[doc = "MDMA channel 18 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18isr](mdma_c18isr) module"]
pub type MDMA_C18ISR = crate::Reg<u32, _MDMA_C18ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18ISR;
#[doc = "`read()` method returns [mdma_c18isr::R](mdma_c18isr::R) reader structure"]
impl crate::Readable for MDMA_C18ISR {}
#[doc = "MDMA channel 18 interrupt/status register"]
pub mod mdma_c18isr;
#[doc = "MDMA channel 18 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18ifcr](mdma_c18ifcr) module"]
pub type MDMA_C18IFCR = crate::Reg<u32, _MDMA_C18IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18IFCR;
#[doc = "`read()` method returns [mdma_c18ifcr::R](mdma_c18ifcr::R) reader structure"]
impl crate::Readable for MDMA_C18IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18ifcr::W](mdma_c18ifcr::W) writer structure"]
impl crate::Writable for MDMA_C18IFCR {}
#[doc = "MDMA channel 18 interrupt flag clear register"]
pub mod mdma_c18ifcr;
#[doc = "MDMA Channel 18 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18esr](mdma_c18esr) module"]
pub type MDMA_C18ESR = crate::Reg<u32, _MDMA_C18ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18ESR;
#[doc = "`read()` method returns [mdma_c18esr::R](mdma_c18esr::R) reader structure"]
impl crate::Readable for MDMA_C18ESR {}
#[doc = "MDMA Channel 18 error status register"]
pub mod mdma_c18esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18cr](mdma_c18cr) module"]
pub type MDMA_C18CR = crate::Reg<u32, _MDMA_C18CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18CR;
#[doc = "`read()` method returns [mdma_c18cr::R](mdma_c18cr::R) reader structure"]
impl crate::Readable for MDMA_C18CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18cr::W](mdma_c18cr::W) writer structure"]
impl crate::Writable for MDMA_C18CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c18cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18tcr](mdma_c18tcr) module"]
pub type MDMA_C18TCR = crate::Reg<u32, _MDMA_C18TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18TCR;
#[doc = "`read()` method returns [mdma_c18tcr::R](mdma_c18tcr::R) reader structure"]
impl crate::Readable for MDMA_C18TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18tcr::W](mdma_c18tcr::W) writer structure"]
impl crate::Writable for MDMA_C18TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c18tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18bndtr](mdma_c18bndtr) module"]
pub type MDMA_C18BNDTR = crate::Reg<u32, _MDMA_C18BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18BNDTR;
#[doc = "`read()` method returns [mdma_c18bndtr::R](mdma_c18bndtr::R) reader structure"]
impl crate::Readable for MDMA_C18BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18bndtr::W](mdma_c18bndtr::W) writer structure"]
impl crate::Writable for MDMA_C18BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c18bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18sar](mdma_c18sar) module"]
pub type MDMA_C18SAR = crate::Reg<u32, _MDMA_C18SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18SAR;
#[doc = "`read()` method returns [mdma_c18sar::R](mdma_c18sar::R) reader structure"]
impl crate::Readable for MDMA_C18SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18sar::W](mdma_c18sar::W) writer structure"]
impl crate::Writable for MDMA_C18SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c18sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18dar](mdma_c18dar) module"]
pub type MDMA_C18DAR = crate::Reg<u32, _MDMA_C18DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18DAR;
#[doc = "`read()` method returns [mdma_c18dar::R](mdma_c18dar::R) reader structure"]
impl crate::Readable for MDMA_C18DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18dar::W](mdma_c18dar::W) writer structure"]
impl crate::Writable for MDMA_C18DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c18dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18brur](mdma_c18brur) module"]
pub type MDMA_C18BRUR = crate::Reg<u32, _MDMA_C18BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18BRUR;
#[doc = "`read()` method returns [mdma_c18brur::R](mdma_c18brur::R) reader structure"]
impl crate::Readable for MDMA_C18BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18brur::W](mdma_c18brur::W) writer structure"]
impl crate::Writable for MDMA_C18BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c18brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18lar](mdma_c18lar) module"]
pub type MDMA_C18LAR = crate::Reg<u32, _MDMA_C18LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18LAR;
#[doc = "`read()` method returns [mdma_c18lar::R](mdma_c18lar::R) reader structure"]
impl crate::Readable for MDMA_C18LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18lar::W](mdma_c18lar::W) writer structure"]
impl crate::Writable for MDMA_C18LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c18lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18tbr](mdma_c18tbr) module"]
pub type MDMA_C18TBR = crate::Reg<u32, _MDMA_C18TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18TBR;
#[doc = "`read()` method returns [mdma_c18tbr::R](mdma_c18tbr::R) reader structure"]
impl crate::Readable for MDMA_C18TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18tbr::W](mdma_c18tbr::W) writer structure"]
impl crate::Writable for MDMA_C18TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c18tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18mar](mdma_c18mar) module"]
pub type MDMA_C18MAR = crate::Reg<u32, _MDMA_C18MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18MAR;
#[doc = "`read()` method returns [mdma_c18mar::R](mdma_c18mar::R) reader structure"]
impl crate::Readable for MDMA_C18MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18mar::W](mdma_c18mar::W) writer structure"]
impl crate::Writable for MDMA_C18MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c18mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c18mdr](mdma_c18mdr) module"]
pub type MDMA_C18MDR = crate::Reg<u32, _MDMA_C18MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C18MDR;
#[doc = "`read()` method returns [mdma_c18mdr::R](mdma_c18mdr::R) reader structure"]
impl crate::Readable for MDMA_C18MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c18mdr::W](mdma_c18mdr::W) writer structure"]
impl crate::Writable for MDMA_C18MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c18mdr;
#[doc = "MDMA channel 19 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19isr](mdma_c19isr) module"]
pub type MDMA_C19ISR = crate::Reg<u32, _MDMA_C19ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19ISR;
#[doc = "`read()` method returns [mdma_c19isr::R](mdma_c19isr::R) reader structure"]
impl crate::Readable for MDMA_C19ISR {}
#[doc = "MDMA channel 19 interrupt/status register"]
pub mod mdma_c19isr;
#[doc = "MDMA channel 19 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19ifcr](mdma_c19ifcr) module"]
pub type MDMA_C19IFCR = crate::Reg<u32, _MDMA_C19IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19IFCR;
#[doc = "`read()` method returns [mdma_c19ifcr::R](mdma_c19ifcr::R) reader structure"]
impl crate::Readable for MDMA_C19IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19ifcr::W](mdma_c19ifcr::W) writer structure"]
impl crate::Writable for MDMA_C19IFCR {}
#[doc = "MDMA channel 19 interrupt flag clear register"]
pub mod mdma_c19ifcr;
#[doc = "MDMA Channel 19 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19esr](mdma_c19esr) module"]
pub type MDMA_C19ESR = crate::Reg<u32, _MDMA_C19ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19ESR;
#[doc = "`read()` method returns [mdma_c19esr::R](mdma_c19esr::R) reader structure"]
impl crate::Readable for MDMA_C19ESR {}
#[doc = "MDMA Channel 19 error status register"]
pub mod mdma_c19esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19cr](mdma_c19cr) module"]
pub type MDMA_C19CR = crate::Reg<u32, _MDMA_C19CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19CR;
#[doc = "`read()` method returns [mdma_c19cr::R](mdma_c19cr::R) reader structure"]
impl crate::Readable for MDMA_C19CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19cr::W](mdma_c19cr::W) writer structure"]
impl crate::Writable for MDMA_C19CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c19cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19tcr](mdma_c19tcr) module"]
pub type MDMA_C19TCR = crate::Reg<u32, _MDMA_C19TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19TCR;
#[doc = "`read()` method returns [mdma_c19tcr::R](mdma_c19tcr::R) reader structure"]
impl crate::Readable for MDMA_C19TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19tcr::W](mdma_c19tcr::W) writer structure"]
impl crate::Writable for MDMA_C19TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c19tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19bndtr](mdma_c19bndtr) module"]
pub type MDMA_C19BNDTR = crate::Reg<u32, _MDMA_C19BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19BNDTR;
#[doc = "`read()` method returns [mdma_c19bndtr::R](mdma_c19bndtr::R) reader structure"]
impl crate::Readable for MDMA_C19BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19bndtr::W](mdma_c19bndtr::W) writer structure"]
impl crate::Writable for MDMA_C19BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c19bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19sar](mdma_c19sar) module"]
pub type MDMA_C19SAR = crate::Reg<u32, _MDMA_C19SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19SAR;
#[doc = "`read()` method returns [mdma_c19sar::R](mdma_c19sar::R) reader structure"]
impl crate::Readable for MDMA_C19SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19sar::W](mdma_c19sar::W) writer structure"]
impl crate::Writable for MDMA_C19SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c19sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19dar](mdma_c19dar) module"]
pub type MDMA_C19DAR = crate::Reg<u32, _MDMA_C19DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19DAR;
#[doc = "`read()` method returns [mdma_c19dar::R](mdma_c19dar::R) reader structure"]
impl crate::Readable for MDMA_C19DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19dar::W](mdma_c19dar::W) writer structure"]
impl crate::Writable for MDMA_C19DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c19dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19brur](mdma_c19brur) module"]
pub type MDMA_C19BRUR = crate::Reg<u32, _MDMA_C19BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19BRUR;
#[doc = "`read()` method returns [mdma_c19brur::R](mdma_c19brur::R) reader structure"]
impl crate::Readable for MDMA_C19BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19brur::W](mdma_c19brur::W) writer structure"]
impl crate::Writable for MDMA_C19BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c19brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19lar](mdma_c19lar) module"]
pub type MDMA_C19LAR = crate::Reg<u32, _MDMA_C19LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19LAR;
#[doc = "`read()` method returns [mdma_c19lar::R](mdma_c19lar::R) reader structure"]
impl crate::Readable for MDMA_C19LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19lar::W](mdma_c19lar::W) writer structure"]
impl crate::Writable for MDMA_C19LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c19lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19tbr](mdma_c19tbr) module"]
pub type MDMA_C19TBR = crate::Reg<u32, _MDMA_C19TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19TBR;
#[doc = "`read()` method returns [mdma_c19tbr::R](mdma_c19tbr::R) reader structure"]
impl crate::Readable for MDMA_C19TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19tbr::W](mdma_c19tbr::W) writer structure"]
impl crate::Writable for MDMA_C19TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c19tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19mar](mdma_c19mar) module"]
pub type MDMA_C19MAR = crate::Reg<u32, _MDMA_C19MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19MAR;
#[doc = "`read()` method returns [mdma_c19mar::R](mdma_c19mar::R) reader structure"]
impl crate::Readable for MDMA_C19MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19mar::W](mdma_c19mar::W) writer structure"]
impl crate::Writable for MDMA_C19MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c19mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19mdr](mdma_c19mdr) module"]
pub type MDMA_C19MDR = crate::Reg<u32, _MDMA_C19MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C19MDR;
#[doc = "`read()` method returns [mdma_c19mdr::R](mdma_c19mdr::R) reader structure"]
impl crate::Readable for MDMA_C19MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c19mdr::W](mdma_c19mdr::W) writer structure"]
impl crate::Writable for MDMA_C19MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c19mdr;
#[doc = "MDMA channel 20 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20isr](mdma_c20isr) module"]
pub type MDMA_C20ISR = crate::Reg<u32, _MDMA_C20ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20ISR;
#[doc = "`read()` method returns [mdma_c20isr::R](mdma_c20isr::R) reader structure"]
impl crate::Readable for MDMA_C20ISR {}
#[doc = "MDMA channel 20 interrupt/status register"]
pub mod mdma_c20isr;
#[doc = "MDMA channel 20 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20ifcr](mdma_c20ifcr) module"]
pub type MDMA_C20IFCR = crate::Reg<u32, _MDMA_C20IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20IFCR;
#[doc = "`read()` method returns [mdma_c20ifcr::R](mdma_c20ifcr::R) reader structure"]
impl crate::Readable for MDMA_C20IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20ifcr::W](mdma_c20ifcr::W) writer structure"]
impl crate::Writable for MDMA_C20IFCR {}
#[doc = "MDMA channel 20 interrupt flag clear register"]
pub mod mdma_c20ifcr;
#[doc = "MDMA Channel 20 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20esr](mdma_c20esr) module"]
pub type MDMA_C20ESR = crate::Reg<u32, _MDMA_C20ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20ESR;
#[doc = "`read()` method returns [mdma_c20esr::R](mdma_c20esr::R) reader structure"]
impl crate::Readable for MDMA_C20ESR {}
#[doc = "MDMA Channel 20 error status register"]
pub mod mdma_c20esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20cr](mdma_c20cr) module"]
pub type MDMA_C20CR = crate::Reg<u32, _MDMA_C20CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20CR;
#[doc = "`read()` method returns [mdma_c20cr::R](mdma_c20cr::R) reader structure"]
impl crate::Readable for MDMA_C20CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20cr::W](mdma_c20cr::W) writer structure"]
impl crate::Writable for MDMA_C20CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c20cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20tcr](mdma_c20tcr) module"]
pub type MDMA_C20TCR = crate::Reg<u32, _MDMA_C20TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20TCR;
#[doc = "`read()` method returns [mdma_c20tcr::R](mdma_c20tcr::R) reader structure"]
impl crate::Readable for MDMA_C20TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20tcr::W](mdma_c20tcr::W) writer structure"]
impl crate::Writable for MDMA_C20TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c20tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20bndtr](mdma_c20bndtr) module"]
pub type MDMA_C20BNDTR = crate::Reg<u32, _MDMA_C20BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20BNDTR;
#[doc = "`read()` method returns [mdma_c20bndtr::R](mdma_c20bndtr::R) reader structure"]
impl crate::Readable for MDMA_C20BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20bndtr::W](mdma_c20bndtr::W) writer structure"]
impl crate::Writable for MDMA_C20BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c20bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20sar](mdma_c20sar) module"]
pub type MDMA_C20SAR = crate::Reg<u32, _MDMA_C20SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20SAR;
#[doc = "`read()` method returns [mdma_c20sar::R](mdma_c20sar::R) reader structure"]
impl crate::Readable for MDMA_C20SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20sar::W](mdma_c20sar::W) writer structure"]
impl crate::Writable for MDMA_C20SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c20sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20dar](mdma_c20dar) module"]
pub type MDMA_C20DAR = crate::Reg<u32, _MDMA_C20DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20DAR;
#[doc = "`read()` method returns [mdma_c20dar::R](mdma_c20dar::R) reader structure"]
impl crate::Readable for MDMA_C20DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20dar::W](mdma_c20dar::W) writer structure"]
impl crate::Writable for MDMA_C20DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c20dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20brur](mdma_c20brur) module"]
pub type MDMA_C20BRUR = crate::Reg<u32, _MDMA_C20BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20BRUR;
#[doc = "`read()` method returns [mdma_c20brur::R](mdma_c20brur::R) reader structure"]
impl crate::Readable for MDMA_C20BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20brur::W](mdma_c20brur::W) writer structure"]
impl crate::Writable for MDMA_C20BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c20brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20lar](mdma_c20lar) module"]
pub type MDMA_C20LAR = crate::Reg<u32, _MDMA_C20LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20LAR;
#[doc = "`read()` method returns [mdma_c20lar::R](mdma_c20lar::R) reader structure"]
impl crate::Readable for MDMA_C20LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20lar::W](mdma_c20lar::W) writer structure"]
impl crate::Writable for MDMA_C20LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c20lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20tbr](mdma_c20tbr) module"]
pub type MDMA_C20TBR = crate::Reg<u32, _MDMA_C20TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20TBR;
#[doc = "`read()` method returns [mdma_c20tbr::R](mdma_c20tbr::R) reader structure"]
impl crate::Readable for MDMA_C20TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20tbr::W](mdma_c20tbr::W) writer structure"]
impl crate::Writable for MDMA_C20TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c20tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20mar](mdma_c20mar) module"]
pub type MDMA_C20MAR = crate::Reg<u32, _MDMA_C20MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20MAR;
#[doc = "`read()` method returns [mdma_c20mar::R](mdma_c20mar::R) reader structure"]
impl crate::Readable for MDMA_C20MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20mar::W](mdma_c20mar::W) writer structure"]
impl crate::Writable for MDMA_C20MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c20mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c20mdr](mdma_c20mdr) module"]
pub type MDMA_C20MDR = crate::Reg<u32, _MDMA_C20MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C20MDR;
#[doc = "`read()` method returns [mdma_c20mdr::R](mdma_c20mdr::R) reader structure"]
impl crate::Readable for MDMA_C20MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c20mdr::W](mdma_c20mdr::W) writer structure"]
impl crate::Writable for MDMA_C20MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c20mdr;
#[doc = "MDMA channel 21 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21isr](mdma_c21isr) module"]
pub type MDMA_C21ISR = crate::Reg<u32, _MDMA_C21ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21ISR;
#[doc = "`read()` method returns [mdma_c21isr::R](mdma_c21isr::R) reader structure"]
impl crate::Readable for MDMA_C21ISR {}
#[doc = "MDMA channel 21 interrupt/status register"]
pub mod mdma_c21isr;
#[doc = "MDMA channel 21 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21ifcr](mdma_c21ifcr) module"]
pub type MDMA_C21IFCR = crate::Reg<u32, _MDMA_C21IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21IFCR;
#[doc = "`read()` method returns [mdma_c21ifcr::R](mdma_c21ifcr::R) reader structure"]
impl crate::Readable for MDMA_C21IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21ifcr::W](mdma_c21ifcr::W) writer structure"]
impl crate::Writable for MDMA_C21IFCR {}
#[doc = "MDMA channel 21 interrupt flag clear register"]
pub mod mdma_c21ifcr;
#[doc = "MDMA Channel 21 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21esr](mdma_c21esr) module"]
pub type MDMA_C21ESR = crate::Reg<u32, _MDMA_C21ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21ESR;
#[doc = "`read()` method returns [mdma_c21esr::R](mdma_c21esr::R) reader structure"]
impl crate::Readable for MDMA_C21ESR {}
#[doc = "MDMA Channel 21 error status register"]
pub mod mdma_c21esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21cr](mdma_c21cr) module"]
pub type MDMA_C21CR = crate::Reg<u32, _MDMA_C21CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21CR;
#[doc = "`read()` method returns [mdma_c21cr::R](mdma_c21cr::R) reader structure"]
impl crate::Readable for MDMA_C21CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21cr::W](mdma_c21cr::W) writer structure"]
impl crate::Writable for MDMA_C21CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c21cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21tcr](mdma_c21tcr) module"]
pub type MDMA_C21TCR = crate::Reg<u32, _MDMA_C21TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21TCR;
#[doc = "`read()` method returns [mdma_c21tcr::R](mdma_c21tcr::R) reader structure"]
impl crate::Readable for MDMA_C21TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21tcr::W](mdma_c21tcr::W) writer structure"]
impl crate::Writable for MDMA_C21TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c21tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21bndtr](mdma_c21bndtr) module"]
pub type MDMA_C21BNDTR = crate::Reg<u32, _MDMA_C21BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21BNDTR;
#[doc = "`read()` method returns [mdma_c21bndtr::R](mdma_c21bndtr::R) reader structure"]
impl crate::Readable for MDMA_C21BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21bndtr::W](mdma_c21bndtr::W) writer structure"]
impl crate::Writable for MDMA_C21BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c21bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21sar](mdma_c21sar) module"]
pub type MDMA_C21SAR = crate::Reg<u32, _MDMA_C21SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21SAR;
#[doc = "`read()` method returns [mdma_c21sar::R](mdma_c21sar::R) reader structure"]
impl crate::Readable for MDMA_C21SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21sar::W](mdma_c21sar::W) writer structure"]
impl crate::Writable for MDMA_C21SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c21sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21dar](mdma_c21dar) module"]
pub type MDMA_C21DAR = crate::Reg<u32, _MDMA_C21DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21DAR;
#[doc = "`read()` method returns [mdma_c21dar::R](mdma_c21dar::R) reader structure"]
impl crate::Readable for MDMA_C21DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21dar::W](mdma_c21dar::W) writer structure"]
impl crate::Writable for MDMA_C21DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c21dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21brur](mdma_c21brur) module"]
pub type MDMA_C21BRUR = crate::Reg<u32, _MDMA_C21BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21BRUR;
#[doc = "`read()` method returns [mdma_c21brur::R](mdma_c21brur::R) reader structure"]
impl crate::Readable for MDMA_C21BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21brur::W](mdma_c21brur::W) writer structure"]
impl crate::Writable for MDMA_C21BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c21brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21lar](mdma_c21lar) module"]
pub type MDMA_C21LAR = crate::Reg<u32, _MDMA_C21LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21LAR;
#[doc = "`read()` method returns [mdma_c21lar::R](mdma_c21lar::R) reader structure"]
impl crate::Readable for MDMA_C21LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21lar::W](mdma_c21lar::W) writer structure"]
impl crate::Writable for MDMA_C21LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c21lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21tbr](mdma_c21tbr) module"]
pub type MDMA_C21TBR = crate::Reg<u32, _MDMA_C21TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21TBR;
#[doc = "`read()` method returns [mdma_c21tbr::R](mdma_c21tbr::R) reader structure"]
impl crate::Readable for MDMA_C21TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21tbr::W](mdma_c21tbr::W) writer structure"]
impl crate::Writable for MDMA_C21TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c21tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21mar](mdma_c21mar) module"]
pub type MDMA_C21MAR = crate::Reg<u32, _MDMA_C21MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21MAR;
#[doc = "`read()` method returns [mdma_c21mar::R](mdma_c21mar::R) reader structure"]
impl crate::Readable for MDMA_C21MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21mar::W](mdma_c21mar::W) writer structure"]
impl crate::Writable for MDMA_C21MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c21mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21mdr](mdma_c21mdr) module"]
pub type MDMA_C21MDR = crate::Reg<u32, _MDMA_C21MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C21MDR;
#[doc = "`read()` method returns [mdma_c21mdr::R](mdma_c21mdr::R) reader structure"]
impl crate::Readable for MDMA_C21MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c21mdr::W](mdma_c21mdr::W) writer structure"]
impl crate::Writable for MDMA_C21MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c21mdr;
#[doc = "MDMA channel 22 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22isr](mdma_c22isr) module"]
pub type MDMA_C22ISR = crate::Reg<u32, _MDMA_C22ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22ISR;
#[doc = "`read()` method returns [mdma_c22isr::R](mdma_c22isr::R) reader structure"]
impl crate::Readable for MDMA_C22ISR {}
#[doc = "MDMA channel 22 interrupt/status register"]
pub mod mdma_c22isr;
#[doc = "MDMA channel 22 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22ifcr](mdma_c22ifcr) module"]
pub type MDMA_C22IFCR = crate::Reg<u32, _MDMA_C22IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22IFCR;
#[doc = "`read()` method returns [mdma_c22ifcr::R](mdma_c22ifcr::R) reader structure"]
impl crate::Readable for MDMA_C22IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22ifcr::W](mdma_c22ifcr::W) writer structure"]
impl crate::Writable for MDMA_C22IFCR {}
#[doc = "MDMA channel 22 interrupt flag clear register"]
pub mod mdma_c22ifcr;
#[doc = "MDMA Channel 22 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22esr](mdma_c22esr) module"]
pub type MDMA_C22ESR = crate::Reg<u32, _MDMA_C22ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22ESR;
#[doc = "`read()` method returns [mdma_c22esr::R](mdma_c22esr::R) reader structure"]
impl crate::Readable for MDMA_C22ESR {}
#[doc = "MDMA Channel 22 error status register"]
pub mod mdma_c22esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22cr](mdma_c22cr) module"]
pub type MDMA_C22CR = crate::Reg<u32, _MDMA_C22CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22CR;
#[doc = "`read()` method returns [mdma_c22cr::R](mdma_c22cr::R) reader structure"]
impl crate::Readable for MDMA_C22CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22cr::W](mdma_c22cr::W) writer structure"]
impl crate::Writable for MDMA_C22CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c22cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22tcr](mdma_c22tcr) module"]
pub type MDMA_C22TCR = crate::Reg<u32, _MDMA_C22TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22TCR;
#[doc = "`read()` method returns [mdma_c22tcr::R](mdma_c22tcr::R) reader structure"]
impl crate::Readable for MDMA_C22TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22tcr::W](mdma_c22tcr::W) writer structure"]
impl crate::Writable for MDMA_C22TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c22tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22bndtr](mdma_c22bndtr) module"]
pub type MDMA_C22BNDTR = crate::Reg<u32, _MDMA_C22BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22BNDTR;
#[doc = "`read()` method returns [mdma_c22bndtr::R](mdma_c22bndtr::R) reader structure"]
impl crate::Readable for MDMA_C22BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22bndtr::W](mdma_c22bndtr::W) writer structure"]
impl crate::Writable for MDMA_C22BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c22bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22sar](mdma_c22sar) module"]
pub type MDMA_C22SAR = crate::Reg<u32, _MDMA_C22SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22SAR;
#[doc = "`read()` method returns [mdma_c22sar::R](mdma_c22sar::R) reader structure"]
impl crate::Readable for MDMA_C22SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22sar::W](mdma_c22sar::W) writer structure"]
impl crate::Writable for MDMA_C22SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c22sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22dar](mdma_c22dar) module"]
pub type MDMA_C22DAR = crate::Reg<u32, _MDMA_C22DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22DAR;
#[doc = "`read()` method returns [mdma_c22dar::R](mdma_c22dar::R) reader structure"]
impl crate::Readable for MDMA_C22DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22dar::W](mdma_c22dar::W) writer structure"]
impl crate::Writable for MDMA_C22DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c22dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22brur](mdma_c22brur) module"]
pub type MDMA_C22BRUR = crate::Reg<u32, _MDMA_C22BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22BRUR;
#[doc = "`read()` method returns [mdma_c22brur::R](mdma_c22brur::R) reader structure"]
impl crate::Readable for MDMA_C22BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22brur::W](mdma_c22brur::W) writer structure"]
impl crate::Writable for MDMA_C22BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c22brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22lar](mdma_c22lar) module"]
pub type MDMA_C22LAR = crate::Reg<u32, _MDMA_C22LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22LAR;
#[doc = "`read()` method returns [mdma_c22lar::R](mdma_c22lar::R) reader structure"]
impl crate::Readable for MDMA_C22LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22lar::W](mdma_c22lar::W) writer structure"]
impl crate::Writable for MDMA_C22LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c22lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22tbr](mdma_c22tbr) module"]
pub type MDMA_C22TBR = crate::Reg<u32, _MDMA_C22TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22TBR;
#[doc = "`read()` method returns [mdma_c22tbr::R](mdma_c22tbr::R) reader structure"]
impl crate::Readable for MDMA_C22TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22tbr::W](mdma_c22tbr::W) writer structure"]
impl crate::Writable for MDMA_C22TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c22tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22mar](mdma_c22mar) module"]
pub type MDMA_C22MAR = crate::Reg<u32, _MDMA_C22MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22MAR;
#[doc = "`read()` method returns [mdma_c22mar::R](mdma_c22mar::R) reader structure"]
impl crate::Readable for MDMA_C22MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22mar::W](mdma_c22mar::W) writer structure"]
impl crate::Writable for MDMA_C22MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c22mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22mdr](mdma_c22mdr) module"]
pub type MDMA_C22MDR = crate::Reg<u32, _MDMA_C22MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C22MDR;
#[doc = "`read()` method returns [mdma_c22mdr::R](mdma_c22mdr::R) reader structure"]
impl crate::Readable for MDMA_C22MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c22mdr::W](mdma_c22mdr::W) writer structure"]
impl crate::Writable for MDMA_C22MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c22mdr;
#[doc = "MDMA channel 23 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23isr](mdma_c23isr) module"]
pub type MDMA_C23ISR = crate::Reg<u32, _MDMA_C23ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23ISR;
#[doc = "`read()` method returns [mdma_c23isr::R](mdma_c23isr::R) reader structure"]
impl crate::Readable for MDMA_C23ISR {}
#[doc = "MDMA channel 23 interrupt/status register"]
pub mod mdma_c23isr;
#[doc = "MDMA channel 23 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23ifcr](mdma_c23ifcr) module"]
pub type MDMA_C23IFCR = crate::Reg<u32, _MDMA_C23IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23IFCR;
#[doc = "`read()` method returns [mdma_c23ifcr::R](mdma_c23ifcr::R) reader structure"]
impl crate::Readable for MDMA_C23IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23ifcr::W](mdma_c23ifcr::W) writer structure"]
impl crate::Writable for MDMA_C23IFCR {}
#[doc = "MDMA channel 23 interrupt flag clear register"]
pub mod mdma_c23ifcr;
#[doc = "MDMA Channel 23 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23esr](mdma_c23esr) module"]
pub type MDMA_C23ESR = crate::Reg<u32, _MDMA_C23ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23ESR;
#[doc = "`read()` method returns [mdma_c23esr::R](mdma_c23esr::R) reader structure"]
impl crate::Readable for MDMA_C23ESR {}
#[doc = "MDMA Channel 23 error status register"]
pub mod mdma_c23esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23cr](mdma_c23cr) module"]
pub type MDMA_C23CR = crate::Reg<u32, _MDMA_C23CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23CR;
#[doc = "`read()` method returns [mdma_c23cr::R](mdma_c23cr::R) reader structure"]
impl crate::Readable for MDMA_C23CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23cr::W](mdma_c23cr::W) writer structure"]
impl crate::Writable for MDMA_C23CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c23cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23tcr](mdma_c23tcr) module"]
pub type MDMA_C23TCR = crate::Reg<u32, _MDMA_C23TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23TCR;
#[doc = "`read()` method returns [mdma_c23tcr::R](mdma_c23tcr::R) reader structure"]
impl crate::Readable for MDMA_C23TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23tcr::W](mdma_c23tcr::W) writer structure"]
impl crate::Writable for MDMA_C23TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c23tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23bndtr](mdma_c23bndtr) module"]
pub type MDMA_C23BNDTR = crate::Reg<u32, _MDMA_C23BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23BNDTR;
#[doc = "`read()` method returns [mdma_c23bndtr::R](mdma_c23bndtr::R) reader structure"]
impl crate::Readable for MDMA_C23BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23bndtr::W](mdma_c23bndtr::W) writer structure"]
impl crate::Writable for MDMA_C23BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c23bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23sar](mdma_c23sar) module"]
pub type MDMA_C23SAR = crate::Reg<u32, _MDMA_C23SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23SAR;
#[doc = "`read()` method returns [mdma_c23sar::R](mdma_c23sar::R) reader structure"]
impl crate::Readable for MDMA_C23SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23sar::W](mdma_c23sar::W) writer structure"]
impl crate::Writable for MDMA_C23SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c23sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23dar](mdma_c23dar) module"]
pub type MDMA_C23DAR = crate::Reg<u32, _MDMA_C23DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23DAR;
#[doc = "`read()` method returns [mdma_c23dar::R](mdma_c23dar::R) reader structure"]
impl crate::Readable for MDMA_C23DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23dar::W](mdma_c23dar::W) writer structure"]
impl crate::Writable for MDMA_C23DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c23dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23brur](mdma_c23brur) module"]
pub type MDMA_C23BRUR = crate::Reg<u32, _MDMA_C23BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23BRUR;
#[doc = "`read()` method returns [mdma_c23brur::R](mdma_c23brur::R) reader structure"]
impl crate::Readable for MDMA_C23BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23brur::W](mdma_c23brur::W) writer structure"]
impl crate::Writable for MDMA_C23BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c23brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23lar](mdma_c23lar) module"]
pub type MDMA_C23LAR = crate::Reg<u32, _MDMA_C23LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23LAR;
#[doc = "`read()` method returns [mdma_c23lar::R](mdma_c23lar::R) reader structure"]
impl crate::Readable for MDMA_C23LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23lar::W](mdma_c23lar::W) writer structure"]
impl crate::Writable for MDMA_C23LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c23lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23tbr](mdma_c23tbr) module"]
pub type MDMA_C23TBR = crate::Reg<u32, _MDMA_C23TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23TBR;
#[doc = "`read()` method returns [mdma_c23tbr::R](mdma_c23tbr::R) reader structure"]
impl crate::Readable for MDMA_C23TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23tbr::W](mdma_c23tbr::W) writer structure"]
impl crate::Writable for MDMA_C23TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c23tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23mar](mdma_c23mar) module"]
pub type MDMA_C23MAR = crate::Reg<u32, _MDMA_C23MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23MAR;
#[doc = "`read()` method returns [mdma_c23mar::R](mdma_c23mar::R) reader structure"]
impl crate::Readable for MDMA_C23MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23mar::W](mdma_c23mar::W) writer structure"]
impl crate::Writable for MDMA_C23MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c23mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c23mdr](mdma_c23mdr) module"]
pub type MDMA_C23MDR = crate::Reg<u32, _MDMA_C23MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C23MDR;
#[doc = "`read()` method returns [mdma_c23mdr::R](mdma_c23mdr::R) reader structure"]
impl crate::Readable for MDMA_C23MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c23mdr::W](mdma_c23mdr::W) writer structure"]
impl crate::Writable for MDMA_C23MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c23mdr;
#[doc = "MDMA channel 24 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24isr](mdma_c24isr) module"]
pub type MDMA_C24ISR = crate::Reg<u32, _MDMA_C24ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24ISR;
#[doc = "`read()` method returns [mdma_c24isr::R](mdma_c24isr::R) reader structure"]
impl crate::Readable for MDMA_C24ISR {}
#[doc = "MDMA channel 24 interrupt/status register"]
pub mod mdma_c24isr;
#[doc = "MDMA channel 24 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24ifcr](mdma_c24ifcr) module"]
pub type MDMA_C24IFCR = crate::Reg<u32, _MDMA_C24IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24IFCR;
#[doc = "`read()` method returns [mdma_c24ifcr::R](mdma_c24ifcr::R) reader structure"]
impl crate::Readable for MDMA_C24IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24ifcr::W](mdma_c24ifcr::W) writer structure"]
impl crate::Writable for MDMA_C24IFCR {}
#[doc = "MDMA channel 24 interrupt flag clear register"]
pub mod mdma_c24ifcr;
#[doc = "MDMA Channel 24 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24esr](mdma_c24esr) module"]
pub type MDMA_C24ESR = crate::Reg<u32, _MDMA_C24ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24ESR;
#[doc = "`read()` method returns [mdma_c24esr::R](mdma_c24esr::R) reader structure"]
impl crate::Readable for MDMA_C24ESR {}
#[doc = "MDMA Channel 24 error status register"]
pub mod mdma_c24esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24cr](mdma_c24cr) module"]
pub type MDMA_C24CR = crate::Reg<u32, _MDMA_C24CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24CR;
#[doc = "`read()` method returns [mdma_c24cr::R](mdma_c24cr::R) reader structure"]
impl crate::Readable for MDMA_C24CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24cr::W](mdma_c24cr::W) writer structure"]
impl crate::Writable for MDMA_C24CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c24cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24tcr](mdma_c24tcr) module"]
pub type MDMA_C24TCR = crate::Reg<u32, _MDMA_C24TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24TCR;
#[doc = "`read()` method returns [mdma_c24tcr::R](mdma_c24tcr::R) reader structure"]
impl crate::Readable for MDMA_C24TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24tcr::W](mdma_c24tcr::W) writer structure"]
impl crate::Writable for MDMA_C24TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c24tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24bndtr](mdma_c24bndtr) module"]
pub type MDMA_C24BNDTR = crate::Reg<u32, _MDMA_C24BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24BNDTR;
#[doc = "`read()` method returns [mdma_c24bndtr::R](mdma_c24bndtr::R) reader structure"]
impl crate::Readable for MDMA_C24BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24bndtr::W](mdma_c24bndtr::W) writer structure"]
impl crate::Writable for MDMA_C24BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c24bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24sar](mdma_c24sar) module"]
pub type MDMA_C24SAR = crate::Reg<u32, _MDMA_C24SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24SAR;
#[doc = "`read()` method returns [mdma_c24sar::R](mdma_c24sar::R) reader structure"]
impl crate::Readable for MDMA_C24SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24sar::W](mdma_c24sar::W) writer structure"]
impl crate::Writable for MDMA_C24SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c24sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24dar](mdma_c24dar) module"]
pub type MDMA_C24DAR = crate::Reg<u32, _MDMA_C24DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24DAR;
#[doc = "`read()` method returns [mdma_c24dar::R](mdma_c24dar::R) reader structure"]
impl crate::Readable for MDMA_C24DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24dar::W](mdma_c24dar::W) writer structure"]
impl crate::Writable for MDMA_C24DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c24dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24brur](mdma_c24brur) module"]
pub type MDMA_C24BRUR = crate::Reg<u32, _MDMA_C24BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24BRUR;
#[doc = "`read()` method returns [mdma_c24brur::R](mdma_c24brur::R) reader structure"]
impl crate::Readable for MDMA_C24BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24brur::W](mdma_c24brur::W) writer structure"]
impl crate::Writable for MDMA_C24BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c24brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24lar](mdma_c24lar) module"]
pub type MDMA_C24LAR = crate::Reg<u32, _MDMA_C24LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24LAR;
#[doc = "`read()` method returns [mdma_c24lar::R](mdma_c24lar::R) reader structure"]
impl crate::Readable for MDMA_C24LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24lar::W](mdma_c24lar::W) writer structure"]
impl crate::Writable for MDMA_C24LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c24lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24tbr](mdma_c24tbr) module"]
pub type MDMA_C24TBR = crate::Reg<u32, _MDMA_C24TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24TBR;
#[doc = "`read()` method returns [mdma_c24tbr::R](mdma_c24tbr::R) reader structure"]
impl crate::Readable for MDMA_C24TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24tbr::W](mdma_c24tbr::W) writer structure"]
impl crate::Writable for MDMA_C24TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c24tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24mar](mdma_c24mar) module"]
pub type MDMA_C24MAR = crate::Reg<u32, _MDMA_C24MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24MAR;
#[doc = "`read()` method returns [mdma_c24mar::R](mdma_c24mar::R) reader structure"]
impl crate::Readable for MDMA_C24MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24mar::W](mdma_c24mar::W) writer structure"]
impl crate::Writable for MDMA_C24MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c24mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c24mdr](mdma_c24mdr) module"]
pub type MDMA_C24MDR = crate::Reg<u32, _MDMA_C24MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C24MDR;
#[doc = "`read()` method returns [mdma_c24mdr::R](mdma_c24mdr::R) reader structure"]
impl crate::Readable for MDMA_C24MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c24mdr::W](mdma_c24mdr::W) writer structure"]
impl crate::Writable for MDMA_C24MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c24mdr;
#[doc = "MDMA channel 25 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25isr](mdma_c25isr) module"]
pub type MDMA_C25ISR = crate::Reg<u32, _MDMA_C25ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25ISR;
#[doc = "`read()` method returns [mdma_c25isr::R](mdma_c25isr::R) reader structure"]
impl crate::Readable for MDMA_C25ISR {}
#[doc = "MDMA channel 25 interrupt/status register"]
pub mod mdma_c25isr;
#[doc = "MDMA channel 25 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25ifcr](mdma_c25ifcr) module"]
pub type MDMA_C25IFCR = crate::Reg<u32, _MDMA_C25IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25IFCR;
#[doc = "`read()` method returns [mdma_c25ifcr::R](mdma_c25ifcr::R) reader structure"]
impl crate::Readable for MDMA_C25IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25ifcr::W](mdma_c25ifcr::W) writer structure"]
impl crate::Writable for MDMA_C25IFCR {}
#[doc = "MDMA channel 25 interrupt flag clear register"]
pub mod mdma_c25ifcr;
#[doc = "MDMA Channel 25 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25esr](mdma_c25esr) module"]
pub type MDMA_C25ESR = crate::Reg<u32, _MDMA_C25ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25ESR;
#[doc = "`read()` method returns [mdma_c25esr::R](mdma_c25esr::R) reader structure"]
impl crate::Readable for MDMA_C25ESR {}
#[doc = "MDMA Channel 25 error status register"]
pub mod mdma_c25esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25cr](mdma_c25cr) module"]
pub type MDMA_C25CR = crate::Reg<u32, _MDMA_C25CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25CR;
#[doc = "`read()` method returns [mdma_c25cr::R](mdma_c25cr::R) reader structure"]
impl crate::Readable for MDMA_C25CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25cr::W](mdma_c25cr::W) writer structure"]
impl crate::Writable for MDMA_C25CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c25cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25tcr](mdma_c25tcr) module"]
pub type MDMA_C25TCR = crate::Reg<u32, _MDMA_C25TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25TCR;
#[doc = "`read()` method returns [mdma_c25tcr::R](mdma_c25tcr::R) reader structure"]
impl crate::Readable for MDMA_C25TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25tcr::W](mdma_c25tcr::W) writer structure"]
impl crate::Writable for MDMA_C25TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c25tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25bndtr](mdma_c25bndtr) module"]
pub type MDMA_C25BNDTR = crate::Reg<u32, _MDMA_C25BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25BNDTR;
#[doc = "`read()` method returns [mdma_c25bndtr::R](mdma_c25bndtr::R) reader structure"]
impl crate::Readable for MDMA_C25BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25bndtr::W](mdma_c25bndtr::W) writer structure"]
impl crate::Writable for MDMA_C25BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c25bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25sar](mdma_c25sar) module"]
pub type MDMA_C25SAR = crate::Reg<u32, _MDMA_C25SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25SAR;
#[doc = "`read()` method returns [mdma_c25sar::R](mdma_c25sar::R) reader structure"]
impl crate::Readable for MDMA_C25SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25sar::W](mdma_c25sar::W) writer structure"]
impl crate::Writable for MDMA_C25SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c25sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25dar](mdma_c25dar) module"]
pub type MDMA_C25DAR = crate::Reg<u32, _MDMA_C25DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25DAR;
#[doc = "`read()` method returns [mdma_c25dar::R](mdma_c25dar::R) reader structure"]
impl crate::Readable for MDMA_C25DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25dar::W](mdma_c25dar::W) writer structure"]
impl crate::Writable for MDMA_C25DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c25dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25brur](mdma_c25brur) module"]
pub type MDMA_C25BRUR = crate::Reg<u32, _MDMA_C25BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25BRUR;
#[doc = "`read()` method returns [mdma_c25brur::R](mdma_c25brur::R) reader structure"]
impl crate::Readable for MDMA_C25BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25brur::W](mdma_c25brur::W) writer structure"]
impl crate::Writable for MDMA_C25BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c25brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25lar](mdma_c25lar) module"]
pub type MDMA_C25LAR = crate::Reg<u32, _MDMA_C25LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25LAR;
#[doc = "`read()` method returns [mdma_c25lar::R](mdma_c25lar::R) reader structure"]
impl crate::Readable for MDMA_C25LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25lar::W](mdma_c25lar::W) writer structure"]
impl crate::Writable for MDMA_C25LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c25lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25tbr](mdma_c25tbr) module"]
pub type MDMA_C25TBR = crate::Reg<u32, _MDMA_C25TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25TBR;
#[doc = "`read()` method returns [mdma_c25tbr::R](mdma_c25tbr::R) reader structure"]
impl crate::Readable for MDMA_C25TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25tbr::W](mdma_c25tbr::W) writer structure"]
impl crate::Writable for MDMA_C25TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c25tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25mar](mdma_c25mar) module"]
pub type MDMA_C25MAR = crate::Reg<u32, _MDMA_C25MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25MAR;
#[doc = "`read()` method returns [mdma_c25mar::R](mdma_c25mar::R) reader structure"]
impl crate::Readable for MDMA_C25MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25mar::W](mdma_c25mar::W) writer structure"]
impl crate::Writable for MDMA_C25MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c25mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25mdr](mdma_c25mdr) module"]
pub type MDMA_C25MDR = crate::Reg<u32, _MDMA_C25MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C25MDR;
#[doc = "`read()` method returns [mdma_c25mdr::R](mdma_c25mdr::R) reader structure"]
impl crate::Readable for MDMA_C25MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c25mdr::W](mdma_c25mdr::W) writer structure"]
impl crate::Writable for MDMA_C25MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c25mdr;
#[doc = "MDMA channel 26 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26isr](mdma_c26isr) module"]
pub type MDMA_C26ISR = crate::Reg<u32, _MDMA_C26ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26ISR;
#[doc = "`read()` method returns [mdma_c26isr::R](mdma_c26isr::R) reader structure"]
impl crate::Readable for MDMA_C26ISR {}
#[doc = "MDMA channel 26 interrupt/status register"]
pub mod mdma_c26isr;
#[doc = "MDMA channel 26 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26ifcr](mdma_c26ifcr) module"]
pub type MDMA_C26IFCR = crate::Reg<u32, _MDMA_C26IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26IFCR;
#[doc = "`read()` method returns [mdma_c26ifcr::R](mdma_c26ifcr::R) reader structure"]
impl crate::Readable for MDMA_C26IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26ifcr::W](mdma_c26ifcr::W) writer structure"]
impl crate::Writable for MDMA_C26IFCR {}
#[doc = "MDMA channel 26 interrupt flag clear register"]
pub mod mdma_c26ifcr;
#[doc = "MDMA Channel 26 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26esr](mdma_c26esr) module"]
pub type MDMA_C26ESR = crate::Reg<u32, _MDMA_C26ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26ESR;
#[doc = "`read()` method returns [mdma_c26esr::R](mdma_c26esr::R) reader structure"]
impl crate::Readable for MDMA_C26ESR {}
#[doc = "MDMA Channel 26 error status register"]
pub mod mdma_c26esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26cr](mdma_c26cr) module"]
pub type MDMA_C26CR = crate::Reg<u32, _MDMA_C26CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26CR;
#[doc = "`read()` method returns [mdma_c26cr::R](mdma_c26cr::R) reader structure"]
impl crate::Readable for MDMA_C26CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26cr::W](mdma_c26cr::W) writer structure"]
impl crate::Writable for MDMA_C26CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c26cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26tcr](mdma_c26tcr) module"]
pub type MDMA_C26TCR = crate::Reg<u32, _MDMA_C26TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26TCR;
#[doc = "`read()` method returns [mdma_c26tcr::R](mdma_c26tcr::R) reader structure"]
impl crate::Readable for MDMA_C26TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26tcr::W](mdma_c26tcr::W) writer structure"]
impl crate::Writable for MDMA_C26TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c26tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26bndtr](mdma_c26bndtr) module"]
pub type MDMA_C26BNDTR = crate::Reg<u32, _MDMA_C26BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26BNDTR;
#[doc = "`read()` method returns [mdma_c26bndtr::R](mdma_c26bndtr::R) reader structure"]
impl crate::Readable for MDMA_C26BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26bndtr::W](mdma_c26bndtr::W) writer structure"]
impl crate::Writable for MDMA_C26BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c26bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26sar](mdma_c26sar) module"]
pub type MDMA_C26SAR = crate::Reg<u32, _MDMA_C26SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26SAR;
#[doc = "`read()` method returns [mdma_c26sar::R](mdma_c26sar::R) reader structure"]
impl crate::Readable for MDMA_C26SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26sar::W](mdma_c26sar::W) writer structure"]
impl crate::Writable for MDMA_C26SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c26sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26dar](mdma_c26dar) module"]
pub type MDMA_C26DAR = crate::Reg<u32, _MDMA_C26DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26DAR;
#[doc = "`read()` method returns [mdma_c26dar::R](mdma_c26dar::R) reader structure"]
impl crate::Readable for MDMA_C26DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26dar::W](mdma_c26dar::W) writer structure"]
impl crate::Writable for MDMA_C26DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c26dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26brur](mdma_c26brur) module"]
pub type MDMA_C26BRUR = crate::Reg<u32, _MDMA_C26BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26BRUR;
#[doc = "`read()` method returns [mdma_c26brur::R](mdma_c26brur::R) reader structure"]
impl crate::Readable for MDMA_C26BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26brur::W](mdma_c26brur::W) writer structure"]
impl crate::Writable for MDMA_C26BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c26brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26lar](mdma_c26lar) module"]
pub type MDMA_C26LAR = crate::Reg<u32, _MDMA_C26LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26LAR;
#[doc = "`read()` method returns [mdma_c26lar::R](mdma_c26lar::R) reader structure"]
impl crate::Readable for MDMA_C26LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26lar::W](mdma_c26lar::W) writer structure"]
impl crate::Writable for MDMA_C26LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c26lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26tbr](mdma_c26tbr) module"]
pub type MDMA_C26TBR = crate::Reg<u32, _MDMA_C26TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26TBR;
#[doc = "`read()` method returns [mdma_c26tbr::R](mdma_c26tbr::R) reader structure"]
impl crate::Readable for MDMA_C26TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26tbr::W](mdma_c26tbr::W) writer structure"]
impl crate::Writable for MDMA_C26TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c26tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26mar](mdma_c26mar) module"]
pub type MDMA_C26MAR = crate::Reg<u32, _MDMA_C26MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26MAR;
#[doc = "`read()` method returns [mdma_c26mar::R](mdma_c26mar::R) reader structure"]
impl crate::Readable for MDMA_C26MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26mar::W](mdma_c26mar::W) writer structure"]
impl crate::Writable for MDMA_C26MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c26mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c26mdr](mdma_c26mdr) module"]
pub type MDMA_C26MDR = crate::Reg<u32, _MDMA_C26MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C26MDR;
#[doc = "`read()` method returns [mdma_c26mdr::R](mdma_c26mdr::R) reader structure"]
impl crate::Readable for MDMA_C26MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c26mdr::W](mdma_c26mdr::W) writer structure"]
impl crate::Writable for MDMA_C26MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c26mdr;
#[doc = "MDMA channel 27 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27isr](mdma_c27isr) module"]
pub type MDMA_C27ISR = crate::Reg<u32, _MDMA_C27ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27ISR;
#[doc = "`read()` method returns [mdma_c27isr::R](mdma_c27isr::R) reader structure"]
impl crate::Readable for MDMA_C27ISR {}
#[doc = "MDMA channel 27 interrupt/status register"]
pub mod mdma_c27isr;
#[doc = "MDMA channel 27 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27ifcr](mdma_c27ifcr) module"]
pub type MDMA_C27IFCR = crate::Reg<u32, _MDMA_C27IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27IFCR;
#[doc = "`read()` method returns [mdma_c27ifcr::R](mdma_c27ifcr::R) reader structure"]
impl crate::Readable for MDMA_C27IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27ifcr::W](mdma_c27ifcr::W) writer structure"]
impl crate::Writable for MDMA_C27IFCR {}
#[doc = "MDMA channel 27 interrupt flag clear register"]
pub mod mdma_c27ifcr;
#[doc = "MDMA Channel 27 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27esr](mdma_c27esr) module"]
pub type MDMA_C27ESR = crate::Reg<u32, _MDMA_C27ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27ESR;
#[doc = "`read()` method returns [mdma_c27esr::R](mdma_c27esr::R) reader structure"]
impl crate::Readable for MDMA_C27ESR {}
#[doc = "MDMA Channel 27 error status register"]
pub mod mdma_c27esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27cr](mdma_c27cr) module"]
pub type MDMA_C27CR = crate::Reg<u32, _MDMA_C27CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27CR;
#[doc = "`read()` method returns [mdma_c27cr::R](mdma_c27cr::R) reader structure"]
impl crate::Readable for MDMA_C27CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27cr::W](mdma_c27cr::W) writer structure"]
impl crate::Writable for MDMA_C27CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c27cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27tcr](mdma_c27tcr) module"]
pub type MDMA_C27TCR = crate::Reg<u32, _MDMA_C27TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27TCR;
#[doc = "`read()` method returns [mdma_c27tcr::R](mdma_c27tcr::R) reader structure"]
impl crate::Readable for MDMA_C27TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27tcr::W](mdma_c27tcr::W) writer structure"]
impl crate::Writable for MDMA_C27TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c27tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27bndtr](mdma_c27bndtr) module"]
pub type MDMA_C27BNDTR = crate::Reg<u32, _MDMA_C27BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27BNDTR;
#[doc = "`read()` method returns [mdma_c27bndtr::R](mdma_c27bndtr::R) reader structure"]
impl crate::Readable for MDMA_C27BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27bndtr::W](mdma_c27bndtr::W) writer structure"]
impl crate::Writable for MDMA_C27BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c27bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27sar](mdma_c27sar) module"]
pub type MDMA_C27SAR = crate::Reg<u32, _MDMA_C27SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27SAR;
#[doc = "`read()` method returns [mdma_c27sar::R](mdma_c27sar::R) reader structure"]
impl crate::Readable for MDMA_C27SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27sar::W](mdma_c27sar::W) writer structure"]
impl crate::Writable for MDMA_C27SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c27sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27dar](mdma_c27dar) module"]
pub type MDMA_C27DAR = crate::Reg<u32, _MDMA_C27DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27DAR;
#[doc = "`read()` method returns [mdma_c27dar::R](mdma_c27dar::R) reader structure"]
impl crate::Readable for MDMA_C27DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27dar::W](mdma_c27dar::W) writer structure"]
impl crate::Writable for MDMA_C27DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c27dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27brur](mdma_c27brur) module"]
pub type MDMA_C27BRUR = crate::Reg<u32, _MDMA_C27BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27BRUR;
#[doc = "`read()` method returns [mdma_c27brur::R](mdma_c27brur::R) reader structure"]
impl crate::Readable for MDMA_C27BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27brur::W](mdma_c27brur::W) writer structure"]
impl crate::Writable for MDMA_C27BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c27brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27lar](mdma_c27lar) module"]
pub type MDMA_C27LAR = crate::Reg<u32, _MDMA_C27LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27LAR;
#[doc = "`read()` method returns [mdma_c27lar::R](mdma_c27lar::R) reader structure"]
impl crate::Readable for MDMA_C27LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27lar::W](mdma_c27lar::W) writer structure"]
impl crate::Writable for MDMA_C27LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c27lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27tbr](mdma_c27tbr) module"]
pub type MDMA_C27TBR = crate::Reg<u32, _MDMA_C27TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27TBR;
#[doc = "`read()` method returns [mdma_c27tbr::R](mdma_c27tbr::R) reader structure"]
impl crate::Readable for MDMA_C27TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27tbr::W](mdma_c27tbr::W) writer structure"]
impl crate::Writable for MDMA_C27TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c27tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27mar](mdma_c27mar) module"]
pub type MDMA_C27MAR = crate::Reg<u32, _MDMA_C27MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27MAR;
#[doc = "`read()` method returns [mdma_c27mar::R](mdma_c27mar::R) reader structure"]
impl crate::Readable for MDMA_C27MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27mar::W](mdma_c27mar::W) writer structure"]
impl crate::Writable for MDMA_C27MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c27mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c27mdr](mdma_c27mdr) module"]
pub type MDMA_C27MDR = crate::Reg<u32, _MDMA_C27MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C27MDR;
#[doc = "`read()` method returns [mdma_c27mdr::R](mdma_c27mdr::R) reader structure"]
impl crate::Readable for MDMA_C27MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c27mdr::W](mdma_c27mdr::W) writer structure"]
impl crate::Writable for MDMA_C27MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c27mdr;
#[doc = "MDMA channel 28 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28isr](mdma_c28isr) module"]
pub type MDMA_C28ISR = crate::Reg<u32, _MDMA_C28ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28ISR;
#[doc = "`read()` method returns [mdma_c28isr::R](mdma_c28isr::R) reader structure"]
impl crate::Readable for MDMA_C28ISR {}
#[doc = "MDMA channel 28 interrupt/status register"]
pub mod mdma_c28isr;
#[doc = "MDMA channel 28 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28ifcr](mdma_c28ifcr) module"]
pub type MDMA_C28IFCR = crate::Reg<u32, _MDMA_C28IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28IFCR;
#[doc = "`read()` method returns [mdma_c28ifcr::R](mdma_c28ifcr::R) reader structure"]
impl crate::Readable for MDMA_C28IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28ifcr::W](mdma_c28ifcr::W) writer structure"]
impl crate::Writable for MDMA_C28IFCR {}
#[doc = "MDMA channel 28 interrupt flag clear register"]
pub mod mdma_c28ifcr;
#[doc = "MDMA Channel 28 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28esr](mdma_c28esr) module"]
pub type MDMA_C28ESR = crate::Reg<u32, _MDMA_C28ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28ESR;
#[doc = "`read()` method returns [mdma_c28esr::R](mdma_c28esr::R) reader structure"]
impl crate::Readable for MDMA_C28ESR {}
#[doc = "MDMA Channel 28 error status register"]
pub mod mdma_c28esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28cr](mdma_c28cr) module"]
pub type MDMA_C28CR = crate::Reg<u32, _MDMA_C28CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28CR;
#[doc = "`read()` method returns [mdma_c28cr::R](mdma_c28cr::R) reader structure"]
impl crate::Readable for MDMA_C28CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28cr::W](mdma_c28cr::W) writer structure"]
impl crate::Writable for MDMA_C28CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c28cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28tcr](mdma_c28tcr) module"]
pub type MDMA_C28TCR = crate::Reg<u32, _MDMA_C28TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28TCR;
#[doc = "`read()` method returns [mdma_c28tcr::R](mdma_c28tcr::R) reader structure"]
impl crate::Readable for MDMA_C28TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28tcr::W](mdma_c28tcr::W) writer structure"]
impl crate::Writable for MDMA_C28TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c28tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28bndtr](mdma_c28bndtr) module"]
pub type MDMA_C28BNDTR = crate::Reg<u32, _MDMA_C28BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28BNDTR;
#[doc = "`read()` method returns [mdma_c28bndtr::R](mdma_c28bndtr::R) reader structure"]
impl crate::Readable for MDMA_C28BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28bndtr::W](mdma_c28bndtr::W) writer structure"]
impl crate::Writable for MDMA_C28BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c28bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28sar](mdma_c28sar) module"]
pub type MDMA_C28SAR = crate::Reg<u32, _MDMA_C28SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28SAR;
#[doc = "`read()` method returns [mdma_c28sar::R](mdma_c28sar::R) reader structure"]
impl crate::Readable for MDMA_C28SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28sar::W](mdma_c28sar::W) writer structure"]
impl crate::Writable for MDMA_C28SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c28sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28dar](mdma_c28dar) module"]
pub type MDMA_C28DAR = crate::Reg<u32, _MDMA_C28DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28DAR;
#[doc = "`read()` method returns [mdma_c28dar::R](mdma_c28dar::R) reader structure"]
impl crate::Readable for MDMA_C28DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28dar::W](mdma_c28dar::W) writer structure"]
impl crate::Writable for MDMA_C28DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c28dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28brur](mdma_c28brur) module"]
pub type MDMA_C28BRUR = crate::Reg<u32, _MDMA_C28BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28BRUR;
#[doc = "`read()` method returns [mdma_c28brur::R](mdma_c28brur::R) reader structure"]
impl crate::Readable for MDMA_C28BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28brur::W](mdma_c28brur::W) writer structure"]
impl crate::Writable for MDMA_C28BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c28brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28lar](mdma_c28lar) module"]
pub type MDMA_C28LAR = crate::Reg<u32, _MDMA_C28LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28LAR;
#[doc = "`read()` method returns [mdma_c28lar::R](mdma_c28lar::R) reader structure"]
impl crate::Readable for MDMA_C28LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28lar::W](mdma_c28lar::W) writer structure"]
impl crate::Writable for MDMA_C28LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c28lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28tbr](mdma_c28tbr) module"]
pub type MDMA_C28TBR = crate::Reg<u32, _MDMA_C28TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28TBR;
#[doc = "`read()` method returns [mdma_c28tbr::R](mdma_c28tbr::R) reader structure"]
impl crate::Readable for MDMA_C28TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28tbr::W](mdma_c28tbr::W) writer structure"]
impl crate::Writable for MDMA_C28TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c28tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28mar](mdma_c28mar) module"]
pub type MDMA_C28MAR = crate::Reg<u32, _MDMA_C28MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28MAR;
#[doc = "`read()` method returns [mdma_c28mar::R](mdma_c28mar::R) reader structure"]
impl crate::Readable for MDMA_C28MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28mar::W](mdma_c28mar::W) writer structure"]
impl crate::Writable for MDMA_C28MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c28mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c28mdr](mdma_c28mdr) module"]
pub type MDMA_C28MDR = crate::Reg<u32, _MDMA_C28MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C28MDR;
#[doc = "`read()` method returns [mdma_c28mdr::R](mdma_c28mdr::R) reader structure"]
impl crate::Readable for MDMA_C28MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c28mdr::W](mdma_c28mdr::W) writer structure"]
impl crate::Writable for MDMA_C28MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c28mdr;
#[doc = "MDMA channel 29 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29isr](mdma_c29isr) module"]
pub type MDMA_C29ISR = crate::Reg<u32, _MDMA_C29ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29ISR;
#[doc = "`read()` method returns [mdma_c29isr::R](mdma_c29isr::R) reader structure"]
impl crate::Readable for MDMA_C29ISR {}
#[doc = "MDMA channel 29 interrupt/status register"]
pub mod mdma_c29isr;
#[doc = "MDMA channel 29 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29ifcr](mdma_c29ifcr) module"]
pub type MDMA_C29IFCR = crate::Reg<u32, _MDMA_C29IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29IFCR;
#[doc = "`read()` method returns [mdma_c29ifcr::R](mdma_c29ifcr::R) reader structure"]
impl crate::Readable for MDMA_C29IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29ifcr::W](mdma_c29ifcr::W) writer structure"]
impl crate::Writable for MDMA_C29IFCR {}
#[doc = "MDMA channel 29 interrupt flag clear register"]
pub mod mdma_c29ifcr;
#[doc = "MDMA Channel 29 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29esr](mdma_c29esr) module"]
pub type MDMA_C29ESR = crate::Reg<u32, _MDMA_C29ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29ESR;
#[doc = "`read()` method returns [mdma_c29esr::R](mdma_c29esr::R) reader structure"]
impl crate::Readable for MDMA_C29ESR {}
#[doc = "MDMA Channel 29 error status register"]
pub mod mdma_c29esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29cr](mdma_c29cr) module"]
pub type MDMA_C29CR = crate::Reg<u32, _MDMA_C29CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29CR;
#[doc = "`read()` method returns [mdma_c29cr::R](mdma_c29cr::R) reader structure"]
impl crate::Readable for MDMA_C29CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29cr::W](mdma_c29cr::W) writer structure"]
impl crate::Writable for MDMA_C29CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c29cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29tcr](mdma_c29tcr) module"]
pub type MDMA_C29TCR = crate::Reg<u32, _MDMA_C29TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29TCR;
#[doc = "`read()` method returns [mdma_c29tcr::R](mdma_c29tcr::R) reader structure"]
impl crate::Readable for MDMA_C29TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29tcr::W](mdma_c29tcr::W) writer structure"]
impl crate::Writable for MDMA_C29TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c29tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29bndtr](mdma_c29bndtr) module"]
pub type MDMA_C29BNDTR = crate::Reg<u32, _MDMA_C29BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29BNDTR;
#[doc = "`read()` method returns [mdma_c29bndtr::R](mdma_c29bndtr::R) reader structure"]
impl crate::Readable for MDMA_C29BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29bndtr::W](mdma_c29bndtr::W) writer structure"]
impl crate::Writable for MDMA_C29BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c29bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29sar](mdma_c29sar) module"]
pub type MDMA_C29SAR = crate::Reg<u32, _MDMA_C29SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29SAR;
#[doc = "`read()` method returns [mdma_c29sar::R](mdma_c29sar::R) reader structure"]
impl crate::Readable for MDMA_C29SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29sar::W](mdma_c29sar::W) writer structure"]
impl crate::Writable for MDMA_C29SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c29sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29dar](mdma_c29dar) module"]
pub type MDMA_C29DAR = crate::Reg<u32, _MDMA_C29DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29DAR;
#[doc = "`read()` method returns [mdma_c29dar::R](mdma_c29dar::R) reader structure"]
impl crate::Readable for MDMA_C29DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29dar::W](mdma_c29dar::W) writer structure"]
impl crate::Writable for MDMA_C29DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c29dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29brur](mdma_c29brur) module"]
pub type MDMA_C29BRUR = crate::Reg<u32, _MDMA_C29BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29BRUR;
#[doc = "`read()` method returns [mdma_c29brur::R](mdma_c29brur::R) reader structure"]
impl crate::Readable for MDMA_C29BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29brur::W](mdma_c29brur::W) writer structure"]
impl crate::Writable for MDMA_C29BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c29brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29lar](mdma_c29lar) module"]
pub type MDMA_C29LAR = crate::Reg<u32, _MDMA_C29LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29LAR;
#[doc = "`read()` method returns [mdma_c29lar::R](mdma_c29lar::R) reader structure"]
impl crate::Readable for MDMA_C29LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29lar::W](mdma_c29lar::W) writer structure"]
impl crate::Writable for MDMA_C29LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c29lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29tbr](mdma_c29tbr) module"]
pub type MDMA_C29TBR = crate::Reg<u32, _MDMA_C29TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29TBR;
#[doc = "`read()` method returns [mdma_c29tbr::R](mdma_c29tbr::R) reader structure"]
impl crate::Readable for MDMA_C29TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29tbr::W](mdma_c29tbr::W) writer structure"]
impl crate::Writable for MDMA_C29TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c29tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29mar](mdma_c29mar) module"]
pub type MDMA_C29MAR = crate::Reg<u32, _MDMA_C29MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29MAR;
#[doc = "`read()` method returns [mdma_c29mar::R](mdma_c29mar::R) reader structure"]
impl crate::Readable for MDMA_C29MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29mar::W](mdma_c29mar::W) writer structure"]
impl crate::Writable for MDMA_C29MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c29mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29mdr](mdma_c29mdr) module"]
pub type MDMA_C29MDR = crate::Reg<u32, _MDMA_C29MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C29MDR;
#[doc = "`read()` method returns [mdma_c29mdr::R](mdma_c29mdr::R) reader structure"]
impl crate::Readable for MDMA_C29MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c29mdr::W](mdma_c29mdr::W) writer structure"]
impl crate::Writable for MDMA_C29MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c29mdr;
#[doc = "MDMA channel 30 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30isr](mdma_c30isr) module"]
pub type MDMA_C30ISR = crate::Reg<u32, _MDMA_C30ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30ISR;
#[doc = "`read()` method returns [mdma_c30isr::R](mdma_c30isr::R) reader structure"]
impl crate::Readable for MDMA_C30ISR {}
#[doc = "MDMA channel 30 interrupt/status register"]
pub mod mdma_c30isr;
#[doc = "MDMA channel 30 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30ifcr](mdma_c30ifcr) module"]
pub type MDMA_C30IFCR = crate::Reg<u32, _MDMA_C30IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30IFCR;
#[doc = "`read()` method returns [mdma_c30ifcr::R](mdma_c30ifcr::R) reader structure"]
impl crate::Readable for MDMA_C30IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30ifcr::W](mdma_c30ifcr::W) writer structure"]
impl crate::Writable for MDMA_C30IFCR {}
#[doc = "MDMA channel 30 interrupt flag clear register"]
pub mod mdma_c30ifcr;
#[doc = "MDMA Channel 30 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30esr](mdma_c30esr) module"]
pub type MDMA_C30ESR = crate::Reg<u32, _MDMA_C30ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30ESR;
#[doc = "`read()` method returns [mdma_c30esr::R](mdma_c30esr::R) reader structure"]
impl crate::Readable for MDMA_C30ESR {}
#[doc = "MDMA Channel 30 error status register"]
pub mod mdma_c30esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30cr](mdma_c30cr) module"]
pub type MDMA_C30CR = crate::Reg<u32, _MDMA_C30CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30CR;
#[doc = "`read()` method returns [mdma_c30cr::R](mdma_c30cr::R) reader structure"]
impl crate::Readable for MDMA_C30CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30cr::W](mdma_c30cr::W) writer structure"]
impl crate::Writable for MDMA_C30CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c30cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30tcr](mdma_c30tcr) module"]
pub type MDMA_C30TCR = crate::Reg<u32, _MDMA_C30TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30TCR;
#[doc = "`read()` method returns [mdma_c30tcr::R](mdma_c30tcr::R) reader structure"]
impl crate::Readable for MDMA_C30TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30tcr::W](mdma_c30tcr::W) writer structure"]
impl crate::Writable for MDMA_C30TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c30tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30bndtr](mdma_c30bndtr) module"]
pub type MDMA_C30BNDTR = crate::Reg<u32, _MDMA_C30BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30BNDTR;
#[doc = "`read()` method returns [mdma_c30bndtr::R](mdma_c30bndtr::R) reader structure"]
impl crate::Readable for MDMA_C30BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30bndtr::W](mdma_c30bndtr::W) writer structure"]
impl crate::Writable for MDMA_C30BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c30bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30sar](mdma_c30sar) module"]
pub type MDMA_C30SAR = crate::Reg<u32, _MDMA_C30SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30SAR;
#[doc = "`read()` method returns [mdma_c30sar::R](mdma_c30sar::R) reader structure"]
impl crate::Readable for MDMA_C30SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30sar::W](mdma_c30sar::W) writer structure"]
impl crate::Writable for MDMA_C30SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c30sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30dar](mdma_c30dar) module"]
pub type MDMA_C30DAR = crate::Reg<u32, _MDMA_C30DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30DAR;
#[doc = "`read()` method returns [mdma_c30dar::R](mdma_c30dar::R) reader structure"]
impl crate::Readable for MDMA_C30DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30dar::W](mdma_c30dar::W) writer structure"]
impl crate::Writable for MDMA_C30DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c30dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30brur](mdma_c30brur) module"]
pub type MDMA_C30BRUR = crate::Reg<u32, _MDMA_C30BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30BRUR;
#[doc = "`read()` method returns [mdma_c30brur::R](mdma_c30brur::R) reader structure"]
impl crate::Readable for MDMA_C30BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30brur::W](mdma_c30brur::W) writer structure"]
impl crate::Writable for MDMA_C30BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c30brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30lar](mdma_c30lar) module"]
pub type MDMA_C30LAR = crate::Reg<u32, _MDMA_C30LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30LAR;
#[doc = "`read()` method returns [mdma_c30lar::R](mdma_c30lar::R) reader structure"]
impl crate::Readable for MDMA_C30LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30lar::W](mdma_c30lar::W) writer structure"]
impl crate::Writable for MDMA_C30LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c30lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30tbr](mdma_c30tbr) module"]
pub type MDMA_C30TBR = crate::Reg<u32, _MDMA_C30TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30TBR;
#[doc = "`read()` method returns [mdma_c30tbr::R](mdma_c30tbr::R) reader structure"]
impl crate::Readable for MDMA_C30TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30tbr::W](mdma_c30tbr::W) writer structure"]
impl crate::Writable for MDMA_C30TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c30tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30mar](mdma_c30mar) module"]
pub type MDMA_C30MAR = crate::Reg<u32, _MDMA_C30MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30MAR;
#[doc = "`read()` method returns [mdma_c30mar::R](mdma_c30mar::R) reader structure"]
impl crate::Readable for MDMA_C30MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30mar::W](mdma_c30mar::W) writer structure"]
impl crate::Writable for MDMA_C30MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c30mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c30mdr](mdma_c30mdr) module"]
pub type MDMA_C30MDR = crate::Reg<u32, _MDMA_C30MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C30MDR;
#[doc = "`read()` method returns [mdma_c30mdr::R](mdma_c30mdr::R) reader structure"]
impl crate::Readable for MDMA_C30MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c30mdr::W](mdma_c30mdr::W) writer structure"]
impl crate::Writable for MDMA_C30MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c30mdr;
#[doc = "MDMA channel 31 interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31isr](mdma_c31isr) module"]
pub type MDMA_C31ISR = crate::Reg<u32, _MDMA_C31ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31ISR;
#[doc = "`read()` method returns [mdma_c31isr::R](mdma_c31isr::R) reader structure"]
impl crate::Readable for MDMA_C31ISR {}
#[doc = "MDMA channel 31 interrupt/status register"]
pub mod mdma_c31isr;
#[doc = "MDMA channel 31 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31ifcr](mdma_c31ifcr) module"]
pub type MDMA_C31IFCR = crate::Reg<u32, _MDMA_C31IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31IFCR;
#[doc = "`read()` method returns [mdma_c31ifcr::R](mdma_c31ifcr::R) reader structure"]
impl crate::Readable for MDMA_C31IFCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31ifcr::W](mdma_c31ifcr::W) writer structure"]
impl crate::Writable for MDMA_C31IFCR {}
#[doc = "MDMA channel 31 interrupt flag clear register"]
pub mod mdma_c31ifcr;
#[doc = "MDMA Channel 31 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31esr](mdma_c31esr) module"]
pub type MDMA_C31ESR = crate::Reg<u32, _MDMA_C31ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31ESR;
#[doc = "`read()` method returns [mdma_c31esr::R](mdma_c31esr::R) reader structure"]
impl crate::Readable for MDMA_C31ESR {}
#[doc = "MDMA Channel 31 error status register"]
pub mod mdma_c31esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31cr](mdma_c31cr) module"]
pub type MDMA_C31CR = crate::Reg<u32, _MDMA_C31CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31CR;
#[doc = "`read()` method returns [mdma_c31cr::R](mdma_c31cr::R) reader structure"]
impl crate::Readable for MDMA_C31CR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31cr::W](mdma_c31cr::W) writer structure"]
impl crate::Writable for MDMA_C31CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c31cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31tcr](mdma_c31tcr) module"]
pub type MDMA_C31TCR = crate::Reg<u32, _MDMA_C31TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31TCR;
#[doc = "`read()` method returns [mdma_c31tcr::R](mdma_c31tcr::R) reader structure"]
impl crate::Readable for MDMA_C31TCR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31tcr::W](mdma_c31tcr::W) writer structure"]
impl crate::Writable for MDMA_C31TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c31tcr;
#[doc = "MDMA Channel block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31bndtr](mdma_c31bndtr) module"]
pub type MDMA_C31BNDTR = crate::Reg<u32, _MDMA_C31BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31BNDTR;
#[doc = "`read()` method returns [mdma_c31bndtr::R](mdma_c31bndtr::R) reader structure"]
impl crate::Readable for MDMA_C31BNDTR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31bndtr::W](mdma_c31bndtr::W) writer structure"]
impl crate::Writable for MDMA_C31BNDTR {}
#[doc = "MDMA Channel block number of data register"]
pub mod mdma_c31bndtr;
#[doc = "MDMA channel source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31sar](mdma_c31sar) module"]
pub type MDMA_C31SAR = crate::Reg<u32, _MDMA_C31SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31SAR;
#[doc = "`read()` method returns [mdma_c31sar::R](mdma_c31sar::R) reader structure"]
impl crate::Readable for MDMA_C31SAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31sar::W](mdma_c31sar::W) writer structure"]
impl crate::Writable for MDMA_C31SAR {}
#[doc = "MDMA channel source address register"]
pub mod mdma_c31sar;
#[doc = "MDMA channel destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31dar](mdma_c31dar) module"]
pub type MDMA_C31DAR = crate::Reg<u32, _MDMA_C31DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31DAR;
#[doc = "`read()` method returns [mdma_c31dar::R](mdma_c31dar::R) reader structure"]
impl crate::Readable for MDMA_C31DAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31dar::W](mdma_c31dar::W) writer structure"]
impl crate::Writable for MDMA_C31DAR {}
#[doc = "MDMA channel destination address register"]
pub mod mdma_c31dar;
#[doc = "MDMA channel Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31brur](mdma_c31brur) module"]
pub type MDMA_C31BRUR = crate::Reg<u32, _MDMA_C31BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31BRUR;
#[doc = "`read()` method returns [mdma_c31brur::R](mdma_c31brur::R) reader structure"]
impl crate::Readable for MDMA_C31BRUR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31brur::W](mdma_c31brur::W) writer structure"]
impl crate::Writable for MDMA_C31BRUR {}
#[doc = "MDMA channel Block Repeat address Update register"]
pub mod mdma_c31brur;
#[doc = "MDMA channel Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31lar](mdma_c31lar) module"]
pub type MDMA_C31LAR = crate::Reg<u32, _MDMA_C31LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31LAR;
#[doc = "`read()` method returns [mdma_c31lar::R](mdma_c31lar::R) reader structure"]
impl crate::Readable for MDMA_C31LAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31lar::W](mdma_c31lar::W) writer structure"]
impl crate::Writable for MDMA_C31LAR {}
#[doc = "MDMA channel Link Address register"]
pub mod mdma_c31lar;
#[doc = "MDMA channel Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31tbr](mdma_c31tbr) module"]
pub type MDMA_C31TBR = crate::Reg<u32, _MDMA_C31TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31TBR;
#[doc = "`read()` method returns [mdma_c31tbr::R](mdma_c31tbr::R) reader structure"]
impl crate::Readable for MDMA_C31TBR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31tbr::W](mdma_c31tbr::W) writer structure"]
impl crate::Writable for MDMA_C31TBR {}
#[doc = "MDMA channel Trigger and Bus selection Register"]
pub mod mdma_c31tbr;
#[doc = "MDMA channel Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31mar](mdma_c31mar) module"]
pub type MDMA_C31MAR = crate::Reg<u32, _MDMA_C31MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31MAR;
#[doc = "`read()` method returns [mdma_c31mar::R](mdma_c31mar::R) reader structure"]
impl crate::Readable for MDMA_C31MAR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31mar::W](mdma_c31mar::W) writer structure"]
impl crate::Writable for MDMA_C31MAR {}
#[doc = "MDMA channel Mask address register"]
pub mod mdma_c31mar;
#[doc = "MDMA channel Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c31mdr](mdma_c31mdr) module"]
pub type MDMA_C31MDR = crate::Reg<u32, _MDMA_C31MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMA_C31MDR;
#[doc = "`read()` method returns [mdma_c31mdr::R](mdma_c31mdr::R) reader structure"]
impl crate::Readable for MDMA_C31MDR {}
#[doc = "`write(|w| ..)` method takes [mdma_c31mdr::W](mdma_c31mdr::W) writer structure"]
impl crate::Writable for MDMA_C31MDR {}
#[doc = "MDMA channel Mask Data register"]
pub mod mdma_c31mdr;
