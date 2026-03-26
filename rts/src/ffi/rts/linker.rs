pub use crate::linker::load_archive::loadArchive;
pub use crate::linker::{
    OStatus, addDLL, addLibrarySearchPath, findSystemLibrary, getObjectLoadStatus, initLinker,
    initLinker_, loadNativeObj, loadObj, lookupSymbol, lookupSymbolInNativeObj, pathchar, purgeObj,
    removeLibrarySearchPath, resolveObjs, unloadNativeObj, unloadObj,
};
use crate::prelude::*;

impl TryFrom<u32> for OStatus {
    type Error = ();

    fn try_from(d: u32) -> Result<OStatus, ()> {
        use OStatus::*;

        match d {
            0 => Ok(OBJECT_LOADED),
            1 => Ok(OBJECT_NEEDED),
            2 => Ok(OBJECT_RESOLVED),
            3 => Ok(OBJECT_READY),
            4 => Ok(OBJECT_UNLOADED),
            5 => Ok(OBJECT_DONT_RESOLVE),
            6 => Ok(OBJECT_NOT_LOADED),
            _ => Err(()),
        }
    }
}
