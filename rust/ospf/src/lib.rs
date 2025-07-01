#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OspfHelperExitReason {
    None = 0,
    InProgress,
    TopologyChange,
    GraceTimeout,
    Completed,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OspfGrRestartReason {
    UnknownRestart = 0,
    SoftwareRestart = 1,
    SoftwareUpgrade = 2,
    SwitchRedundantCard = 3,
    InvalidReasonCode = 4,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OspfGrHelperRejectedReason {
    None = 0,
    SupportDisabled,
    NotAValidNeighbour,
    PlannedOnlyRestart,
    TopologyChangeRtxmtList,
    LsaAgeMore,
    Restarting,
}

pub fn ospf_exit_reason2str(reason: OspfHelperExitReason) -> &'static str {
    match reason {
        OspfHelperExitReason::None => "Unknown reason",
        OspfHelperExitReason::InProgress => "Helper in progress",
        OspfHelperExitReason::TopologyChange => "Topology Change",
        OspfHelperExitReason::GraceTimeout => "Grace timer expiry",
        OspfHelperExitReason::Completed => "Successful graceful restart",
    }
}

pub fn ospf_restart_reason2str(reason: OspfGrRestartReason) -> &'static str {
    match reason {
        OspfGrRestartReason::UnknownRestart => "Unknown restart",
        OspfGrRestartReason::SoftwareRestart => "Software restart",
        OspfGrRestartReason::SoftwareUpgrade => "Software reload/upgrade",
        OspfGrRestartReason::SwitchRedundantCard => "Switch to redundant control processor",
        OspfGrRestartReason::InvalidReasonCode => "Invalid reason",
    }
}

pub fn ospf_rejected_reason2str(reason: OspfGrHelperRejectedReason) -> &'static str {
    match reason {
        OspfGrHelperRejectedReason::None => "Unknown reason",
        OspfGrHelperRejectedReason::SupportDisabled => "Helper support disabled",
        OspfGrHelperRejectedReason::NotAValidNeighbour => "Neighbour is not in FULL state",
        OspfGrHelperRejectedReason::PlannedOnlyRestart => "Supports only planned restart but received unplanned",
        OspfGrHelperRejectedReason::TopologyChangeRtxmtList => "Topo change due to change in lsa rxmt list",
        OspfGrHelperRejectedReason::LsaAgeMore => "LSA age is more than Grace interval",
        OspfGrHelperRejectedReason::Restarting => "Router is in the process of graceful restart",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reason_strings() {
        assert_eq!(ospf_exit_reason2str(OspfHelperExitReason::None), "Unknown reason");
        assert_eq!(ospf_restart_reason2str(OspfGrRestartReason::SoftwareRestart), "Software restart");
        assert_eq!(ospf_rejected_reason2str(OspfGrHelperRejectedReason::Restarting), "Router is in the process of graceful restart");
    }
}

