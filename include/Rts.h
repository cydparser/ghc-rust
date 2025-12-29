#pragma once

#include "target.h"
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct StgTSO_ StgTSO;
typedef struct bdescr_ bdescr;

#define HS_BOOL_TRUE 1

#define SIZEOF_HSINT 8

#define ALIGNMENT_HSINT 8

#define SIZEOF_HSWORD 8

#define ALIGNMENT_HSWORD 8

#define SIZEOF_HSDOUBLE 8

#define ALIGNMENT_HSDOUBLE 8

#define SIZEOF_HSFLOAT 4

#define ALIGNMENT_HSFLOAT 4

#define SIZEOF_HSPTR 8

#define ALIGNMENT_HSPTR 8

#define SIZEOF_HSFUNPTR 8

#define ALIGNMENT_HSFUNPTR 8

#define SIZEOF_HSSTABLEPTR 8

#define ALIGNMENT_HSSTABLEPTR 8

#define SIZEOF_INT8 1

#define ALIGNMENT_INT8 1

#define SIZEOF_WORD8 1

#define ALIGNMENT_WORD8 1

#define SIZEOF_INT16 2

#define ALIGNMENT_INT16 2

#define SIZEOF_WORD16 2

#define ALIGNMENT_WORD16 2

#define SIZEOF_INT32 4

#define ALIGNMENT_INT32 4

#define SIZEOF_WORD32 4

#define ALIGNMENT_WORD32 4

#define SIZEOF_INT64 8

#define ALIGNMENT_INT64 8

#define SIZEOF_WORD64 8

#define ALIGNMENT_WORD64 8

#define WORD_SIZE_IN_BITS 64

#define TAG_BITS 3

#define IN_STG_CODE 0

#define _REENTRANT 1

#define EXIT_INTERNAL_ERROR 254

#define RESERVED_STACK_WORDS 21

#define BITMAP_BITS_SHIFT 6

#define ThreadRunGHC 1

#define ThreadInterpret 2

#define ThreadKilled 3

#define ThreadComplete 4

#define NotBlocked 0

#define BlockedOnMVar 1

#define BlockedOnMVarRead 14

#define BlockedOnBlackHole 2

#define BlockedOnRead 3

#define BlockedOnWrite 4

#define BlockedOnDelay 5

#define BlockedOnSTM 6

#define BlockedOnDoProc 7

#define BlockedOnCCall 10

#define BlockedOnCCall_Interruptible 11

#define BlockedOnMsgThrowTo 12

#define ThreadMigrating 13

#define HeapOverflow 1

#define StackOverflow 2

#define ThreadBlocked 4

#define ThreadFinished 5

#define TSO_LOCKED 2

#define TSO_BLOCKEX 4

#define TSO_INTERRUPTIBLE 8

#define TSO_STOPPED_ON_BREAKPOINT 16

#define TSO_MARKED 64

#define TSO_SQUEEZED 128

#define TSO_ALLOC_LIMIT 256

#define TSO_STOP_NEXT_BREAKPOINT 512

#define TSO_STOP_AFTER_RETURN 1024

#define CLOSURE_DESC_BUFFER_SIZE 11

#define NO_GC_STATS 0

#define COLLECT_GC_STATS 1

#define ONELINE_GC_STATS 2

#define SUMMARY_GC_STATS 3

#define VERBOSE_GC_STATS 4

#define COST_CENTRES_NONE 0

#define COST_CENTRES_SUMMARY 1

#define COST_CENTRES_VERBOSE 2

#define COST_CENTRES_ALL 3

#define COST_CENTRES_JSON 4

#define NO_HEAP_PROFILING 0

#define HEAP_BY_CCS 1

#define HEAP_BY_MOD 2

#define HEAP_BY_DESCR 4

#define HEAP_BY_TYPE 5

#define HEAP_BY_RETAINER 6

#define HEAP_BY_LDV 7

#define HEAP_BY_CLOSURE_TYPE 8

#define HEAP_BY_INFO_TABLE 9

#define HEAP_BY_ERA 10

#define TRACE_NONE 0

#define TRACE_EVENTLOG 1

#define TRACE_STDERR 2

#define STG_SIG_DFL -1

#define STG_SIG_IGN -2

#define STG_SIG_ERR -3

#define STG_SIG_HAN -4

#define STG_SIG_RST -5

#define BLOCK_SIZE 4096

#define MBLOCK_SIZE 1048576

#define INVALID_OBJECT 0

#define CONSTR 1

#define CONSTR_1_0 2

#define CONSTR_0_1 3

#define CONSTR_2_0 4

#define CONSTR_1_1 5

#define CONSTR_0_2 6

#define CONSTR_NOCAF 7

#define FUN 8

#define FUN_1_0 9

#define FUN_0_1 10

#define FUN_2_0 11

#define FUN_1_1 12

#define FUN_0_2 13

#define FUN_STATIC 14

#define THUNK 15

#define THUNK_1_0 16

#define THUNK_0_1 17

#define THUNK_2_0 18

#define THUNK_1_1 19

#define THUNK_0_2 20

#define THUNK_STATIC 21

#define THUNK_SELECTOR 22

#define BCO 23

#define AP 24

#define PAP 25

#define AP_STACK 26

#define IND 27

#define IND_STATIC 28

#define RET_BCO 29

#define RET_SMALL 30

#define RET_BIG 31

#define RET_FUN 32

#define UPDATE_FRAME 33

#define CATCH_FRAME 34

#define UNDERFLOW_FRAME 35

#define STOP_FRAME 36

#define BLOCKING_QUEUE 37

#define BLACKHOLE 38

#define MVAR_CLEAN 39

#define MVAR_DIRTY 40

#define TVAR 41

#define ARR_WORDS 42

#define MUT_ARR_PTRS_CLEAN 43

#define MUT_ARR_PTRS_DIRTY 44

#define MUT_ARR_PTRS_FROZEN_DIRTY 45

#define MUT_ARR_PTRS_FROZEN_CLEAN 46

#define MUT_VAR_CLEAN 47

#define MUT_VAR_DIRTY 48

#define WEAK 49

#define PRIM 50

#define MUT_PRIM 51

#define TSO 52

#define STACK 53

#define TREC_CHUNK 54

#define ATOMICALLY_FRAME 55

#define CATCH_RETRY_FRAME 56

#define CATCH_STM_FRAME 57

#define WHITEHOLE 58

