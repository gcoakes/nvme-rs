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
use num_traits::FromPrimitive;

/// This includes the phase tag which is inconsistent with the specification. This is done so it is
/// aligned properly.
#[bitfield(bits = 16)]
#[derive(BitfieldSpecifier, Debug)]
pub struct StatusField {
    phase_tag: bool,
    sc: u8,
    sct: StatusCodeType,
    crd: B2,
    more: bool,
    dnr: bool,
}

impl StatusField {
    pub fn status_code(&self) -> StatusCode {
        let sc = self.sc();
        match self.sct() {
            StatusCodeType::Generic => FromPrimitive::from_u8(sc).map(StatusCode::Generic),
            StatusCodeType::CmdSpecific => FromPrimitive::from_u8(sc).map(StatusCode::CmdSpecific),
            StatusCodeType::MediaAndDataIntegrity => {
                FromPrimitive::from_u8(sc).map(StatusCode::MediaAndDataIntegrity)
            }
            StatusCodeType::PathRelated => FromPrimitive::from_u8(sc).map(StatusCode::PathRelated),
            StatusCodeType::VendorSpecific => Some(StatusCode::VendorSpecific(sc)),
            _ => Some(StatusCode::Unknown(sc)),
        }
        .unwrap_or(StatusCode::Unknown(sc))
    }

    pub fn successful(&self) -> bool {
        self.sct() == StatusCodeType::Generic && self.sc() == 0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StatusCode {
    Generic(GenericStatus),
    CmdSpecific(CmdSpecificStatus),
    MediaAndDataIntegrity(MediaAndDataIntegrityStatus),
    PathRelated(PathRelatedStatus),
    VendorSpecific(u8),
    Unknown(u8),
}

#[derive(BitfieldSpecifier, Debug, PartialEq, Eq)]
#[bits = 3]
pub enum StatusCodeType {
    Generic = 0,
    CmdSpecific = 1,
    MediaAndDataIntegrity = 2,
    PathRelated = 3,

    // The enum must exhaust the bits or else it can't reliably be transmuted.
    #[doc(hidden)]
    _Reserved4 = 4,
    #[doc(hidden)]
    _Reserved5 = 5,
    #[doc(hidden)]
    _Reserved6 = 6,

    VendorSpecific = 7,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, num_derive::FromPrimitive)]
pub enum GenericStatus {
    Success = 0x00,
    InvalidCmdOpcode = 0x01,
    InvalidFieldInCmd = 0x02,
    CmdIdConflict = 0x03,
    DataTransferErr = 0x04,
    CmdAbortedDueToPowerLossNotification = 0x05,
    InternalErr = 0x06,
    CmdAbortRequested = 0x07,
    CmdAbortedDueToSqDeletion = 0x08,
    CmdAbortedDueToFailedFusedCmd = 0x09,
    CmdAbortedDueToMissingFusedCmd = 0x0a,
    InvalidNmspOrFormat = 0x0b,
    CmdSequenceErr = 0x0c,
    InvalidSglSegmentDescriptor = 0x0d,
    InvalidNumberOfSglDescriptors = 0x0e,
    DataSglLengthInvalid = 0x0f,
    MetadataSglLengthInvalid = 0x10,
    SglDescriptorTypeInvalid = 0x11,
    InvalidUseOfCtrlMemoryBuffer = 0x12,
    PrpOffsetInvalid = 0x13,
    AtomicWriteUnitExceeded = 0x14,
    OperationDenied = 0x15,
    SglOffsetInvalid = 0x16,

    HostIdInconsistentFormat = 0x18,
    KeepAliveTimerExpired = 0x19,
    KeepAliveTimeoutInvalid = 0x1a,
    CmdAbortedDueToPreemptAndAbort = 0x1b,
    SanitizeFailed = 0x1c,
    SanitizeInProgress = 0x1d,
    SglDataBlockGranularityInvalid = 0x1e,
    CmdNotSupportedForQueueInCmb = 0x1f,
    NmspIsWriteProtected = 0x20,
    CmdInterrupted = 0x21,
    TransientTransportErr = 0x22,

