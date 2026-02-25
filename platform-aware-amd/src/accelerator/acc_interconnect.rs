use platform_aware_features::{AccInterconnect, create_feature_hierarchy};

//
// AMD (GPU/accelerator interconnects):
//   - InfinityFabric: AMD’s umbrella interconnect technology.
//   - XGMI: “External Global Memory Interconnect”, AMD’s high-speed GPU↔GPU interconnect
//           based on Infinity Fabric (common on Instinct MI200/MI300 nodes).
//   - InfinityFabricLink: the branded GPU↔GPU bridge used on some Radeon Pro configurations
//                         (e.g., Mac Pro MPX modules).
//   - AFL (Accelerated Fabric Link): AMD’s announced/positioned scale-up interconnect concept
//                                    aimed at NVLink-class scale-up in future systems.
//
// Notes on “versions”:
//   NVIDIA names NVLink V1..V5 and NVSwitch G1..G4.
//   For AMD, public docs commonly use “Infinity Fabric” and “xGMI” as names, but do NOT
//   consistently publish a simple “xGMI v1/v2/v3” scheme the way NVLink does.
//   So below we keep AMD’s public naming and (optionally) model “generations” where the
//   name is actually used publicly (AFL).
//

// ------------------------------------------------------------------
// (1) Infinity Fabric umbrella + GPU-to-GPU links
// ------------------------------------------------------------------

create_feature_hierarchy! { register_amd_interconnect_1 ;
    "acc_interconnect" :
    AccInterconnect :>
    InfinityFabric :>
    XGMI;
    // XGMI is AMD’s GPU-to-GPU interconnect based on Infinity Fabric.
    // (No stable public “XGMIV1..Vn” naming across AMD docs.)
}

// Optional sibling under InfinityFabric: Infinity Fabric Link (GPU bridge board/bridge)
// If you prefer, you can make it a child of AccInterconnect directly; semantically it’s
// a branded physical implementation/bridge for GPU↔GPU.
create_feature_hierarchy! { register_amd_interconnect_2 ;
    "acc_interconnect" :
    AccInterconnect :>
    InfinityFabricLink;
}

// ------------------------------------------------------------------
// (2) Scale-up switching / fabric concept (AFL)
// ------------------------------------------------------------------

create_feature_hierarchy! { register_amd_interconnect_3 ;
    "acc_interconnect" :
    AccInterconnect :>
    AFL;
    // AFL = Accelerated Fabric Link (Infinity Fabric scale-up concept).
}

// If you want to mirror the NVSwitch hierarchy, you can define an AFL "switch fabric" node.
// Public material discusses AFL arriving via switch silicon ecosystem; exact “G1/G2”
// style names aren’t standardized publicly, so keep it minimal.
create_feature_hierarchy! { register_amd_interconnect_4 ;
    "acc_interconnect" :
    AccInterconnect :>
    AFLSwitch;
}