#define SMALL_MUT_ARR_PTRS_CLEAN 59

#define SMALL_MUT_ARR_PTRS_DIRTY 60

#define SMALL_MUT_ARR_PTRS_FROZEN_DIRTY 61

#define SMALL_MUT_ARR_PTRS_FROZEN_CLEAN 62

#define COMPACT_NFDATA 63

#define CONTINUATION 64

#define ANN_FRAME 65

#define N_CLOSURE_TYPES 66

#define ARG_GEN 0

#define ARG_GEN_BIG 1

#define ARG_BCO 2

#define ARG_NONE 3

#define ARG_N 4

#define ARG_P 5

#define ARG_F 6

#define ARG_D 7

#define ARG_L 8

#define ARG_V16 9

#define ARG_V32 10

#define ARG_V64 11

#define ARG_NN 12

#define ARG_NP 13

#define ARG_PN 14

#define ARG_PP 15

#define ARG_NNN 16

#define ARG_NNP 17

#define ARG_NPN 18

#define ARG_NPP 19

#define ARG_PNN 20

#define ARG_PNP 21

#define ARG_PPN 22

#define ARG_PPP 23

#define ARG_PPPP 24

#define ARG_PPPPP 25

#define ARG_PPPPPP 26

#define ARG_PPPPPPP 27

#define ARG_PPPPPPPP 28

#define MACHREGS_NO_REGS 0

enum TRecState
#ifdef __cplusplus
    : uint32_t
#endif // __cplusplus
{
  TREC_ACTIVE = 0,
  TREC_CONDEMNED = 1,
  TREC_ABORTED = 2,
  TREC_WAITING = 3,
};
#ifndef __cplusplus
typedef uint32_t TRecState;
#endif // __cplusplus

enum OStatus
#ifdef __cplusplus
    : uint32_t
#endif // __cplusplus
{
  OBJECT_LOADED = 0,
  OBJECT_NEEDED = 1,
  OBJECT_RESOLVED = 2,
  OBJECT_READY = 3,
  OBJECT_UNLOADED = 4,
  OBJECT_DONT_RESOLVE = 5,
  OBJECT_NOT_LOADED = 6,
};
#ifndef __cplusplus
typedef uint32_t OStatus;
#endif // __cplusplus

enum RtsOptsEnabledEnum
#ifdef __cplusplus
    : uint32_t
#endif // __cplusplus
{
  RtsOptsNone = 0,
  RtsOptsIgnore = 1,
  RtsOptsIgnoreAll = 2,
  RtsOptsSafeOnly = 3,
  RtsOptsAll = 4,
};
#ifndef __cplusplus
typedef uint32_t RtsOptsEnabledEnum;
#endif // __cplusplus

enum SchedulerStatus
#ifdef __cplusplus
    : uint32_t
#endif // __cplusplus
{
  NoStatus = 0,
  Success = 1,
  Killed = 2,
  Interrupted = 3,
  HeapExhausted = 4,
  SchedulerStatus_End = 5,
};
#ifndef __cplusplus
typedef uint32_t SchedulerStatus;
#endif // __cplusplus

typedef void *HsStablePtr;

typedef void (*HsFunPtr)(void);

#if defined(SIZEOF_VOID_P_8)
typedef uint64_t StgWord;
#endif

#if defined(SIZEOF_VOID_P_4)
typedef uint32_t StgWord;
#endif

/**
 * A heap or stack pointer.
 */
typedef StgWord *StgPtr;

typedef uint64_t StgWord64;

typedef uint32_t StgHalfWord;

typedef struct StgClosureInfo__bindgen_ty_1 {
  StgHalfWord ptrs;
  StgHalfWord nptrs;
} StgClosureInfo__bindgen_ty_1;

#if defined(SIZEOF_VOID_P_8)
typedef int64_t StgInt;
#endif

#if defined(SIZEOF_VOID_P_4)
typedef int32_t StgInt;
#endif

typedef union StgClosureInfo {
  struct StgClosureInfo__bindgen_ty_1 payload;
  StgWord bitmap;
  StgInt large_bitmap_offset;
  StgWord selector_offset;
} StgClosureInfo;

#if defined(SIZEOF_VOID_P_8)
typedef int32_t StgHalfInt;
#endif

#if defined(SIZEOF_VOID_P_4)
typedef int16_t StgHalfInt;
#endif

typedef StgHalfInt StgSRTField;

typedef uint8_t StgWord8;

typedef StgWord8 StgCode;

typedef struct __IncompleteArrayField_StgCode {
  StgCode _0[0];
} __IncompleteArrayField_StgCode;

typedef struct StgInfoTable_ {
  union StgClosureInfo layout;
  StgHalfWord type_;
  StgSRTField srt;
  struct __IncompleteArrayField_StgCode code;
} StgInfoTable_;

typedef struct StgInfoTable_ StgInfoTable;

typedef struct StgHeader {
  const StgInfoTable *info;
} StgHeader;

typedef struct __IncompleteArrayField_____StgClosure_ {
  struct StgClosure_ *_0[0];
} __IncompleteArrayField_____StgClosure_;

typedef struct StgClosure_ StgClosure;

typedef uint32_t StgWord32;

typedef struct __IncompleteArrayField_StgWord {
  StgWord _0[0];
} __IncompleteArrayField_StgWord;

typedef struct StgStack_ {
  struct StgHeader header;
  StgWord32 stack_size;
  StgWord8 dirty;
  StgWord8 marking;
  StgPtr sp;
  struct __IncompleteArrayField_StgWord stack;
} StgStack_;

typedef uint16_t StgWord16;

typedef struct Message_ Message;

typedef union StgTSOBlockInfo {
  StgClosure *closure;
  StgTSO *prev;
  struct MessageBlackHole_ *bh;
  struct MessageThrowTo_ *throwto;
  struct MessageWakeup_ *wakeup;
  StgInt fd;
  StgWord target;
} StgTSOBlockInfo;

typedef StgWord64 StgThreadID;

typedef struct StgTVarWatchQueue_ StgTVarWatchQueue;

typedef struct StgTRecChunk_ StgTRecChunk;

typedef struct StgArrBytes {
  struct StgHeader header;
  StgWord bytes;
  struct __IncompleteArrayField_StgWord payload;
} StgArrBytes;

typedef struct StgBlockingQueue_ StgBlockingQueue;

typedef int64_t StgInt64;

