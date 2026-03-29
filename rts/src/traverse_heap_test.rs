use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::storage::block::bdescr;
use crate::ffi::rts::storage::closure_types::{ARR_WORDS, THUNK_SELECTOR, WEAK};
use crate::ffi::rts::storage::closures::{
    _StgWeak, C2RustUnnamed, StgArrBytes, StgClosure_, StgHeader, StgProfHeader, StgSMPThunkHeader,
    StgSelector, StgThunkHeader, StgWeak,
};
use crate::ffi::rts::storage::info_tables::{
    C2RustUnnamed_0, StgClosureInfo, StgInfoTable_, StgProfInfo,
};
use crate::ffi::rts::types::{StgClosure, StgInfoTable};
use crate::ffi::stg::types::StgHalfWord;
use crate::prelude::*;
use crate::traverse_heap::{
    closeTraverseStack, initializeTraverseStack, nextPos, nullStackData, posTypeStep, stackAccum,
    stackAccum_, stackData, stackData_, stackElement, stackElement_, stackPos_,
    traverseInvalidateClosureData, traversePushClosure, traverseState, traverseState_,
    traverseWorkStack,
};

/// cbindgen:no-export
struct node {
    id: u32,
    u: node_union,
}

union node_union {
    cls: StgClosure,
    weak: StgWeak,
    selector: StgSelector,
    arrbytes: StgArrBytes,
}

static mut info_weak: StgInfoTable = StgInfoTable_ {
    prof: StgProfInfo {
        closure_type_off: 0,
        closure_desc_off: 0,
    },
    layout: StgClosureInfo {
        payload: C2RustUnnamed_0 { ptrs: 0, nptrs: 0 },
    },
    r#type: WEAK as StgHalfWord,
    srt: 0,
    code: [],
};

static mut info_selector: StgInfoTable = StgInfoTable_ {
    prof: StgProfInfo {
        closure_type_off: 0,
        closure_desc_off: 0,
    },
    layout: StgClosureInfo {
        payload: C2RustUnnamed_0 { ptrs: 0, nptrs: 0 },
    },
    r#type: THUNK_SELECTOR as StgHalfWord,
    srt: 0,
    code: [],
};

static mut info_arrwords: StgInfoTable = StgInfoTable_ {
    prof: StgProfInfo {
        closure_type_off: 0,
        closure_desc_off: 0,
    },
    layout: StgClosureInfo {
        payload: C2RustUnnamed_0 { ptrs: 0, nptrs: 0 },
    },
    r#type: ARR_WORDS as StgHalfWord,
    srt: 0,
    code: [],
};

static mut n1003: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n1002: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n1001: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n1000: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n1103: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n1102: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n1101: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n1100: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n2006: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n2007: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n2008: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n2005: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n2004: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n2003: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n2002: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n2001: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

static mut n2000: node = node {
    id: 0,
    u: node_union {
        cls: StgClosure_ {
            header: StgHeader {
                info: null::<StgInfoTable>(),
                prof: StgProfHeader {
                    ccs: null_mut::<CostCentreStack>(),
                    hp: C2RustUnnamed { trav: 0 },
                },
            },
            payload: [],
        },
    },
};

unsafe fn testReturn(
    mut c: *mut StgClosure,
    acc: stackAccum,
    mut c_parent: *mut StgClosure,
    mut acc_parent: *mut stackAccum,
) {
    let mut n = ({
        let mut __mptr: *const StgClosure = c;
        (__mptr as *mut c_char).offset(-8) as *mut node
    });

    printf(c"return %u\n".as_ptr(), (*n).id);
}

unsafe fn testVisit(
    mut c: *mut StgClosure,
    mut cp: *const StgClosure,
    data: stackData,
    first_visit: bool,
    mut acc: *mut stackAccum,
    mut child_data: *mut stackData,
) -> bool {
    let mut n = ({
        let mut __mptr: *const StgClosure = c;
        (__mptr as *mut c_char).offset(-8) as *mut node
    });

    printf(c"visit  %u\n".as_ptr(), (*n).id);

    return first_visit;
}

static mut g_tests: [*mut node; 3] = unsafe {
    [
        &raw const n1000 as *mut node,
        &raw const n1100 as *mut node,
        &raw const n2000 as *mut node,
    ]
};

