/*
 * Copyright (C) 2021  Oakes, Gregory C. <gregcoakes@gmail.com>
 * Author: Oakes, Gregory C. <gregcoakes@gmail.com>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation, either
 * version 3 of the License, or any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program.  If not, see
 * <http://www.gnu.org/licenses/>.
 */

use crate::{util::BitArray, FixedStr, Reserved, TransmuteSafe};

use modular_bitfield::prelude::*;

#[test_structure(size = 4096)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdCtrl {
    #[loc(0:1)]
    pub vid: u16,
    #[loc(2:3)]
    pub ssvid: u16,
    #[loc(4:23)]
    pub sn: FixedStr<20>,
    #[loc(24:63)]
    pub mn: FixedStr<40>,
    #[loc(64:71)]
    pub fr: FixedStr<8>,
    #[loc(72:72)]
    pub rab: u8,
    #[loc(73:75)]
    pub ieee: Ieee,
    #[loc(76:76)]
    pub cmic: Cmic,
    #[loc(77:77)]
    pub mdts: u8,
    #[loc(78:79)]
    pub cntlid: u16,
    #[loc(80:83)]
    pub ver: u32,
    #[loc(84:87)]
    pub rtd3r: u32,
    #[loc(88:91)]
    pub rtd3e: u32,
    #[loc(92:95)]
    pub oaes: Oaes,
    #[loc(96:99)]
    pub ctrattr: CtrlAttr,
    #[loc(100:101)]
    pub rrls: BitArray<2>,
    #[loc(102:110)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd102: Reserved<9>,
    #[loc(111:111)]
    pub cntrltype: CtrlType,
    // TODO: Assess whether this has any value in destructured form. It appears
    // to simply be a unique identifier for the type of device.
    /// FRU Globally Unique Identifier (Big Endian)
    #[loc(112:127)]
    pub fguid: u128,
    #[loc(128:133)]
    pub crdt: [u16; 3],
    #[loc(134:239)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd134: Reserved<106>,
    // Separately defined reserved because the second one is from the NVMe-MI
    // spec.
    #[loc(240:252)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd240: Reserved<13>,
    #[loc(253:253)]
    pub nvmsr: Nvmsr,
    #[loc(254:254)]
    pub vwci: Vwci,
    #[loc(255:255)]
    pub mec: Mec,
    #[loc(256:257)]
    pub oacs: Oacs,
    #[loc(258:258)]
    pub acl: u8,
    #[loc(259:259)]
    pub aerl: u8,
    #[loc(260:260)]
    pub frmw: Frmw,
    #[loc(261:261)]
    pub lpa: Lpa,
    #[loc(262:262)]
    pub elpe: u8,
    #[loc(263:263)]
    pub npss: u8,
    #[loc(264:264)]
    pub avscc: Avscc,
    #[loc(265:265)]
    pub apsta: Apsta,
    #[loc(266:267)]
    pub wctemp: u16,
    #[loc(268:269)]
    pub cctemp: u16,
    #[loc(270:271)]
    pub mtfa: u16,
    #[loc(272:275)]
    pub hmpre: u32,
    #[loc(276:279)]
    pub hmmin: u32,
    #[loc(280:295)]
    pub tnvmcap: u128,
    #[loc(296:311)]
    pub unvmcap: u128,
    #[loc(312:315)]
    pub rpmbs: Rpmbs,
    #[loc(316:317)]
    pub edst: u16,
    #[loc(318:318)]
    pub dsto: Dsto,
    #[loc(319:319)]
    pub fwug: u8,
    #[loc(320:321)]
    pub kas: u16,
    #[loc(322:323)]
    pub hctma: Hctma,
    #[loc(324:325)]
    pub mntmt: u16,
    #[loc(326:327)]
    pub mxtmt: u16,
    #[loc(328:331)]
    pub sanicap: Sanicap,
    #[loc(332:335)]
    pub hmminds: u32,
    #[loc(336:337)]
    pub hmmaxd: u16,
    #[loc(338:339)]
    pub nsetidmax: u16,
    #[loc(340:341)]
    pub endgidmax: u16,
    #[loc(342:342)]
    pub anatt: u8,
    #[loc(343:343)]
    pub anacap: Anacap,
    #[loc(344:347)]
    pub anagrpmax: u32,
    #[loc(348:351)]
    pub nanagrpid: u32,
    #[loc(352:355)]
    pub pels: u32,
    #[loc(356:511)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd356: Reserved<156>,
    #[loc(512:512)]
    pub sqes: QueueEntrySize,
    #[loc(513:513)]
    pub cqes: QueueEntrySize,
    #[loc(514:515)]
    pub maxcmd: u16,
    #[loc(516:519)]
    pub nn: u32,
    #[loc(520:521)]
    pub oncs: Oncs,
    #[loc(522:523)]
    pub fuses: Fuses,
    #[loc(524:524)]
    pub fna: Fna,
    #[loc(525:525)]
    pub vwc: u8,
    #[loc(526:527)]
    pub awun: u16,
    #[loc(528:529)]
    pub awupf: u16,
    #[loc(530:530)]
    pub nvscc: Nvscc,
    #[loc(531:531)]
    pub nwpc: Nwpc,
    #[loc(532:533)]
    pub acwu: u16,
    #[loc(534:535)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd534: Reserved<2>,
    #[loc(536:539)]
    pub sgls: Sgls,
    #[loc(540:543)]
    pub mnan: u32,
    #[loc(544:767)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd544: Reserved<224>,
    // TODO: Properly handle null termination.
    #[loc(768:1023)]
    pub subnqn: FixedStr<256>,
    #[loc(1024:1791)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd1024: Reserved<768>,
    // TODO: NVMe over Fabrics.
    #[loc(1792:2047)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd1792: Reserved<256>,

    #[loc(2048:3071)]
    pub psds: [PowerState; 32],
    #[loc(3072:4095)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd3072: Reserved<1024>,
}