typedef struct StgTSO_ {
  struct StgHeader header;
  struct StgTSO_ *_link;
  struct StgTSO_ *global_link;
  struct StgStack_ *stackobj;
  StgWord16 what_next;
  StgWord32 flags;
  StgWord32 why_blocked;
  union StgTSOBlockInfo block_info;
  StgThreadID id;
  StgWord32 saved_errno;
  StgWord32 dirty;
  struct InCall_ *bound;
  struct Capability_ *cap;
  struct StgTRecHeader_ *trec;
  struct StgArrBytes *label;
  struct MessageThrowTo_ *blocked_exceptions;
  StgBlockingQueue *bq;
  StgInt64 alloc_limit;
  StgWord32 tot_stack_size;
} StgTSO_;

typedef struct StgTSO_ StgTSO;

/**
 * An adjusted index into stable_ptr_table (see [ref:NULL StgStablePtr])
 */
typedef void *StgStablePtr;

typedef void *(*(*StgFunPtr)(void))(void);

typedef struct EventLogWriter {
  void (*initEventLogWriter)(void);
  bool (*writeEventLog)(void *eventlog, uintptr_t eventlog_size);
  void (*flushEventLog)(void);
  void (*stopEventLogWriter)(void);
} EventLogWriter;

typedef struct Capability_ Capability;

typedef struct ExecPage {
  char contents;
} ExecPage;

typedef struct __IncompleteArrayField_StgPtr {
  StgPtr _0[0];
} __IncompleteArrayField_StgPtr;

typedef struct ForeignExportsList {
  struct ForeignExportsList *next;
  int n_entries;
  struct _ObjectCode *oc;
  StgStablePtr **stable_ptrs;
  struct __IncompleteArrayField_StgPtr exports;
} ForeignExportsList;

typedef struct _HpcModuleInfo HpcModuleInfo;

typedef uint32_t StringIdx;

typedef struct IpeBufferEntry {
  StringIdx table_name;
  uint32_t closure_desc;
  StringIdx ty_desc;
  StringIdx label;
  StringIdx src_file;
  StringIdx src_span;
} IpeBufferEntry;

typedef struct IpeBufferListNode_ {
  struct IpeBufferListNode_ *next;
  StgWord compressed;
  StgWord count;
  const StgInfoTable **tables;
  struct IpeBufferEntry *entries;
  StgWord entries_size;
  const char *string_table;
  StgWord string_table_size;
  StringIdx unit_id;
  StringIdx module_name;
} IpeBufferListNode_;

typedef struct IpeBufferListNode_ IpeBufferListNode;

typedef struct InfoProv_ {
  const char *table_name;
  uint32_t closure_desc;
  const char *ty_desc;
  const char *label;
  const char *unit_id;
  const char *module;
  const char *src_file;
  const char *src_span;
} InfoProv_;

typedef struct InfoProv_ InfoProv;

typedef struct InfoProvEnt_ {
  const StgInfoTable *info;
  InfoProv prov;
} InfoProvEnt_;

typedef struct InfoProvEnt_ InfoProvEnt;

typedef struct BacktraceChunk_ BacktraceChunk;

typedef struct Backtrace_ {
  StgWord n_frames;
  BacktraceChunk *last;
} Backtrace_;

typedef struct Backtrace_ Backtrace;

typedef struct LibdwSession_ LibdwSession;

typedef struct Location_ Location;

typedef char pathchar;

typedef StgInt HsInt;

typedef void *HsPtr;

typedef StgInt HsBool;

typedef StgWord W_;

typedef int64_t Time;

typedef struct GCDetails_ GCDetails;

typedef struct RtsConfig {
  RtsOptsEnabledEnum rts_opts_enabled;
  HsBool rts_opts_suggestions;
  const char *rts_opts;
  HsBool rts_hs_main;
  HsBool keep_cafs;
  const struct EventLogWriter *eventlog_writer;
  void (*defaultsHook)(void);
  void (*onExitHook)(void);
  void (*stackOverflowHook)(W_ stack_size);
  void (*outOfHeapHook)(W_ request_size, W_ heap_size);
  void (*mallocFailHook)(W_ request_size, const char *msg);
  void (*gcDoneHook)(const GCDetails *stats);
  void (*longGCSync)(uint32_t this_cap, Time time_ns);
  void (*longGCSyncEnd)(Time time_ns);
} RtsConfig;

typedef void *StgAddr;

typedef StgWord32 StgChar;

typedef float StgFloat;

typedef union StgUnion {
  StgWord w;
  StgAddr a;
  StgChar c;
  StgFloat f;
  StgInt i;
  StgPtr p;
} StgUnion;

typedef double StgDouble;

typedef struct StgWord128 {
  StgWord64 h;
  StgWord64 l;
} StgWord128;

typedef struct StgWord256 {
  struct StgWord128 h;
  struct StgWord128 l;
} StgWord256;

typedef struct StgWord512 {
  struct StgWord256 h;
  struct StgWord256 l;
} StgWord512;

typedef int StgBool;

typedef struct CostCentre_ {
  StgInt ccID;
  char *label;
  char *module;
  char *srcloc;
  StgWord64 mem_alloc;
  StgWord time_ticks;
  StgBool is_caf;
  struct CostCentre_ *link;
} CostCentre_;

typedef struct CostCentre_ CostCentre;

typedef struct IndexTable_ {
  uint8_t _address;
} IndexTable_;

typedef struct CostCentreStack_ {
  StgInt ccsID;
  CostCentre *cc;
  struct CostCentreStack_ *prevStack;
  struct IndexTable_ *indexTable;
  struct CostCentreStack_ *root;
  StgWord depth;
  StgWord64 scc_count;
  StgWord selected;
  StgWord time_ticks;
  StgWord64 mem_alloc;
  StgWord64 inherited_alloc;
  StgWord inherited_ticks;
} CostCentreStack_;

typedef struct NonmovingSegmentInfo {
  StgWord16 allocator_idx;
  StgWord16 next_free_snap;
} NonmovingSegmentInfo;

typedef union bdescr___bindgen_ty_1 {
  StgPtr free;
  struct NonmovingSegmentInfo nonmoving_segment;
} bdescr___bindgen_ty_1;

typedef union bdescr___bindgen_ty_2 {
  struct bdescr_ *back;
  StgWord *bitmap;
  StgPtr scan;
} bdescr___bindgen_ty_2;

typedef StgWord memcount;

typedef struct _StgWeak StgWeak;

