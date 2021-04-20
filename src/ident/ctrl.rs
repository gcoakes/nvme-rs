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

#[repr(packed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdCtrl {
    pub vid: u16,
    pub ssvid: u16,
    pub sn: FixedStr<20>,
    pub mn: FixedStr<40>,
    pub fr: FixedStr<8>,
    pub rab: u8,
    pub ieee: Ieee,
    pub cmic: Cmic,
    pub mdts: u8,
    pub cntlid: u16,
    pub ver: u32,
    pub rtd3r: u32,
    pub rtd3e: u32,
    pub oaes: Oaes,
    pub ctrattr: CtrlAttr,
    pub rrls: BitArray<2>,
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd102: Reserved<9>,
    pub cntrltype: CtrlType,
    // TODO: Assess whether this has any value in destructured form. It appears
    // to simply be a unique identifier for the type of device.
    /// FRU Globally Unique Identifier (Big Endian)
    pub fguid: u128,
    pub crdt: [u16; 3],
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd134: Reserved<106>,
    // Separately defined reserved because the second one is from the NVMe-MI
    // spec.
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd240: Reserved<13>,
    pub nvmsr: Nvmsr,
    pub vwci: Vwci,
    pub mec: Mec,
    pub oacs: Oacs,
    pub acl: u8,
    pub aerl: u8,
    pub frmw: Frmw,
    pub lpa: Lpa,
    pub elpe: u8,
    pub npss: u8,
    pub avscc: Avscc,
    pub apsta: Apsta,
    pub wctemp: u16,
    pub cctemp: u16,
    pub mtfa: u16,
    pub hmpre: u32,
    pub hmmin: u32,
    pub tnvmcap: u128,
    pub unvmcap: u128,
    pub rpmbs: Rpmbs,
    pub edst: u16,
    pub dsto: Dsto,
    pub fwug: u8,
    pub kas: u16,
    pub hctma: Hctma,
    pub mntmt: u16,
    pub mxtmt: u16,
    pub sanicap: Sanicap,
    pub hmminds: u32,
    pub hmmaxd: u16,
    pub nsetidmax: u16,
    pub endgidmax: u16,
    pub anatt: u8,
    pub anacap: Anacap,
    pub anagrpmax: u32,
    pub nanagrpid: u32,
    pub pels: u32,
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd356: Reserved<156>,
    pub sqes: QueueEntrySize,
    pub cqes: QueueEntrySize,
    pub maxcmd: u16,
    pub nn: u32,
    pub oncs: Oncs,
    pub fuses: Fuses,
    pub fna: Fna,
    pub vwc: u8,
    pub awun: u16,
    pub awupf: u16,
    pub nvscc: Nvscc,
    pub nwpc: Nwpc,
    pub acwu: u16,
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd534: Reserved<2>,
    pub sgls: Sgls,
    pub mnan: u32,
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd544: Reserved<224>,
    // TODO: Properly handle null termination.
    pub subnqn: FixedStr<256>,
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd1024: Reserved<768>,
    // TODO: NVMe over Fabrics.
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd1792: Reserved<256>,

    pub psds: [PowerState; 32],
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

#[test]
fn structure() {
    use memoffset::offset_of;
    use std::mem::size_of;
    assert_eq!(offset_of!(IdCtrl, ver), 80);
    assert_eq!(size_of::<IdCtrl>(), 4096);
}
