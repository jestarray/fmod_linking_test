use fmod_sys::{
    FMOD_Debug_Initialize, FMOD_Studio_Bank_LoadSampleData,
    FMOD_Studio_EventDescription_CreateInstance, FMOD_Studio_EventInstance_Release,
    FMOD_Studio_EventInstance_SetVolume, FMOD_Studio_EventInstance_Start,
    FMOD_Studio_EventInstance_Stop, FMOD_Studio_ParseID, FMOD_Studio_System_Create,
    FMOD_Studio_System_GetCoreSystem, FMOD_Studio_System_GetEventByID,
    FMOD_Studio_System_Initialize, FMOD_Studio_System_LoadBankFile, FMOD_Studio_System_Update,
    FMOD_System_GetDriverInfo, FMOD_System_GetNumDrivers, FMOD_DEBUG_DISPLAY_LINENUMBERS,
    FMOD_DEBUG_DISPLAY_TIMESTAMPS, FMOD_DEBUG_LEVEL_ERROR, FMOD_DEBUG_LEVEL_WARNING,
    FMOD_DEBUG_MODE, FMOD_DEBUG_TYPE_MEMORY, FMOD_GUID, FMOD_INIT_NORMAL, FMOD_STUDIO_BANK,
    FMOD_STUDIO_EVENTDESCRIPTION, FMOD_STUDIO_EVENTINSTANCE, FMOD_STUDIO_INIT_NORMAL,
    FMOD_STUDIO_LOAD_BANK_NONBLOCKING, FMOD_STUDIO_STOP_MODE, FMOD_STUDIO_SYSTEM, FMOD_SYSTEM,
    FMOD_VERSION,
};
use std::{ffi::CString, ptr::null_mut};
fn main() {
    println!("Hello, world!");
    unsafe {
        let mut system = null_mut();
        /*  FMOD_System_GetNumDrivers();
        FMOD_System_GetDriverInfo(); */
        FMOD_Studio_System_Create(&mut system, FMOD_VERSION);

        let mut core = null_mut();
        FMOD_Studio_System_GetCoreSystem(system, &mut core);

        FMOD_Studio_System_Initialize(
            system,
            32,
            FMOD_STUDIO_INIT_NORMAL,
            FMOD_INIT_NORMAL,
            std::ptr::null_mut(),
        );
    }
}