typedef struct generation_ {
  uint32_t no;
  bdescr *blocks;
  memcount n_blocks;
  memcount n_words;
  bdescr *large_objects;
  memcount n_large_blocks;
  memcount n_large_words;
  memcount n_new_large_words;
  bdescr *compact_objects;
  memcount n_compact_blocks;
  bdescr *compact_blocks_in_import;
  memcount n_compact_blocks_in_import;
  memcount max_blocks;
  StgTSO *threads;
  StgWeak *weak_ptr_list;
  struct generation_ *to;
  uint32_t collections;
  uint32_t par_collections;
  uint32_t failed_promotions;
  int mark;
  int compact;
  bdescr *old_blocks;
  memcount n_old_blocks;
  memcount live_estimate;
  bdescr *scavenged_large_objects;
  memcount n_scavenged_large_blocks;
  bdescr *live_compact_objects;
  memcount n_live_compact_blocks;
  bdescr *bitmap;
  StgTSO *old_threads;
  StgWeak *old_weak_ptr_list;
} generation_;

typedef struct bdescr_ {
  StgPtr start;
  union bdescr___bindgen_ty_1 __bindgen_anon_1;
  struct bdescr_ *link;
  union bdescr___bindgen_ty_2 u;
  struct generation_ *gen_;
  StgWord16 gen_no;
  StgWord16 dest_no;
  StgWord16 node;
  StgWord16 flags;
  StgWord32 blocks;
  StgWord32 _padding[3];
} bdescr_;

typedef struct bdescr_ bdescr;

typedef struct nursery_ {
  bdescr *blocks;
  memcount n_blocks;
} nursery_;

typedef struct StgRegTable {
  union StgUnion rR1;
  union StgUnion rR2;
  union StgUnion rR3;
  union StgUnion rR4;
  union StgUnion rR5;
  union StgUnion rR6;
  union StgUnion rR7;
  union StgUnion rR8;
  union StgUnion rR9;
  union StgUnion rR10;
  StgFloat rF1;
  StgFloat rF2;
  StgFloat rF3;
  StgFloat rF4;
  StgFloat rF5;
  StgFloat rF6;
  StgDouble rD1;
  StgDouble rD2;
  StgDouble rD3;
  StgDouble rD4;
  StgDouble rD5;
  StgDouble rD6;
  struct StgWord128 rXMM1;
  struct StgWord128 rXMM2;
  struct StgWord128 rXMM3;
  struct StgWord128 rXMM4;
  struct StgWord128 rXMM5;
  struct StgWord128 rXMM6;
  struct StgWord256 rYMM1;
  struct StgWord256 rYMM2;
  struct StgWord256 rYMM3;
  struct StgWord256 rYMM4;
  struct StgWord256 rYMM5;
  struct StgWord256 rYMM6;
  struct StgWord512 rZMM1;
  struct StgWord512 rZMM2;
  struct StgWord512 rZMM3;
  struct StgWord512 rZMM4;
  struct StgWord512 rZMM5;
  struct StgWord512 rZMM6;
  StgWord64 rL1;
  StgPtr rSp;
  StgPtr rSpLim;
  StgPtr rHp;
  StgPtr rHpLim;
  struct CostCentreStack_ *rCCCS;
  struct StgTSO_ *rCurrentTSO;
  struct nursery_ *rNursery;
  struct bdescr_ *rCurrentNursery;
  struct bdescr_ *rCurrentAlloc;
  StgWord rHpAlloc;
  StgWord rRet;
} StgRegTable;

typedef struct StgThunk_ StgThunk;

typedef struct TODO_ OSThreadId;

typedef void *(*OSThreadProc)(void *arg1);

typedef struct TODO_ Condition;

typedef struct TODO_ Mutex;

typedef StgInt I_;

typedef struct CostCentreStack_ CostCentreStack;

typedef struct StgInd {
  struct StgHeader header;
  StgClosure *indirectee;
} StgInd;

typedef struct StgIndStatic {
  struct StgHeader header;
  StgClosure *indirectee;
  StgClosure *static_link;
  const StgInfoTable *saved_info;
} StgIndStatic;

typedef struct __IncompleteArrayField_____StgClosure {
  StgClosure *_0[0];
} __IncompleteArrayField_____StgClosure;

typedef struct PauseToken_ PauseToken;

typedef struct _RTSStats RTSStats;

typedef struct StgClosure_ *HaskellObj;

typedef StgChar HsChar;

typedef int8_t StgInt8;

typedef StgInt8 HsInt8;

typedef int16_t StgInt16;

typedef StgInt16 HsInt16;

typedef int32_t StgInt32;

typedef StgInt32 HsInt32;

typedef StgInt64 HsInt64;

typedef StgWord HsWord;

typedef StgWord8 HsWord8;

typedef StgWord16 HsWord16;

typedef StgWord32 HsWord32;

typedef StgWord64 HsWord64;

typedef StgFloat HsFloat;

typedef StgDouble HsDouble;

typedef void (*ListThreadsCb)(void *user, StgTSO *arg1);

typedef void (*ListRootsCb)(void *user, StgClosure *arg1);

typedef struct HsIface {
  StgClosure *processRemoteCompletion_closure;
  StgClosure *runIO_closure;
  StgClosure *runNonIO_closure;
  StgClosure *Z0T_closure;
  StgClosure *True_closure;
  StgClosure *False_closure;
  StgClosure *unpackCString_closure;
  StgClosure *runFinalizzerBatch_closure;
  StgClosure *stackOverflow_closure;
  StgClosure *heapOverflow_closure;
  StgClosure *allocationLimitExceeded_closure;
  StgClosure *blockedIndefinitelyOnMVar_closure;
  StgClosure *blockedIndefinitelyOnSTM_closure;
  StgClosure *cannotCompactFunction_closure;
  StgClosure *cannotCompactPinned_closure;
  StgClosure *cannotCompactMutable_closure;
  StgClosure *nonTermination_closure;
  StgClosure *nestedAtomically_closure;
  StgClosure *noMatchingContinuationPrompt_closure;
  StgClosure *blockedOnBadFD_closure;
  StgClosure *runSparks_closure;
  StgClosure *ensureIOManagerIsRunning_closure;
  StgClosure *interruptIOManager_closure;
  StgClosure *ioManagerCapabilitiesChanged_closure;
  StgClosure *runHandlersPtr_closure;
  StgClosure *flushStdHandles_closure;
  StgClosure *runMainIO_closure;
  const StgInfoTable *Czh_con_info;
  const StgInfoTable *Izh_con_info;
  const StgInfoTable *Fzh_con_info;
  const StgInfoTable *Dzh_con_info;
  const StgInfoTable *Wzh_con_info;
  StgClosure *absentSumFieldError_closure;
  StgClosure *runAllocationLimitHandler_closure;
  const StgInfoTable *Ptr_con_info;
  const StgInfoTable *FunPtr_con_info;
  const StgInfoTable *I8zh_con_info;
  const StgInfoTable *I16zh_con_info;
  const StgInfoTable *I32zh_con_info;
  const StgInfoTable *I64zh_con_info;
  const StgInfoTable *W8zh_con_info;
  const StgInfoTable *W16zh_con_info;
  const StgInfoTable *W32zh_con_info;
  const StgInfoTable *W64zh_con_info;
  const StgInfoTable *StablePtr_con_info;
  StgClosure *StackSnapshot_closure;
  StgClosure *divZZeroException_closure;
  StgClosure *underflowException_closure;
  StgClosure *overflowException_closure;
  const StgInfoTable *unpackCStringzh_info;
  const StgInfoTable *unpackCStringUtf8zh_info;
} HsIface;