static mut state: traverseState = traverseState_ {
    flip: 0,
    firstStack: null_mut::<bdescr>(),
    currentStack: null_mut::<bdescr>(),
    stackBottom: null_mut::<stackElement>(),
    stackTop: null_mut::<stackElement>(),
    stackLimit: null_mut::<stackElement>(),
    stackSize: 0,
    maxStackSize: 0,
    return_cb: None,
};

unsafe fn traverseHeapRunTests() {
    let mut ts: *mut traverseState = &raw mut state;
    printf(c"with return\n".as_ptr());

    state.return_cb = Some(
        testReturn
            as unsafe extern "C" fn(
                *mut StgClosure,
                stackAccum,
                *mut StgClosure,
                *mut stackAccum,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut StgClosure,
                stackAccum,
                *mut StgClosure,
                *mut stackAccum,
            ) -> (),
        >;

    initializeTraverseStack(ts);
    traverseInvalidateClosureData(ts);

    let mut i: usize = 0;

    while i < (size_of::<[*mut node; 3]>() as usize).wrapping_div(size_of::<*mut node>() as usize) {
        let mut n = g_tests[i as usize];

        let mut se = stackElement_ {
            info: stackPos_ {
                r#type: posTypeStep,
                next: nextPos { step: 0 },
            },
            c: null_mut::<StgClosure>(),
            sep: null_mut::<stackElement_>(),
            data: stackData_ {
                c_child_r: null_mut::<CostCentreStack>(),
            },
            accum: stackAccum_ { subtree_sizeW: 0 },
        };

        memset(
            &raw mut se as *mut c_void,
            0,
            size_of::<stackElement>() as usize,
        );
        printf(c"\n\npush   %u\n".as_ptr(), (*n).id);

        traversePushClosure(
            ts,
            &raw mut (*n).u.cls,
            &raw mut (*n).u.cls,
            &raw mut se,
            nullStackData,
        );

        traverseWorkStack(
            ts,
            Some(
                testVisit
                    as unsafe extern "C" fn(
                        *mut StgClosure,
                        *const StgClosure,
                        stackData,
                        bool,
                        *mut stackAccum,
                        *mut stackData,
                    ) -> bool,
            ),
        );

        i = i.wrapping_add(1);
    }

    closeTraverseStack(ts);
    printf(c"\n\n\n\njust visit\n".as_ptr());
    state.return_cb = None;
    initializeTraverseStack(ts);
    traverseInvalidateClosureData(ts);

    let mut i_0: usize = 0;

    while i_0 < (size_of::<[*mut node; 3]>() as usize).wrapping_div(size_of::<*mut node>() as usize)
    {
        let mut n_0 = g_tests[i_0 as usize];
        printf(c"\n\npush   %u\n".as_ptr(), (*n_0).id);

        traversePushClosure(
            ts,
            &raw mut (*n_0).u.cls,
            &raw mut (*n_0).u.cls,
            null_mut::<stackElement>(),
            nullStackData,
        );

        traverseWorkStack(
            ts,
            Some(
                testVisit
                    as unsafe extern "C" fn(
                        *mut StgClosure,
                        *const StgClosure,
                        stackData,
                        bool,
                        *mut stackAccum,
                        *mut stackData,
                    ) -> bool,
            ),
        );

        i_0 = i_0.wrapping_add(1);
    }

    closeTraverseStack(ts);
}

unsafe fn run_static_initializers() {
    n1003 = node {
        id: 1003,
        u: node_union {
            arrbytes: StgArrBytes {
                header: StgHeader {
                    info: (&raw mut info_arrwords).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                bytes: 0,
                payload: [],
            },
        },
    };

    n1002 = node {
        id: 1002,
        u: node_union {
            arrbytes: StgArrBytes {
                header: StgHeader {
                    info: (&raw mut info_arrwords).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                bytes: 0,
                payload: [],
            },
        },
    };

    n1001 = node {
        id: 1001,
        u: node_union {
            arrbytes: StgArrBytes {
                header: StgHeader {
                    info: (&raw mut info_arrwords).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                bytes: 0,
                payload: [],
            },
        },
    };

    n1000 = node {
        id: 1000,
        u: node_union {
            weak: _StgWeak {
                header: StgHeader {
                    info: (&raw mut info_weak).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                cfinalizers: null_mut::<StgClosure>(),
                key: &raw mut n1001.u as *mut StgClosure,
                value: &raw mut n1002.u as *mut StgClosure,
                finalizer: &raw mut n1003.u as *mut StgClosure,
                link: null_mut::<_StgWeak>(),
            },
        },
    };

    n1103 = node {
        id: 1103,
        u: node_union {
            arrbytes: StgArrBytes {
                header: StgHeader {
                    info: (&raw mut info_arrwords).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                bytes: 0,
                payload: [],
            },
        },
    };

    n1102 = node {
        id: 1102,
        u: node_union {
            arrbytes: StgArrBytes {
                header: StgHeader {
                    info: (&raw mut info_arrwords).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                bytes: 0,
                payload: [],
            },
        },
    };

    n1101 = node {
        id: 1101,
        u: node_union {
            selector: StgSelector {
                header: StgThunkHeader {
                    info: (&raw mut info_selector).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                    smp: StgSMPThunkHeader { pad: 0 },
                },
                selectee: &raw mut n1102.u as *mut StgClosure,
            },
        },
    };

    n1100 = node {
        id: 1100,
        u: node_union {
            weak: _StgWeak {
                header: StgHeader {
                    info: (&raw mut info_weak).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                cfinalizers: null_mut::<StgClosure>(),
                key: &raw mut n1101.u as *mut StgClosure,
                value: &raw mut n1102.u as *mut StgClosure,
                finalizer: &raw mut n1103.u as *mut StgClosure,
                link: null_mut::<_StgWeak>(),
            },
        },
    };

    n2006 = node {
        id: 2006,
        u: node_union {
            arrbytes: StgArrBytes {
                header: StgHeader {
                    info: (&raw mut info_arrwords).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                bytes: 0,
                payload: [],
            },
        },
    };

    n2007 = node {
        id: 2007,
        u: node_union {
            arrbytes: StgArrBytes {
                header: StgHeader {
                    info: (&raw mut info_arrwords).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                bytes: 0,
                payload: [],
            },
        },
    };

    n2008 = node {
        id: 2008,
        u: node_union {
            arrbytes: StgArrBytes {
                header: StgHeader {
                    info: (&raw mut info_arrwords).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                bytes: 0,
                payload: [],
            },
        },
    };

    n2005 = node {
        id: 2005,
        u: node_union {
            weak: _StgWeak {
                header: StgHeader {
                    info: (&raw mut info_weak).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                cfinalizers: null_mut::<StgClosure>(),
                key: &raw mut n2006.u as *mut StgClosure,
                value: &raw mut n2007.u as *mut StgClosure,
                finalizer: &raw mut n2008.u as *mut StgClosure,
                link: null_mut::<_StgWeak>(),
            },
        },
    };

    n2004 = node {
        id: 2004,
        u: node_union {
            selector: StgSelector {
                header: StgThunkHeader {
                    info: (&raw mut info_selector).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                    smp: StgSMPThunkHeader { pad: 0 },
                },
                selectee: &raw mut n2005.u as *mut StgClosure,
            },
        },
    };

    n2003 = node {
        id: 2003,
        u: node_union {
            arrbytes: StgArrBytes {
                header: StgHeader {
                    info: (&raw mut info_arrwords).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                bytes: 0,
                payload: [],
            },
        },
    };

    n2002 = node {
        id: 2002,
        u: node_union {
            selector: StgSelector {
                header: StgThunkHeader {
                    info: (&raw mut info_selector).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                    smp: StgSMPThunkHeader { pad: 0 },
                },
                selectee: &raw mut n2004.u as *mut StgClosure,
            },
        },
    };

    n2001 = node {
        id: 2001,
        u: node_union {
            arrbytes: StgArrBytes {
                header: StgHeader {
                    info: (&raw mut info_arrwords).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                bytes: 0,
                payload: [],
            },
        },
    };

    n2000 = node {
        id: 2000,
        u: node_union {
            weak: _StgWeak {
                header: StgHeader {
                    info: (&raw mut info_weak).offset(1),
                    prof: StgProfHeader {
                        ccs: null_mut::<CostCentreStack>(),
                        hp: C2RustUnnamed { trav: 0 },
                    },
                },
                cfinalizers: null_mut::<StgClosure>(),
                key: &raw mut n2001.u as *mut StgClosure,
                value: &raw mut n2002.u as *mut StgClosure,
                finalizer: &raw mut n2003.u as *mut StgClosure,
                link: null_mut::<_StgWeak>(),
            },
        },
    };
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
