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

use crate::{Reserved, TransmuteSafe};

use modular_bitfield::prelude::*;

#[test_structure(size = 512)]
#[repr(C, packed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SmartLog {
    #[loc(0:0)]
    pub crit_warning: u8,
    #[loc(1:2)]
    pub comp_temp: u16,
    #[loc(3:3)]
    pub avail_spare: u8,
    #[loc(4:4)]
    pub avail_spare_thresh: u8,
    #[loc(5:5)]
    pub percent_used: u8,
    #[loc(6:6)]
    pub endur_grp_crit_warning: u8,
    #[loc(7:31)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd7: Reserved<25>,
    #[loc(32:47)]
    pub data_units_read: u128,
    #[loc(48:63)]
    pub data_units_written: u128,
    #[loc(64:79)]
    pub host_read_cmds: u128,
    #[loc(80:95)]
    pub host_write_cmds: u128,
    #[loc(96:111)]
    pub ctrl_busy_time: u128,
    #[loc(112:127)]
    pub pwr_cycles: u128,
    #[loc(128:143)]
    pub pwr_on_hrs: u128,
    #[loc(144:159)]
    pub unsafe_shutdowns: u128,
    #[loc(160:175)]
    pub mad_integrity_errs: u128,
    #[loc(176:191)]
    pub num_err_log_entries: u128,
    #[loc(192:195)]
    pub warning_comp_temp_time: u32,
    #[loc(196:199)]
    pub crit_comp_temp_time: u32,
    #[loc(200:215)]
    pub temp_sensors: [u16; 8],
    #[loc(216:223)]
    pub therm_mgmt_temp_transition_cnts: [u32; 2],
    #[loc(224:231)]
    pub total_time_therm_mgmt_temp: [u32; 2],
    #[loc(232:511)]
    #[cfg_attr(feature = "serde", serde(skip))]
    __rsvd232: Reserved<280>,
}

impl TransmuteSafe for SmartLog {}

impl SmartLog {
    pub fn has_critical_warning(&self) -> bool {
        self.crit_warning > 0
    }
    pub fn crit_warning(&self) -> CriticalWarning {
        CriticalWarning::from_bytes([self.crit_warning])
    }
    pub fn has_endur_grp_crit_warning(&self) -> bool {
        self.endur_grp_crit_warning > 0
    }
    pub fn endur_grp_crit_warning(&self) -> EndurGrpCritWarning {
        EndurGrpCritWarning::from_bytes([self.endur_grp_crit_warning])
    }
}

#[bitfield]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CriticalWarning {
    pub spare_cap_below_thresh: bool,
    pub temp_out_of_thresh: bool,
    pub nvm_subsys_degraded: bool,
    pub readonly: bool,
    pub volatile_mem_backup_failed: bool,
    pub pmr_readonly: bool,
    #[skip]
    __: B2,
}

#[bitfield]
pub struct EndurGrpCritWarning {
    pub spare_cap_below_thresh: bool,
    #[skip]
    __: B1,
    pub nvm_subsys_degraded: bool,
    pub readonly: bool,
    #[skip]
    __: B4,
}