typedef struct generation_ generation;

typedef union StgFunInfoExtraRev___bindgen_ty_1 {
  StgWord bitmap;
  StgInt bitmap_offset;
} StgFunInfoExtraRev___bindgen_ty_1;

typedef struct StgFunInfoExtraRev_ {
  StgInt slow_apply_offset;
  union StgFunInfoExtraRev___bindgen_ty_1 b;
  StgHalfWord fun_type;
  StgHalfWord arity;
} StgFunInfoExtraRev_;

typedef struct StgFunInfoExtraRev_ StgFunInfoExtraRev;

typedef struct StgFunInfoTable {
  StgFunInfoExtraRev f;
  StgInfoTable i;
} StgFunInfoTable;

typedef struct StgIntCharlikeClosure {
  struct StgHeader header;
  StgWord data;
} StgIntCharlikeClosure;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern HsWord64 ghc_unique_counter64;

extern HsInt ghc_unique_inc;

extern StgWord nonmoving_write_barrier_enabled;

extern struct HsIface *ghc_hs_iface;

extern generation *generations;

extern generation *g0;

extern bool keepCAFs;

extern const StgWord stg_arg_bitmaps[0];

extern W_ mblocks_allocated;

extern uint32_t n_capabilities;

extern uint32_t enabled_capabilities;

extern const StgInfoTable stg_upd_frame_info;

extern const StgInfoTable stg_bh_upd_frame_info;

extern const StgInfoTable stg_catch_frame_info;

extern const StgInfoTable stg_catch_retry_frame_info;

extern const StgInfoTable stg_atomically_frame_info;

extern const StgInfoTable stg_catch_stm_frame_info;

extern const StgInfoTable stg_stack_underflow_frame_d_info;

extern const StgInfoTable stg_ctoi_t3_info;

extern const StgInfoTable stg_primcall_info;

extern const StgInfoTable stg_IND_STATIC_info;

extern const StgInfoTable __stg_EAGER_BLACKHOLE_info;

extern const struct StgFunInfoTable stg_BCO_info;

extern const StgInfoTable stg_STACK_info;

extern const StgInfoTable stg_ARR_WORDS_info;

extern const StgInfoTable stg_MUT_ARR_PTRS_FROZEN_CLEAN_info;

extern const StgInfoTable stg_SRT_1_info;

extern const StgInfoTable stg_SRT_16_info;

extern struct StgIntCharlikeClosure stg_INTLIKE_closure[272];

extern const StgInfoTable stg_unpack_cstring_info;

extern const StgInfoTable stg_unpack_cstring_utf8_info;

extern const StgInfoTable stg_ap_pp_info;

extern const StgInfoTable stg_ret_p_info;

extern const StgInfoTable stg_ret_n_info;

extern const StgInfoTable stg_ret_t_info;

extern const StgInfoTable stg_stop_thread_info;

extern StgInt ENT_STATIC_THK_SINGLE_ctr;

extern StgInt ENT_DYN_THK_SINGLE_ctr;

extern StgInt ENT_STATIC_THK_MANY_ctr;

extern StgInt ENT_DYN_THK_MANY_ctr;

extern StgInt ENT_STATIC_FUN_DIRECT_ctr;

extern StgInt ENT_DYN_FUN_DIRECT_ctr;

extern StgInt ENT_DYN_CON_ctr;

extern StgInt ENT_LNE_ctr;

extern StgInt UNKNOWN_CALL_ctr;

extern StgInt VERY_SLOW_CALL_ctr;

extern StgInt KNOWN_CALL_ctr;

extern StgInt KNOWN_CALL_TOO_FEW_ARGS_ctr;

extern StgInt KNOWN_CALL_EXTRA_ARGS_ctr;

extern StgInt UPDF_OMITTED_ctr;

extern StgInt UPDF_PUSHED_ctr;

extern StgInt ALLOC_HEAP_ctr;

extern StgInt ALLOC_HEAP_tot;

extern StgInt HEAP_CHK_ctr;

extern StgInt STK_CHK_ctr;

extern StgInt ALLOC_FUN_ctr;

extern StgInt ALLOC_FUN_gds;

extern StgInt UPD_CAF_BH_UPDATABLE_ctr;

extern StgInt UPD_CAF_BH_SINGLE_ENTRY_ctr;

extern StgInt ALLOC_UP_THK_ctr;

extern StgInt ALLOC_SE_THK_ctr;

extern StgInt ALLOC_THK_gds;

extern StgInt ALLOC_THK_slp;

extern StgInt ALLOC_CON_ctr;

extern StgInt ALLOC_CON_gds;

extern StgInt ALLOC_PRIM_ctr;

extern StgInt ALLOC_PRIM_adm;

extern StgInt ALLOC_PRIM_gds;

extern StgInt ALLOC_PRIM_slp;

extern StgInt ALLOC_PAP_ctr;

extern StgInt ALLOC_PAP_gds;

extern StgInt ALLOC_PAP_slp;

extern StgInt RET_NEW_ctr;

extern StgInt RET_OLD_ctr;

extern StgInt RET_UNBOXED_TUP_ctr;

extern StgInt TAG_UNTAGGED_pred;