impl TransmuteSafe for IdCtrl {}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u32", from = "u32"))]
pub struct Ieee([u8; 3]);

impl From<Ieee> for u32 {
    fn from(value: Ieee) -> Self {
        let mut tmp = [0u8; 4];
        tmp[0..=2].clone_from_slice(&value.0[..]);
        u32::from_le_bytes(tmp)
    }
}

impl From<u32> for Ieee {
    fn from(value: u32) -> Self {
        let mut tmp = [0u8; 3];
        tmp[..].copy_from_slice(&value.to_le_bytes()[0..=2]);
        Ieee(tmp)
    }
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Cmic {
    pub multiple_subsys: bool,
    pub multiple_ctrls: bool,
    pub sr_iov: bool,
    pub asym_nmsp_access_reporting: bool,
    #[skip]
    __rsvd: B4,
}

#[bitfield]
#[repr(u32)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u32", from = "u32"))]
pub struct Oaes {
    #[skip]
    __rsvd: B8,
    pub nmsp_attr: bool,
    pub fw_activation: bool,
    #[skip]
    __rsvd: B1,
    pub asym_nmsp_access: bool,
    pub predictable_latency_aggregate: bool,
    pub lba_status_info: bool,
    pub endur_grp_aggregate: bool,
    #[skip]
    __rsvd: B17,
}

#[bitfield]
#[repr(u32)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u32", from = "u32"))]
pub struct CtrlAttr {
    pub host_id_128bit: bool,
    pub non_op_pwr_state_permissive: bool,
    pub nvm_sets: bool,
    pub read_recovery_levels: bool,
    pub endur_grps: bool,
    pub predictable_latency: bool,
    pub tbkas: bool,
    pub nmsp_granularity: bool,
    pub sq_assoc: bool,
    pub uuid_list: bool,
    #[skip]
    __rsvd: B22,
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CtrlType {
    NotReported = 0x00,
    IoCtrl = 0x01,
    DiscoveryCtrl = 0x02,
    AdminCtrl = 0x03,
    // TODO: Make this fill the repr space so there is no undefined behavior
    // when transmuting. Through testing, this would result in NotReported being
    // the enum value, if the actual value lies within the reserved space.
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Nvmsr {
    pub nvmesd: bool,
    pub nvmee: bool,
    #[skip]
    __rsvd: B6,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Vwci {
    pub vwcr: B7,
    pub vwcrv: bool,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Mec {
    pub i2c: bool,
    pub pcie: bool,
    #[skip]
    __rsvd: B6,
}

#[bitfield]
#[repr(u16)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u16", from = "u16"))]
pub struct Oacs {
    pub sec_send_rec: bool,
    pub format_nvm: bool,
    pub fw_commit: bool,
    pub nmsp_mgmt: bool,
    pub dev_self_test: bool,
    pub directives: bool,
    pub nvme_mi: bool,
    pub virt_mgmt: bool,
    pub doorbell_buf_conf: bool,
    pub get_lba_status: bool,
    #[skip]
    __rsvd: B6,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Frmw {
    pub slot_1_ro: bool,
    pub num_fw_slots: B3,
    pub fw_activate_without_reset: bool,
    #[skip]
    __rsvd: B3,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Lpa {
    pub smart_ns_specific: bool,
    pub cmd_supp_eff: bool,
    pub ext_get_log: bool,
    pub telem: bool,
    pub persist_event: bool,
    #[skip]
    __rsvd: B3,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Avscc {
    pub vndr_standard: bool,
    #[skip]
    __rsvd: B7,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Apsta {
    pub auto_pwr_transition: bool,
    #[skip]
    __rsvd: B7,
}

#[bitfield]
#[repr(u32)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u32", from = "u32"))]
pub struct Rpmbs {
    pub num_rpmb_units: B3,
    pub auth_method: B3,
    #[skip]
    __rsvd: B10,
    pub total_size: B8,
    pub access_size: B8,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Dsto {
    pub sync_self_test: bool,
    #[skip]
    __rsvd: B7,
}

#[bitfield]
#[repr(u16)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u16", from = "u16"))]
pub struct Hctma {
    pub host_thermal_mgmt: bool,
    #[skip]
    __rsvd: B15,
}

#[bitfield]
#[repr(u32)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u32", from = "u32"))]
pub struct Sanicap {
    pub ces: bool,
    pub bes: bool,
    pub ows: bool,
    #[skip]
    __rsvd: B26,
    pub ndi: bool,
    pub nodmmas: Nodmmas,
}

#[derive(BitfieldSpecifier)]
pub enum Nodmmas {
    Undefined = 0,
    NotModified = 1,
    Modified = 2,
    _Reserved = 3,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Anacap {
    pub report_optimized: bool,
    pub report_non_optimized: bool,
    pub report_inaccessible: bool,
    pub report_persistent_loss: bool,
    pub report_change: bool,
    #[skip]
    __rsvd: B1,
    pub anagrpid_static: bool,
    pub anagrpid_supported: bool,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct QueueEntrySize {
    pub min: B4,
    pub max: B4,
}

#[bitfield]
#[repr(u16)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u16", from = "u16"))]
pub struct Oncs {
    pub compare: bool,
    pub write_uncorrectable: bool,
    pub dataset_mgmt: bool,
    pub write_zeros: bool,
    pub save_select: bool,
    pub reservations: bool,
    pub timestamp: bool,
    pub verify: bool,
    #[skip]
    __rsvd: B8,
}

#[bitfield]
#[repr(u16)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u16", from = "u16"))]
pub struct Fuses {
    pub comp_write: bool,
    #[skip]
    __rsvd: B15,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Fna {
    pub format_all: bool,
    pub secure_erase_all: bool,
    pub crypto_erase: bool,
    #[skip]
    __rsvd: B5,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Nvscc {
    pub vndr_standard: bool,
    #[skip]
    __rsvd: B7,
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", from = "u8"))]
pub struct Nwpc {
    pub supported: bool,
    pub until_pwr_cycle: bool,
    pub permanent: bool,
    #[skip]
    __rsvd: B5,
}

#[bitfield]
#[repr(u32)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u32", from = "u32"))]
pub struct Sgls {
    pub support: SglsType,
    pub keyed: bool,
    #[skip]
    __rsvd: B13,
    pub bit_bucket: bool,
    pub meta_byte_aligned_contigious: bool,
    pub oversized_buffer: bool,
    pub meta_singular_sgl: bool,
    pub addr_field: bool,
    pub transport_descriptor: bool,
    #[skip]
    __rsvd: B10,
}

#[derive(BitfieldSpecifier)]
pub enum SglsType {
    NotSupported = 0,
    Supported = 1,
    SupportedAligned = 2,
    _Reserved = 3,
}

#[bitfield]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(into = "PowerStateUnpacked", from = "PowerStateUnpacked")
)]
pub struct PowerState {
    pub mp: u16,
    #[skip]
    __rsvd: u8,
    pub mxps: bool,
    pub nops: bool,
    #[skip]
    __rsvd: B6,
    pub enlat: u32,
    pub exlat: u32,
    pub rrt: B5,
    #[skip]
    __rsvd: B3,
    pub rrl: B5,
    #[skip]
    __rsvd: B3,
    pub rwt: B5,
    #[skip]
    __rsvd: B3,
    pub rwl: B5,
    #[skip]
    __rsvd: B3,
    pub idlp: u16,
    #[skip]
    __rsvd: B6,
    pub ips: B2,
    #[skip]
    __rsvd: B8,
    pub actp: u16,
    pub apw: B3,
    #[skip]
    __rsvd: B3,
    pub aps: B2,
    #[skip]
    __rsvd: B64,
    #[skip]
    __rsvd: B8,
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct PowerStateUnpacked {
    pub mp: u16,
    pub mxps: bool,
    pub nops: bool,
    pub enlat: u32,
    pub exlat: u32,
    pub rrt: u8,
    pub rrl: u8,
    pub rwt: u8,
    pub idlp: u16,
    pub ips: u8,
    pub actp: u16,
    pub apw: u8,
    pub aps: u8,
}

#[cfg(feature = "serde")]
impl From<PowerStateUnpacked> for PowerState {
    fn from(unpacked: PowerStateUnpacked) -> Self {
        PowerState::new()
            .with_mp(unpacked.mp)
            .with_mxps(unpacked.mxps)
            .with_nops(unpacked.nops)
            .with_enlat(unpacked.enlat)
            .with_exlat(unpacked.exlat)
            .with_rrt(unpacked.rrt)
            .with_rrl(unpacked.rrl)
            .with_rwt(unpacked.rwt)
            .with_idlp(unpacked.idlp)
            .with_ips(unpacked.ips)
            .with_actp(unpacked.actp)
            .with_apw(unpacked.apw)
            .with_aps(unpacked.aps)
    }
}

#[cfg(feature = "serde")]
impl From<PowerState> for PowerStateUnpacked {
    fn from(packed: PowerState) -> Self {
        PowerStateUnpacked {
            mp: packed.mp(),
            mxps: packed.mxps(),
            nops: packed.nops(),
            enlat: packed.enlat(),
            exlat: packed.exlat(),
            rrt: packed.rrt(),
            rrl: packed.rrl(),
            rwt: packed.rwt(),
            idlp: packed.idlp(),
            ips: packed.ips(),
            actp: packed.actp(),
            apw: packed.apw(),
            aps: packed.aps(),
        }
    }
}
