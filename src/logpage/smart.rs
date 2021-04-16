/*
 * Copyright (C) 2021  Oakes, Gregory C. <gregcoakes@gmail.com>
 * Author: Oakes, Gregory C. <gregcoakes@gmail.com>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation, either
 * version 3 of the License, or (at your option) any later version.
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

use modular_bitfield::prelude::*;

#[repr(packed)]
pub struct SmartLog {
    pub crit_warning: u8,
    pub comp_temp: u16,
    pub avail_spare: u8,
    pub avail_spare_thresh: u8,
    pub percent_used: u8,
    pub endur_grp_crit_warning: u8,
    __rsvd7: [u8; 25],
    pub data_units_read: u128,
    pub data_units_written: u128,
    pub host_read_commands: u128,
    pub host_write_commands: u128,
    pub ctrl_busy_time: u128,
    pub pwr_cycles: u128,
    pub pwr_on_hrs: u128,
    pub unsafe_shutdowns: u128,
    pub mad_integrity_errs: u128,
    pub num_err_log_entries: u128,
    pub warning_comp_temp_time: u32,
    pub crit_comp_temp_time: u32,
    pub temp_sensors: [u16; 8],
    pub therm_mgmt_temp_1_transition_cnts: u32,
    pub therm_mgmt_temp_2_transition_cnts: u32,
    pub total_time_therm_mgmt_temp_1: u32,
    pub total_time_therm_mgmt_temp_2: u32,
    __rsvd232: [u8; 280],
}

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

#[test]
fn structure() {
    use std::mem;
    assert_eq!(mem::size_of::<SmartLog>(), 512);
}