extern StgInt TAG_UNTAGGED_miss;

extern StgInt TAG_TAGGED_pred;

extern StgInt RET_NEW_hst[9];

extern StgInt RET_OLD_hst[9];

extern StgInt RET_UNBOXED_TUP_hst[9];

void hs_init(int *argc, char ***argv);

void hs_exit(void);

void hs_exit_nowait(void);

void hs_thread_done(void);

void hs_perform_gc(void);

void hs_lock_stable_ptr_table(void);

void hs_lock_stable_tables(void);

void hs_unlock_stable_ptr_table(void);

void hs_unlock_stable_tables(void);

void hs_free_stable_ptr_unsafe(HsStablePtr sp);

void hs_free_stable_ptr(HsStablePtr sp);

void hs_free_fun_ptr(HsFunPtr fp);

StgPtr hs_spt_lookup(StgWord64 *key);

int hs_spt_keys(StgPtr *keys, int szKeys);

int hs_spt_key_count(void);

void hs_try_putmvar(int capability, HsStablePtr sp);

void hs_try_putmvar_with_value(int capability, HsStablePtr sp,
                               StgClosure *value);

void _assertFail(const char *filename, unsigned int linenum);

void reportStackOverflow(StgTSO *tso);

void reportHeapOverflow(void);

void stg_exit(int n);

int stg_sig_install(int arg1, int arg2, void *arg3);

int rts_isProfiled(void);

int rts_isDynamic(void);

int rts_isThreaded(void);

int rts_isDebugged(void);

int rts_isTracing(void);

void *createAdjustor(StgStablePtr hptr, StgFunPtr wptr, char *typeString);

void freeHaskellFunctionPtr(void *ptr);

void blockUserSignals(void);

void unblockUserSignals(void);

bool startEventLogging(const struct EventLogWriter *writer);

void endEventLogging(void);

void flushEventLog(Capability **cap);

struct ExecPage *allocateExecPage(void);

void freezeExecPage(struct ExecPage *page);

int lockFile(StgWord64 id, StgWord64 dev, StgWord64 ino, int for_writing);

int unlockFile(StgWord64 id);

void registerForeignExports(struct ForeignExportsList *exports);

StgWord64 getMonotonicNSec(void);

StgStablePtr getOrSetGHCConcSignalSignalHandlerStore(StgStablePtr ptr);

StgStablePtr getOrSetGHCConcWindowsPendingDelaysStore(StgStablePtr ptr);

StgStablePtr getOrSetGHCConcWindowsIOManagerThreadStore(StgStablePtr ptr);

StgStablePtr getOrSetGHCConcWindowsProddingStore(StgStablePtr ptr);

StgStablePtr getOrSetSystemEventThreadEventManagerStore(StgStablePtr ptr);

StgStablePtr getOrSetSystemEventThreadIOManagerThreadStore(StgStablePtr ptr);

StgStablePtr getOrSetSystemTimerThreadEventManagerStore(StgStablePtr ptr);

StgStablePtr getOrSetSystemTimerThreadIOManagerThreadStore(StgStablePtr ptr);

StgStablePtr getOrSetLibHSghcFastStringTable(StgStablePtr ptr);

StgStablePtr getOrSetLibHSghcGlobalHasPprDebug(StgStablePtr ptr);

StgStablePtr getOrSetLibHSghcGlobalHasNoDebugOutput(StgStablePtr ptr);

StgStablePtr getOrSetLibHSghcGlobalHasNoStateHack(StgStablePtr ptr);

void hs_hpc_module(char *modName, StgWord32 modCount, StgWord32 modHashNo,
                   StgWord64 *tixArr);

HpcModuleInfo *hs_hpc_rootModule(void);

void setIOManagerControlFd(uint32_t cap_no, int fd);

void setTimerManagerControlFd(int fd);

void setIOManagerWakeupFd(int fd);

void registerInfoProvList(IpeBufferListNode *node);

void formatClosureDescIpe(const InfoProvEnt *ipe_buf, char *str_buf);

bool lookupIPE(const StgInfoTable *info, InfoProvEnt *out);

void backtraceFree(Backtrace *bt);

Backtrace *libdwGetBacktrace(LibdwSession *session);

int libdwLookupLocation(LibdwSession *session, Location *loc, StgPtr pc);

LibdwSession *libdwPoolTake(void);

void libdwPoolRelease(LibdwSession *sess);

void libdwPoolClear(void);

void initLinker(void);

void initLinker_(int retain_cafs);

void *lookupSymbol(char *lbl);

OStatus getObjectLoadStatus(pathchar *path);

HsInt unloadObj(pathchar *path);

HsInt purgeObj(pathchar *path);

HsInt loadObj(pathchar *path);

HsInt loadArchive(pathchar *path);

HsInt resolveObjs(void);

void *loadNativeObj(pathchar *path, char **errmsg);

HsInt unloadNativeObj(void *handle);

void *lookupSymbolInNativeObj(void *handle, const char *symbol_name);

const char *addDLL(pathchar *dll_name);

HsPtr addLibrarySearchPath(pathchar *dll_path);

HsBool removeLibrarySearchPath(HsPtr dll_path_index);

pathchar *findSystemLibrary(pathchar *dll_name);

void hs_main(int argc, char **argv, StgClosure *main_closure,
             struct RtsConfig rts_config);

void rtsOutOfBoundsAccess(void);

void rtsMemcpyRangeOverlap(void);

void updateRemembSetPushClosure_(struct StgRegTable *reg,
                                 struct StgClosure_ *p);

void updateRemembSetPushThunk_(struct StgRegTable *reg, StgThunk *p);

StgFunPtr stg_copyArray_barrier(void);

void shutdownThread(void);

int createOSThread(OSThreadId *tid, const char *name, OSThreadProc startProc,
                   void *param);

void initCondition(Condition *pCond);

void broadcastCondition(Condition *pCond);

void waitCondition(Condition *pCond, Mutex *pMut);

void initMutex(Mutex *pMut);

int forkOS_createThread(HsStablePtr entry);

uint32_t getNumberOfProcessors(void);

StgInt newSpark(struct StgRegTable *reg, StgClosure *p);

StgDouble __int_encodeDouble(I_ j, I_ e);

StgDouble __word_encodeDouble(W_ j, I_ e);

void stopProfTimer(void);

void startProfTimer(void);

void requestHeapCensus(void);

void startHeapProfTimer(void);