    LbaOutOfRange = 0x80,
    CapacityExceeded = 0x81,
    NmspNotReady = 0x82,
    ReservationConflict = 0x83,
    FormatInProgress = 0x84,
}

impl Copy for GenericStatus {}

#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, num_derive::FromPrimitive)]
pub enum CmdSpecificStatus {
    CompletionQueueInvalid = 0x00,
    InvalidQueueId = 0x01,
    InvalidQueueSize = 0x02,
    AbortCmdLimitExceeded = 0x03,

    AsyncEventReqLimitExceeded = 0x05,
    InvalidFwSlot = 0x06,
    InvalidFwImage = 0x07,
    InvalidInterruptVector = 0x08,
    InvalidLogPage = 0x09,
    InvalidFormat = 0x0a,
    FwActivationRequiresConventionalReset = 0x0b,
    InvalidQueueDeletion = 0x0c,
    FeatureIdNotSaveable = 0x0d,
    FeatureNotChangeable = 0x0e,
    FeatureNotNamespaceSpecific = 0x0f,
    FwActivationRequiresNvmSubsystemReset = 0x10,
    FwActivationRequiresCtrlLevelReset = 0x11,
    FwActivationRequiresMaximumTimeViolation = 0x12,
    FwActivationProhibited = 0x13,
    OverlappingRange = 0x14,
    NmspInsufficientCapacity = 0x15,
    NmspIdUnavailable = 0x16,

    NmspAlreadyAttached = 0x18,
    NmspIsPrivate = 0x19,
    NmspNotAttached = 0x1a,
    ThinProvisioningNotSupported = 0x1b,
    CtrlListInvalid = 0x1c,
    DeviceSelfTestInProgress = 0x1d,
    BootPartitionWriteProhibited = 0x1e,
    InvalidCtrlId = 0x1f,
    InvalidSecondaryCtrlState = 0x20,
    InvalidNumberOfCtrlResources = 0x21,
    InvalidResourceIdentifier = 0x22,
    SanitizeProhibitedWhilePersistentMemoryRegionIsEnabled = 0x23,
    AnaGroupIdInvalid = 0x24,
    AnaAttachFailed = 0x25,

    ConflictingAttrs = 0x80,
    InvalidProtectionInfo = 0x81,
    AttemptedWriteToReadOnlyRange = 0x82,
}

impl Copy for CmdSpecificStatus {}

#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, num_derive::FromPrimitive)]
pub enum MediaAndDataIntegrityStatus {
    WriteFault = 0x80,
    UnrecoveredReadErr = 0x81,
    E2EGuardCheckErr = 0x82,
    E2EApplicationTagCheckErr = 0x83,
    E2ERefTagCheckErr = 0x84,
    CompareFailure = 0x85,
    AccessDenied = 0x86,
    DeallocatedOrUnwrittenLogicalBlock = 0x87,
}

impl Copy for MediaAndDataIntegrityStatus {}

#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, num_derive::FromPrimitive)]
pub enum PathRelatedStatus {
    InternalPathErr = 0x00,
    AsymAccessPersistentLoss = 0x01,
    AsymAccessInaccessible = 0x02,
    AsymAccessTransition = 0x03,

    CtrlPathingErr = 0x60,

    HostPathingErr = 0x70,
    CmdAbortedByHost = 0x71,
}

impl Copy for PathRelatedStatus {}

#[test]
fn test_from_bytes() {
    assert_eq!(
        StatusField::from_bytes([0b0000_0010, 0b0000_0000]).status_code(),
        StatusCode::Generic(GenericStatus::InvalidCmdOpcode)
    );
    assert_eq!(
        StatusField::from_bytes([0b0000_0000, 0b0000_0000]).status_code(),
        StatusCode::Generic(GenericStatus::Success)
    );
    assert!(StatusField::from_bytes([0b0000_0000, 0b0000_0000]).successful());
    assert!(!StatusField::from_bytes([0b0000_0010, 0b0000_0000]).successful());
    assert!(!StatusField::from_bytes([0b0000_0010, 0b0000_0000]).successful());
    assert_eq!(
        StatusField::from_bytes([0b0010_1110, 0b0000_0000]).status_code(),
        StatusCode::Unknown(0x17)
    );
}
