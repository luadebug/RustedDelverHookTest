use std::fmt::Debug;
use std::os::raw::c_void;
use std::os::windows::io::AsRawHandle;
use std::{fs::OpenOptions, time::Duration};

use crate::hooks::patcher;
use crate::jnihook::jnihook::{
    jnihook_result_t_JNIHOOK_ERR_ADD_JVMTI_CAPS,
    jnihook_result_t_JNIHOOK_ERR_CLASS_FILE_CACHE,
    jnihook_result_t_JNIHOOK_ERR_GET_JNI,
    jnihook_result_t_JNIHOOK_ERR_GET_JVMTI,
    jnihook_result_t_JNIHOOK_ERR_JAVA_EXCEPTION,
    jnihook_result_t_JNIHOOK_ERR_JNI_OPERATION,
    jnihook_result_t_JNIHOOK_ERR_JVMTI_OPERATION,
    jnihook_result_t_JNIHOOK_ERR_SETUP_CLASS_FILE_LOAD_HOOK,
    jnihook_result_t_JNIHOOK_OK
};
#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types
)]
use crate::jnihook::jnihook::{JNIHook_Attach, JNIHook_Init};
use crate::modules::manager;
use crate::modules::module::ModuleData;
use crate::util::logger::Logger;
use jni::sys::{jint, jmethodID, jvalue, JNIEnv};
use jni::AttachGuard;
use once_cell::sync::OnceCell;
use winapi::um::libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA};
use windows::core::s;

// static mut JAVA_VM: OnceCell<jni::JavaVM> = OnceCell::new();
// static mut ENV: OnceCell<AttachGuard> = OnceCell::new();
// static mut RUNNING: bool = true;

// extern "C" fn hk_player_attack(
//     jni: *mut jni::JNIEnv,
//     callable_method: jmethodID,
//     args: *mut jvalue
// ) -> jvalue {
//     Logger::log("[DH] hk_player_attack called!");
//     if !args.is_null() {
//         Logger::log(format!("[hk_player_attack] args: {args:#?}"));
//     }
//     jvalue { i: 0 }
// }
extern "C" fn hk_take_damage(
    _jni: *mut JNIEnv,
    _callable_method: jmethodID,
    args: *mut jvalue
) -> jvalue {
    Logger::log("[DH] hk_take_damage called!");
    if !args.is_null() {
        Logger::log(format!("[hk_take_damage] args: {args:#?}"));
    }
    jvalue { i: 0 }
}

extern "C" fn hk_get_walk_speed(
    _jni: *mut JNIEnv,
    _callable_method: jmethodID,
    args: *mut jvalue
) -> jvalue {
    Logger::log("[DH] hk_get_walk_speed called!");
    if !args.is_null() {
        Logger::log(format!("[hk_get_walk_speed] args: {args:#?}"));
    }
    jvalue { f: 0.3f32 }
}


extern "C" fn hk_get_damage_stat_boost_method(
    _jni: *mut JNIEnv,
    _callable_method: jmethodID,
    args: *mut jvalue
) -> jvalue {
    Logger::log("[DH] hk_get_damage_stat_boost_method called!");
    if !args.is_null() {
        Logger::log(format!("[hk_get_damage_stat_boost_method] args: {args:#?}"));
    }
    jvalue { i: 99999 }
}

extern "C" fn hk_get_attack_speed_stat_boost(
    _jni: *mut JNIEnv,
    _callable_method: jmethodID,
    args: *mut jvalue
) -> jvalue {
    Logger::log("[DH] hk_get_attack_speed_stat_boost called!");
    if !args.is_null() {
        Logger::log(format!("[hk_get_attack_speed_stat_boost] args: {args:#?}"));
    }
    jvalue { f: 10.0f32 }
}

extern "C" fn hk_get_magic_resist_mod_boost_method(
    _jni: *mut JNIEnv,
    _callable_method: jmethodID,
    args: *mut jvalue
) -> jvalue {
    Logger::log("[DH] hk_get_magic_resist_mod_boost_method called!");
    if !args.is_null() {
        Logger::log(format!("[hk_get_magic_resist_mod_boost_method] args: {args:#?}"));
    }
    jvalue { f: 999.0f32 }
}