void stopHeapProfTimer(void);

void setUserEra(StgWord w);

StgWord getUserEra(void);

StgWord incrementUserEra(StgWord w);

void registerCcList(CostCentre **cc_list);

void registerCcsList(CostCentreStack **cc_list);

void hs_spt_insert(StgWord64 *key, void *spe_closure);

void hs_spt_insert_stableptr(StgWord64 *key, StgStablePtr *entry);

void hs_spt_remove(StgWord64 *key);

bdescr *allocAlignedGroupOnNode(uint32_t node, W_ n);

bdescr *allocGroup_lock(W_ n);

void freeGroup_lock(bdescr *p);

StgPtr allocate(Capability *cap, W_ n);

void setAllocLimitKill(bool arg1, bool arg2);

void performGC(void);

void performMajorGC(void);

void performBlockingMajorGC(void);

struct StgInd *newCAF(struct StgRegTable *reg, struct StgIndStatic *caf);

void revertCAFs(void);

void setKeepCAFs(void);

void setHighMemDynamic(void);

void dirty_MUT_VAR(struct StgRegTable *reg, struct StgMutVar *mv,
                   StgClosure *old);

StgWord heap_view_closureSize(StgClosure *closure);

StgWord collect_pointers(StgClosure *closure, StgClosure **ptrs);

void *getMBlocks(uint32_t n);

void freeMBlocks(void *addr, uint32_t n);

void releaseFreeMemory(void);

StgTSO *createThread(Capability *cap, W_ stack_size);

StgTSO *createGenThread(Capability *cap, W_ stack_size, StgClosure *closure);

void *suspendThread(struct StgRegTable *arg1, bool interruptible);

struct StgRegTable *resumeThread(void *arg1);

bool eq_thread(StgPtr tso1, StgPtr tso2);

int cmp_thread(StgPtr tso1, StgPtr tso2);

StgThreadID rts_getThreadId(StgPtr tso);

void rts_enableThreadAllocationLimit(StgPtr tso);

void rts_disableThreadAllocationLimit(StgPtr tso);

struct _StgMutArrPtrs *listThreads(Capability *cap);

pid_t forkProcess(HsStablePtr *entry);

HsBool rtsSupportsBoundThreads(void);

void setNumCapabilities(uint32_t new_);

void requestTickyCounterSamples(void);

Time getProcessElapsedTime(void);

void startTimer(void);

void stopTimer(void);

int rtsTimerSignal(void);

void *__hscore_get_saved_termios(int fd);

void __hscore_set_saved_termios(int fd, void *ts);

int genericRaise(int sig);

Capability *pauseTokenCapability(PauseToken *pauseToken);

void getRTSStats(RTSStats *s);

int getRTSStatsEnabled(void);

uint64_t getAllocations(void);

void hs_init_with_rtsopts(int *argc, char ***argv);

void hs_init_ghc(int *argc, char ***argv, struct RtsConfig rts_config);

void shutdownHaskellAndExit(int exitCode, int fastExit);

void shutdownHaskellAndSignal(int sig, int fastExit);

void getProgArgv(int *argc, char ***argv);

void setProgArgv(int argc, char **argv);

void getFullProgArgv(int *argc, char ***argv);

Capability *rts_lock(void);

void rts_unlock(Capability *token);

Capability *rts_unsafeGetMyCapability(void);

void rts_setInCallCapability(int preferred_capability, int affinity);

void rts_pinThreadToNumaNode(int node);

HaskellObj rts_mkChar(Capability *arg1, HsChar c);

HaskellObj rts_mkInt(Capability *arg1, HsInt i);

HaskellObj rts_mkInt8(Capability *arg1, HsInt8 i);

HaskellObj rts_mkInt16(Capability *arg1, HsInt16 i);

HaskellObj rts_mkInt32(Capability *arg1, HsInt32 i);

HaskellObj rts_mkInt64(Capability *arg1, HsInt64 i);

HaskellObj rts_mkWord(Capability *arg1, HsWord w);

HaskellObj rts_mkWord8(Capability *arg1, HsWord8 w);

HaskellObj rts_mkWord16(Capability *arg1, HsWord16 w);

HaskellObj rts_mkWord32(Capability *arg1, HsWord32 w);

HaskellObj rts_mkWord64(Capability *arg1, HsWord64 w);

HaskellObj rts_mkPtr(Capability *arg1, HsPtr a);

HaskellObj rts_mkFunPtr(Capability *arg1, HsFunPtr a);

HaskellObj rts_mkFloat(Capability *arg1, HsFloat f);

HaskellObj rts_mkDouble(Capability *arg1, HsDouble f);

HaskellObj rts_mkStablePtr(Capability *arg1, HsStablePtr s);

HaskellObj rts_mkBool(Capability *arg1, HsBool b);

HaskellObj rts_mkString(Capability *arg1, char *s);

HaskellObj rts_apply(Capability *arg1, HaskellObj arg2, HaskellObj arg3);

HsChar rts_getChar(HaskellObj arg1);

HsInt rts_getInt(HaskellObj arg1);

HsInt8 rts_getInt8(HaskellObj arg1);

HsInt16 rts_getInt16(HaskellObj arg1);

HsInt32 rts_getInt32(HaskellObj arg1);

HsInt64 rts_getInt64(HaskellObj arg1);

HsWord rts_getWord(HaskellObj arg1);

HsWord8 rts_getWord8(HaskellObj arg1);

HsWord16 rts_getWord16(HaskellObj arg1);

HsWord32 rts_getWord32(HaskellObj arg1);

HsWord64 rts_getWord64(HaskellObj arg1);

HsPtr rts_getPtr(HaskellObj arg1);

HsFunPtr rts_getFunPtr(HaskellObj arg1);

HsFloat rts_getFloat(HaskellObj arg1);

HsDouble rts_getDouble(HaskellObj arg1);

HsStablePtr rts_getStablePtr(HaskellObj arg1);

HsBool rts_getBool(HaskellObj arg1);

void rts_eval(Capability **arg1, HaskellObj p, HaskellObj *ret);

void rts_eval_(Capability **arg1, HaskellObj p, unsigned int stack_size,
               HaskellObj *ret);

void rts_evalIO(Capability **arg1, HaskellObj p, HaskellObj *ret);

void rts_evalStableIOMain(Capability **arg1, HsStablePtr s, HsStablePtr *ret);

void rts_evalStableIO(Capability **arg1, HsStablePtr s, HsStablePtr *ret);

void rts_evalLazyIO(Capability **arg1, HaskellObj p, HaskellObj *ret);

void rts_evalLazyIO_(Capability **arg1, HaskellObj p, unsigned int stack_size,
                     HaskellObj *ret);

void rts_inCall(Capability **arg1, HaskellObj p, HaskellObj *ret);

void rts_checkSchedStatus(char *site, Capability *arg1);

SchedulerStatus rts_getSchedStatus(Capability *cap);

PauseToken *rts_pause(void);

void rts_resume(PauseToken *pauseToken);

bool rts_isPaused(void);

void rts_listThreads(ListThreadsCb cb, void *user);

void rts_listMiscRoots(ListRootsCb cb, void *user);

void rts_clearMemory(void);

StgFunPtr stg_ctoi_t(void);

StgFunPtr stg_ap_n_fast(void);

StgFunPtr stg_ap_p_fast(void);

StgFunPtr stg_ap_pp_fast(void);

StgFunPtr stg_ap_ppp_fast(void);

StgFunPtr stg_gc_noregs(void);

StgFunPtr __stg_gc_enter_1(void);

StgFunPtr stg_gc_unpt_r1(void);

StgFunPtr stg_gc_unbx_r1(void);

StgFunPtr stg_gc_f1(void);

StgFunPtr stg_gc_d1(void);

StgFunPtr stg_gc_l1(void);

StgFunPtr stg_gc_pp(void);

StgFunPtr stg_gc_ppp(void);

StgFunPtr stg_gc_pppp(void);

StgFunPtr __stg_gc_fun(void);

StgFunPtr StgReturn(void);

StgFunPtr stg_paniczh(void);

StgFunPtr stg_absentErrorzh(void);

StgFunPtr stg_getThreadAllocationCounterzh(void);

StgFunPtr stg_getOtherThreadAllocationCounterzh(void);

StgWord hs_atomic_add8(StgWord x, StgWord val);

StgWord hs_atomic_add16(StgWord x, StgWord val);

StgWord hs_atomic_add32(StgWord x, StgWord val);

StgWord64 hs_atomic_add64(StgWord x, StgWord64 val);

StgWord hs_atomic_sub8(StgWord x, StgWord val);

StgWord hs_atomic_sub16(StgWord x, StgWord val);

StgWord hs_atomic_sub32(StgWord x, StgWord val);

StgWord64 hs_atomic_sub64(StgWord x, StgWord64 val);

StgWord hs_atomic_and8(StgWord x, StgWord val);

StgWord hs_atomic_and16(StgWord x, StgWord val);

StgWord hs_atomic_and32(StgWord x, StgWord val);

StgWord64 hs_atomic_and64(StgWord x, StgWord64 val);

StgWord hs_atomic_nand8(StgWord x, StgWord val);

StgWord hs_atomic_nand16(StgWord x, StgWord val);

StgWord hs_atomic_nand32(StgWord x, StgWord val);

StgWord64 hs_atomic_nand64(StgWord x, StgWord64 val);

StgWord hs_atomic_or8(StgWord x, StgWord val);

StgWord hs_atomic_or16(StgWord x, StgWord val);

StgWord hs_atomic_or32(StgWord x, StgWord val);

StgWord64 hs_atomic_or64(StgWord x, StgWord64 val);

StgWord hs_atomic_xor8(StgWord x, StgWord val);

StgWord hs_atomic_xor16(StgWord x, StgWord val);

StgWord hs_atomic_xor32(StgWord x, StgWord val);

StgWord64 hs_atomic_xor64(StgWord x, StgWord64 val);

StgWord hs_cmpxchg8(StgWord x, StgWord old, StgWord new_);

StgWord hs_cmpxchg16(StgWord x, StgWord old, StgWord new_);

StgWord hs_cmpxchg32(StgWord x, StgWord old, StgWord new_);

StgWord64 hs_cmpxchg64(StgWord x, StgWord64 old, StgWord64 new_);

StgWord hs_atomicread8(StgWord x);

StgWord hs_atomicread16(StgWord x);

StgWord hs_atomicread32(StgWord x);

StgWord64 hs_atomicread64(StgWord x);

void hs_atomicwrite8(StgWord x, StgWord val);

void hs_atomicwrite16(StgWord x, StgWord val);

void hs_atomicwrite32(StgWord x, StgWord val);

void hs_atomicwrite64(StgWord x, StgWord64 val);

StgWord hs_xchg8(StgWord x, StgWord val);

StgWord hs_xchg16(StgWord x, StgWord val);

StgWord hs_xchg32(StgWord x, StgWord val);

StgWord64 hs_xchg64(StgWord x, StgWord64 val);

StgWord16 hs_bswap16(StgWord16 x);

StgWord32 hs_bswap32(StgWord32 x);

StgWord64 hs_bswap64(StgWord64 x);

StgWord hs_bitrev8(StgWord x);

StgWord16 hs_bitrev16(StgWord16 x);

StgWord32 hs_bitrev32(StgWord32 x);

StgWord64 hs_bitrev64(StgWord64 x);

StgWord64 hs_pdep64(StgWord64 src, StgWord64 mask);

StgWord hs_pdep32(StgWord src, StgWord mask);

StgWord hs_pdep16(StgWord src, StgWord mask);

StgWord hs_pdep8(StgWord src, StgWord mask);

StgWord64 hs_pext64(StgWord64 src, StgWord64 mask);

StgWord hs_pext32(StgWord src, StgWord mask);

StgWord hs_pext16(StgWord src, StgWord mask);

StgWord hs_pext8(StgWord src, StgWord mask);

StgWord hs_popcnt8(StgWord x);

StgWord hs_popcnt16(StgWord x);

StgWord hs_popcnt32(StgWord x);

StgWord hs_popcnt64(StgWord64 x);

StgWord hs_popcnt(StgWord x);

StgFloat hs_word2float32(StgWord x);

StgDouble hs_word2float64(StgWord x);

StgWord hs_clz8(StgWord x);

StgWord hs_clz16(StgWord x);

StgWord hs_clz32(StgWord x);

StgWord hs_clz64(StgWord64 x);

StgWord hs_ctz8(StgWord x);

StgWord hs_ctz16(StgWord x);

StgWord hs_ctz32(StgWord x);

StgWord hs_ctz64(StgWord64 x);

W_ hs_mulIntMayOflo(W_ a, W_ b);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