extern "C" fn hk_get_magic_stat_boost_method(
    _jni: *mut JNIEnv,
    _callable_method: jmethodID,
    args: *mut jvalue
) -> jvalue {
    Logger::log("[DH] hk_get_magic_stat_boost_method called!");
    if !args.is_null() {
        Logger::log(format!("[hk_get_magic_stat_boost_method] args: {args:#?}"));
    }
    jvalue { i: 99999 }
}

extern "C" fn hk_get_defense_stat_boost_method(
    _jni: *mut JNIEnv,
    _callable_method: jmethodID,
    args: *mut jvalue
) -> jvalue {
    Logger::log("[DH] hk_get_defense_stat_boost_method called!");
    if !args.is_null() {
        Logger::log(format!("[hk_get_defense_stat_boost_method] args: {args:#?}"));
    }
    jvalue { i: 99999 }
}

pub unsafe fn entry() {
    let mut jvm: OnceCell<jni::JavaVM> = OnceCell::new();
    let mut java_thread: OnceCell<AttachGuard> = OnceCell::new();
    Logger::log("Attached to javaw.exe");
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open("CONOUT$")
        .unwrap();
    let _ = winapi::um::processenv::SetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE, file.as_raw_handle() as *mut c_void);

    Logger::log("Set STD output handle");

    let vms = crate::util::jvm::get_created_jvms();

    if vms.is_none() {
        exit_log("No JVM Was found, exiting...");
    }
    Logger::log_fmt(format_args!("{}{:?}", "Found JVMS: ", vms));

    let vm = vms.unwrap()[0];

    if vm.is_null() {
        exit_log("First VM was null, exiting...");
    }
    Logger::log_fmt(format_args!("{}{:?}", "First JVM: ", vm));

    let java_vm_res = jni::JavaVM::from_raw(vm as *mut jni::sys::JavaVM);
    match java_vm_res
    {
        Ok(java_vm) => {
            jvm = OnceCell::from(java_vm);
        }
        Err(err) => {
            Logger::log_fmt(format_args!("Failed to retrieve JavaVM pointer: {:?}", err));
        }
    }
    let env_res = jvm.wait().attach_current_thread(); //java_vm.get_mut().unwrap().attach_current_thread();
    match env_res
    {
        Ok(env) => {
            java_thread = OnceCell::from(env);
        }
        Err(err) => {
            Logger::log_fmt(format_args!("Failed to retrieve JNI environment: {:?}", err));
        }
    }

    Logger::log("Set Global vars");
        let vm_ptr = jvm.wait().get_java_vm_pointer();

        let jnienv = java_thread.get_mut().unwrap();

            //TODO fix attackmethod hook
            // let Attack_method = jnienv.get_method_id(
            //     "com/interrupt/dungeoneer/entities/Player", "Attack",
            //     "(Lcom/interrupt/dungeoneer/game/Level;)V"
            // ).unwrap();
            //TODO fix movespeed hook
            // let get_walk_speed_method = jnienv.get_method_id(pc_ref, "getWalkSpeed", "()F").unwrap();

            let take_damage_method = jnienv.get_method_id(
                "com/interrupt/dungeoneer/entities/Player", "takeDamage",
                "(ILcom/interrupt/dungeoneer/entities/items/Weapon$DamageType;Lcom/interrupt/dungeoneer/entities/Entity;)I"
            ).unwrap();

            let get_attack_speed_stat_boost_method = jnienv.get_method_id(
                "com/interrupt/dungeoneer/entities/Player", "getAttackSpeedStatBoost",
                "()F"
            ).unwrap();

            let get_magic_resist_mod_boost_method = jnienv.get_method_id(
                "com/interrupt/dungeoneer/entities/Player", "getMagicResistModBoost",
                "()F"
            ).unwrap();

            let get_magic_stat_boost_method = jnienv.get_method_id(
                "com/interrupt/dungeoneer/entities/Player", "getMagicStatBoost",
                "()I"
            ).unwrap();

            let get_damage_stat_boost_method = jnienv.get_method_id(
                "com/interrupt/dungeoneer/entities/Player", "getDamageStatBoost",
                "()I"
            ).unwrap();

            let get_defense_stat_boost_method = jnienv.get_method_id(
                "com/interrupt/dungeoneer/entities/Player", "getDefenseStatBoost",
                "()I"
            ).unwrap();

            let jnihook_init_res = JNIHook_Init(vm_ptr);
            match jnihook_init_res {
                jnihook_result_t_JNIHOOK_OK => {
                    Logger::log("[JNIHOOK_OK] Successfully initialized JNIHook!");
                }
                jnihook_result_t_JNIHOOK_ERR_GET_JNI => {
                    Logger::log("[JNIHOOK_ERR_GET_JNI] Failed to get JNI interface");
                }
                jnihook_result_t_JNIHOOK_ERR_GET_JVMTI => {
                    Logger::log("[JNIHOOK_ERR_GET_JVMTI] Failed to get JVMTI interface");
                }
                jnihook_result_t_JNIHOOK_ERR_ADD_JVMTI_CAPS => {
                    Logger::log("[JNIHOOK_ERR_ADD_JVMTI_CAPS] Failed to add JVMTI capabilities");
                }
                jnihook_result_t_JNIHOOK_ERR_SETUP_CLASS_FILE_LOAD_HOOK => {
                    Logger::log("[JNIHOOK_ERR_SETUP_CLASS_FILE_LOAD_HOOK] Failed to setup class file load hook");
                }
                jnihook_result_t_JNIHOOK_ERR_JNI_OPERATION => {
                    Logger::log("[JNIHOOK_ERR_JNI_OPERATION] JNI operation failed");
                }
                jnihook_result_t_JNIHOOK_ERR_JVMTI_OPERATION => {
                    Logger::log("[JNIHOOK_ERR_JVMTI_OPERATION] JVMTI operation failed");
                }
                jnihook_result_t_JNIHOOK_ERR_CLASS_FILE_CACHE => {
                    Logger::log("[JNIHOOK_ERR_CLASS_FILE_CACHE] Class file cache error");
                }
                jnihook_result_t_JNIHOOK_ERR_JAVA_EXCEPTION => {
                    Logger::log("[JNIHOOK_ERR_JAVA_EXCEPTION] Java exception occurred during initialization");
                }
                _ => {
                    Logger::log(&format!("[UNKNOWN_ERROR] Unknown error code: {}", jnihook_init_res));
                }
            }

            let mut hkresult: jint;
            //TODO fix attackmethod hook

            // hkresult = JNIHook_Attach(Attack_method.into_raw(),
            //                           hk_player_attack as *mut c_void,
            //                           std::ptr::null_mut());
            // Logger::log(format!("[DH] Player::Attack Hook Result: {}", hkresult));

            // hkresult = JNIHook_Attach(get_walk_speed_method.into_raw(),
            //                           hk_get_walk_speed as *mut c_void,
            //                           std::ptr::null_mut());
            // Logger::log(format!("[DH] Player::getWalkSpeed Hook Result: {}", hkresult));

            hkresult = JNIHook_Attach(take_damage_method.into_raw(),
                                      hk_take_damage as *mut c_void,
                                      std::ptr::null_mut());
            Logger::log(format!("[DH] Player::takeDamage Hook Result: {}", hkresult));
            hkresult = JNIHook_Attach(get_damage_stat_boost_method.into_raw(),
                                      hk_get_damage_stat_boost_method as *mut c_void,
                                      std::ptr::null_mut());
            Logger::log(format!("[DH] Player::getDamageStatBoost Hook Result: {}", hkresult));
            hkresult = JNIHook_Attach(get_attack_speed_stat_boost_method.into_raw(),
                                      hk_get_attack_speed_stat_boost as *mut c_void,
                                      std::ptr::null_mut());
            Logger::log(format!("[DH] Player::getAttackSpeedStatBoost Hook Result: {}", hkresult));
            hkresult = JNIHook_Attach(get_magic_resist_mod_boost_method.into_raw(),
                                      hk_get_magic_resist_mod_boost_method as *mut c_void,
                                      std::ptr::null_mut());
            Logger::log(format!("[DH] Player::get_magic_resist_mod_boost_method Hook Result: {}", hkresult));
            hkresult = JNIHook_Attach(get_magic_stat_boost_method.into_raw(),
                                      hk_get_magic_stat_boost_method as *mut c_void,
                                      std::ptr::null_mut());
            Logger::log(format!("[DH] Player::get_magic_stat_boost_method Hook Result: {}", hkresult));
            hkresult = JNIHook_Attach(get_defense_stat_boost_method.into_raw(),
                                      hk_get_defense_stat_boost_method as *mut c_void,
                                      std::ptr::null_mut());
            Logger::log(format!("[DH] Player::get_defense_stat_boost_method Hook Result: {}", hkresult));
        // }
        // else {
            // Handle the case where java_vm is None
            // Logger::log("[PANIC] JNIENV is None!!!");
        // }
        //let mut a: JavaVM = jni::sys::JavaVM::from(b.get_java_vm_pointer()).expect("REASON");

        //JNIHook_Init(&mut a);
        //JNIHook_Init(&mut a);
    // } else {
        // Handle the case where java_vm is None
        // Logger::log("[PANIC] java_vm is None!!!");
    // }
    //Logger::log("Retrieving Class Loader...");

    //CLASS_LOADER = crate::util::jvm::get_class_loader();

    //Logger::log_fmt(format_args!("{}{:?}", "Retrieved Class Loader: ", CLASS_LOADER.as_ref().unwrap()));

    Logger::log("Applying patches...");
    patcher::apply_patches();
    Logger::log("Patches applied");

    //Logger::log("Loading Mappings...");
    //mappings::init_mappings();
    //Logger::log("Mappings loaded");
    
    //Logger::log("Getting MC Type...");
    //mappings::init_mappings();
    //Logger::log_fmt(format_args!("{}{:?}", "MC Type: ", mappings::CURRENT_TYPE));

    //Logger::log("Initializing Modules...");
    //let mut key_handler = KeyHandler::new();
    //manager::init();
    //Logger::log("Modules initialized");

    Logger::log("Looping...");

    let _loop_thread = std::thread::spawn(|| {
        Logger::log("Quitted loop thread");
    });

    //let mut ticks = 0;
    
    loop {

        //key_handler.on_tick();
        //manager::on_loop();

        //let last_ticks = ticks;
        //Logger::log(format!("Assigned last_ticks = ticks {} = {}", last_ticks, ticks));
        //let mc = Minecraft::get_minecraft();
        //Logger::log("Finished getting minecraft");
        //let player = mc.the_player();
        //Logger::log("Finished getting player");
        //Logger::log(format!("Player X={} Y={} Z={}", player.get_pos_x(), player.get_pos_y(), player.get_pos_z()));
        //ticks = player.get_ticks_existed();
        //if last_ticks != ticks {
            //manager::on_tick();
        //}

        //if GetAsyncKeyState(win_key_codes::VK_0) != 0 || !RUNNING {
            //RUNNING = false;
            //break;
        //}
        std::thread::sleep(Duration::from_millis(2))
    }

    //loop_thread.join().unwrap();

    //exit_log("Exited loop, now freeing library...")
}

pub unsafe fn on_key(key: i32) {
    for module in manager::MODULES.as_mut().unwrap().iter_mut() {
        let m: &mut ModuleData = module.as_mut().get_mod();
        m.on_key(key as i16);
        if m.key == key as i16 {
            if m.toggled {
                module.on_enable()
            } else {
                module.on_disable()
            }
        }
    }
}

pub unsafe fn exit_log(log: &str) {
    Logger::log(log);
    std::thread::sleep(Duration::from_millis(500));
    exit();
}

pub unsafe fn exit() {
    Logger::log("Deinitializing patches...");
    patcher::stop();
    Logger::log("Deinitialized patches, Freeing library...");
    let module = GetModuleHandleA(s!("void.dll").as_ptr() as *const i8);
    FreeLibraryAndExitThread(module, 0);
}
