use std::{fs::OpenOptions, time::Duration};
use std::fmt::Debug;
use std::os::windows::io::AsRawHandle;

use jni::AttachGuard;
use jni::objects::{JClass, JMethodID, JObject};
use jni::signature::{Primitive, ReturnType};
use jni::sys::{jint, jmethodID, JNI_OK, JNIEnv, jobject, jvalue};
use winapi::um::libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA};
use windows::core::s;

use crate::hooks::patcher;
use crate::jnihook::jnihook::{JNIHook_Attach, JNIHook_Init};
use crate::modules::manager;
use crate::modules::module::ModuleData;
use crate::util::logger::Logger;
//use jnihook_sys::{JNIHook_Attach, JNIHook_Init};

pub static mut JAVA_VM: Option<jni::JavaVM> = None;
pub static mut ENV: Option<AttachGuard<'static>> = None;
pub static mut CLASS_LOADER: Option<JObject> = None;

pub static mut RUNNING: bool = true;


/*extern "C" fn hk_player_attack(
    jni: *mut JNIEnv,
    callable_method: jmethodID,
    args: *mut jvalue,
    nargs: usize,
    arg: *mut ::std::os::raw::c_void,
) -> jvalue {
    Logger::log("[DH] hk_player_attack called!");
    Logger::log(format!("[DH] Number of args: {}", nargs));
    Logger::log("[DH] Args: {}");
    //Logger::log(format!("[DH] - thisptr: {:?}", );
    //Logger::log(format!("[DH] - level: {}", (*args[1]).l));
    Logger::log("[DH] Calling original Player::Attack...");
    //jni.CallVoidMethod
/*    jni.CallVoidMethod(&(args.wrapping_add(0).l),
                       callable_method,
                       &(args.wrapping_add(1).l));*/
    Logger::log("[DH] Called original Player::Attack");
    jvalue {
        i: 0,
    }
}*/
/*
extern "C" fn hk_player_attack(
    jni: *mut JNIEnv,
    callable_method: jmethodID,
    args: *mut jvalue,
    nargs: usize,
    arg: *mut ::std::os::raw::c_void,
) -> jvalue {
    // Safety: Ensure that the JNI pointer is valid and not null
    let mut jni_env = unsafe {
        jni::JNIEnv::from_raw(jni).expect("Failed to convert JNIEnv from raw pointer")
    };

    // Log that the function was called
    Logger::log("[DH] hk_player_attack called!");
    Logger::log(format!("[DH] Number of args: {}", nargs));

    // Safety: Convert the raw pointer to a slice
    let args_slice = unsafe {
        std::slice::from_raw_parts(args, nargs)
    };

    // Log the arguments safely
    if nargs == 0 {
        Logger::log("[DH]  - thisptr: Not provided");
        return jvalue { i: 0 }; // Return early if this pointer is not provided
    }

    unsafe {
        let this_ptr = args_slice[0].l; // Get the `this` pointer
        Logger::log(format!("[DH]  - thisptr: {:?}", this_ptr));

        if nargs < 2 {
            Logger::log("[DH]  - level: Not provided");
            return jvalue { i: 0 }; // Return early if level pointer is not provided
        }

        let level_ptr = args_slice[1].l; // Get the `level` pointer
        Logger::log(format!("[DH]  - level: {:?}", level_ptr));

        // Convert raw pointers to JObject
        let jobject_arg_obj = JObject::from_raw(this_ptr);
        let level_ptr_obj = JObject::from_raw(level_ptr);

        // Log the conversion of pointers to JObject
        Logger::log(format!("[DH] Converted thisptr to JObject: {:?}", jobject_arg_obj));
        Logger::log(format!("[DH] Converted level_ptr to JObject: {:?}", level_ptr_obj));

        // Create JValue for both arguments
        let jvalue_this = jni::objects::JValue::from(jobject_arg_obj.as_ref());
        let jvalue_level = jni::objects::JValue::from(level_ptr_obj.as_ref());
        Logger::log(format!("[DH] Created JValue for this: {:?}", jvalue_this));
        Logger::log(format!("[DH] Created JValue for level argument: {:?}", jvalue_level));


        let PlayerClass = jni_env.find_class("com/interrupt/dungeoneer/entities/Player").unwrap();
        Logger::log(format!("[DH] PlayerClass pointer: {:?}", PlayerClass));
        if this_ptr.is_null() {
            Logger::log("[DH] this_ptr is null!");
            return jvalue { i: 0 };
        }

        if level_ptr.is_null() {
            Logger::log("[DH] level_ptr is null!");
            return jvalue { i: 0 };
        }

        if PlayerClass.is_null() {
            Logger::log("[DH] PlayerClass is null!");
            return jvalue { i: 0 };
        }
        // Call the original Java method using the call_method method
        let result = jni_env.call_method(
            //&PlayerClass,
            &jobject_arg_obj,  // Convert to JObject
            "Attack",          // Method name
            "(Lcom/interrupt/dungeoneer/game/Level;)V", // Signature (void return type)
            &[jvalue_level],    // Arguments
        );

        match result {
            Ok(_) => {
                Logger::log("[DH] Successfully called Player::Attack");
            }
            Err(err) => {
                Logger::log(format!("[DH] Error calling Player::Attack: {:?}", err));
                // Check for exceptions after the call
                if let Err(exception) = jni_env.exception_occurred() {
                    Logger::log(format!("[DH] Exception occurred while calling Player::Attack! {}", exception));
                    // Clear the exception to prevent further crashes
                    jni_env.exception_clear().
                        expect("Failed to clear exception");
                }
                Logger::log("[DH] Stack trace: Please check the Java side for issues.");
            }
        }
    }

    // Log after calling the original method
    Logger::log("[DH] Called original Player::Attack");

    // Return a default jvalue
    Logger::log("[DH] Returning default jvalue: 0");
    jvalue { i: 0 }
}
*/
/*
extern "C" fn hk_player_attack(
    jni: *mut JNIEnv,
    callable_method: jmethodID,
    args: *mut jvalue,
    nargs: usize,
    arg: *mut ::std::os::raw::c_void,
) -> jvalue {
    // Safety: Ensure that the JNI pointer is valid and not null
    let mut jni_env = unsafe {
        jni::JNIEnv::from_raw(jni).expect("Failed to convert JNIEnv from raw pointer")
    };

    // Log that the function was called
    Logger::log("[DH] hk_player_attack called!");
    Logger::log(format!("[DH] Number of args: {}", nargs));

    // Safety: Convert the raw pointer to a slice
    let args_slice = unsafe {
        std::slice::from_raw_parts(args, nargs)
    };

    // Log the arguments safely
    if nargs < 2 {
        Logger::log("[DH] Not enough arguments provided");
        return jvalue { i: 0 }; // Return early if arguments are not provided
    }

    unsafe {
        let this_ptr = args_slice[0].l; // Get the `this` pointer
        let level_ptr = args_slice[1].l; // Get the `level` pointer

        // Log pointers
        Logger::log(format!("[DH] thisptr: {:?}", this_ptr));
        Logger::log(format!("[DH] level: {:?}", level_ptr));

        // Ensure pointers are valid
        if this_ptr.is_null() {
            Logger::log("[DH] this_ptr is null!");
            return jvalue { i: 0 };
        }

        if level_ptr.is_null() {
            Logger::log("[DH] level_ptr is null!");
            return jvalue { i: 0 };
        }
        let JniEnv_ref = &*jni;
        Logger::log(format!("[DH] Acquired JniEnv_ref {:?}", JniEnv_ref));
        let JNINativeInterf = *(*JniEnv_ref);
        Logger::log("[DH] Acquired JNINativeInterf");
        let CallVoidMethodFunc = JNINativeInterf.CallVoidMethod.unwrap();
        Logger::log(format!("[DH] Acquired func {:p}", CallVoidMethodFunc));
        Logger::log("[DH] Going to invoke CallVoidMethodFunc...");
        CallVoidMethodFunc(jni, this_ptr, callable_method);
        Logger::log("[DH] Invoked CallVoidMethodFunc");
/*        // Convert raw pointers to JObject references
        let jobject_arg_obj = JObject::from_raw(this_ptr);
        let level_ptr_obj = JObject::from_raw(level_ptr);

        // Log the conversion of pointers to JObject
        Logger::log(format!("[DH] Converted thisptr to JObject: {:?}", jobject_arg_obj));
        Logger::log(format!("[DH] Converted level_ptr to JObject: {:?}", level_ptr_obj));

        // Create JValue for both arguments
        let jvalue_this = jni::objects::JValue::from(jobject_arg_obj.as_ref());
        let jvalue_level = jni::objects::JValue::from(level_ptr_obj.as_ref());

        // Call the original Java method
        let result = jni_env.call_method(
            jobject_arg_obj,  // Pass the `this` object
            "Attack",          // Method name
            "(Lcom/interrupt/dungeoneer/game/Level;)V", // Signature
            &[jvalue_level],    // Arguments
        );

        // Handle the result
        match result {
            Ok(_) => {
                Logger::log("[DH] Successfully called Player::Attack");
            }
            Err(err) => {
                Logger::log(format!("[DH] Error calling Player::Attack: {:?}", err));
                // Check for exceptions after the call
                if let Err(exception) = jni_env.exception_occurred() {
                    Logger::log(format!("[DH] Exception occurred while calling Player::Attack! {}", exception));
                    // Clear the exception to prevent further crashes
                    jni_env.exception_clear().expect("Failed to clear exception");
                }
            }
        }*/
    }

    // Log after calling the original method
    Logger::log("[DH] Called original Player::Attack");

    // Return a default jvalue
    jvalue { i: 0 }
}
*/

extern "C" fn hk_player_attack(
    jni: *mut JNIEnv,
    callable_method: jmethodID,
    args: *mut jvalue,
    nargs: usize,
    arg: *mut ::std::os::raw::c_void,
) -> jvalue {
    // Safety: Ensure that the JNI pointer is valid and not null
    let mut jni_env = unsafe {
        jni::JNIEnv::from_raw(jni).expect("Failed to convert JNIEnv from raw pointer")
    };

    // Log that the function was called
    Logger::log("[DH] hk_player_attack called!");
    Logger::log(format!("[DH] Number of args: {}", nargs));

    // Safety: Convert the raw pointer to a slice
    let args_slice = unsafe {
        std::slice::from_raw_parts(args, nargs)
    };

    // Log the arguments safely
    if nargs < 2 {
        Logger::log("[DH] Not enough arguments provided");
        return jvalue { i: 0 }; // Return early if arguments are not provided
    }

    unsafe {
        let this_ptr = args_slice[0].l; // Get the `this` pointer
        let level_ptr = args_slice[1].l; // Get the `level` pointer

        // Log pointers
        Logger::log(format!("[DH] thisptr: {:?}", this_ptr));
        Logger::log(format!("[DH] level: {:?}", level_ptr));

        // Ensure pointers are valid
        if this_ptr.is_null() {
            Logger::log("[DH] this_ptr is null!");
            return jvalue { i: 0 };
        }

        if level_ptr.is_null() {
            Logger::log("[DH] level_ptr is null!");
            return jvalue { i: 0 };
        }

        let JObject_this_ptr = JObject::from_raw(this_ptr);
        //let JObject_level_ptr = JObject::from_raw(level_ptr);
        let JMethodID_callable_method = JMethodID::from_raw(callable_method);
        // Create a jvalue array to hold the arguments
        //*args
        let args_attack: [jvalue; 1] = [
            jvalue {
                l: level_ptr, // Convert JObject to jvalue
            },
        ];
        jni_env.call_method_unchecked(JObject_this_ptr,
                                      JMethodID_callable_method,
                                      ReturnType::Primitive(Primitive::Void),
                                      &args_attack).expect("Failed to call method unchecked");

        /*
        let jni_env_ref = &*jni;
        Logger::log(format!("[DH] Acquired jni_env_ref {:?}", jni_env_ref));
        let jninative_interf = *(*jni_env_ref);
        Logger::log("[DH] Acquired jninative_interf");
        let call_void_method_func = jninative_interf.CallVoidMethod.unwrap();
        Logger::log(format!("[DH] Acquired func {:p}", call_void_method_func));
        Logger::log("[DH] Going to invoke call_void_method_func...");
        Logger::log(format!("callable_method [JMethodID] = {:?}", callable_method));
        call_void_method_func(jni, (*args).l, callable_method);
        Logger::log("[DH] Invoked call_void_method_func");
       */





        /*        // Convert raw pointers to JObject references
                let jobject_arg_obj = JObject::from_raw(this_ptr);
                let level_ptr_obj = JObject::from_raw(level_ptr);

                // Log the conversion of pointers to JObject
                Logger::log(format!("[DH] Converted thisptr to JObject: {:?}", jobject_arg_obj));
                Logger::log(format!("[DH] Converted level_ptr to JObject: {:?}", level_ptr_obj));

                // Create JValue for both arguments
                let jvalue_this = jni::objects::JValue::from(jobject_arg_obj.as_ref());
                let jvalue_level = jni::objects::JValue::from(level_ptr_obj.as_ref());

                // Call the original Java method
                let result = jni_env.call_method(
                    jobject_arg_obj,  // Pass the `this` object
                    "Attack",          // Method name
                    "(Lcom/interrupt/dungeoneer/game/Level;)V", // Signature
                    &[jvalue_level],    // Arguments
                );

                // Handle the result
                match result {
                    Ok(_) => {
                        Logger::log("[DH] Successfully called Player::Attack");
                    }
                    Err(err) => {
                        Logger::log(format!("[DH] Error calling Player::Attack: {:?}", err));
                        // Check for exceptions after the call
                        if let Err(exception) = jni_env.exception_occurred() {
                            Logger::log(format!("[DH] Exception occurred while calling Player::Attack! {}", exception));
                            // Clear the exception to prevent further crashes
                            jni_env.exception_clear().expect("Failed to clear exception");
                        }
                    }
                }*/
    }

    // Log after calling the original method
    Logger::log("[DH] Called original Player::Attack");

    // Return a default jvalue
    jvalue { i: 0 }
}

extern "C" fn hk_take_damage(
    jni: *mut JNIEnv,
    callable_method: jmethodID,
    args: *mut jvalue,
    nargs: usize,
    arg: *mut ::std::os::raw::c_void,
) -> jvalue {
    Logger::log("[DH] hk_take_damage called!");
    Logger::log(format!("[DH] Number of args: {}", nargs));
    Logger::log("[DH] Args: ");

    // Safety: Convert the raw pointer to a slice
    let args_slice = unsafe {
        std::slice::from_raw_parts(args, nargs)
    };

    // Log the arguments safely
/*    if nargs < 4 {
        Logger::log("[DH] Not enough arguments provided");
        return jvalue { i: 0 }; // Return early if arguments are not provided
    }*/
    unsafe {
        let this_ptr = args_slice[0].l; // Get the `this` pointer
        let damage_ptr = args_slice[1].l; // Get the `damage` pointer
        let damage_type_ptr = args_slice[2].l; // Get the `damageType` pointer
        let instigator_ptr = args_slice[3].l; // Get the `instigator` pointer
        Logger::log(format!("[DH]  - this_ptr: {:?}", this_ptr));
        Logger::log(format!("[DH]  - damage_ptr: {:?}", damage_ptr));
        Logger::log(format!("[DH]  - damage_type_ptr: {:?}", damage_type_ptr));
        Logger::log(format!("[DH]  - instigator_ptr: {:?}", instigator_ptr));
    }
    return jvalue { i: 0 };
}

extern "C" fn hk_get_walk_speed(
    jni: *mut JNIEnv,
    callable_method: jmethodID,
    args: *mut jvalue,
    nargs: usize,
    arg: *mut ::std::os::raw::c_void,
) -> jvalue {
    Logger::log("[DH] hk_get_walk_speed called!");
    Logger::log(format!("[DH] Number of args: {}", nargs));
    Logger::log("[DH] Args: ");

    // Safety: Convert the raw pointer to a slice
    let args_slice = unsafe {
        std::slice::from_raw_parts(args, nargs)
    };

    // Log the arguments safely
/*    if nargs < 1 {
        Logger::log("[DH] Not enough arguments provided");
        return jvalue { i: 0 }; // Return early if arguments are not provided
    }*/

    unsafe {
        let this_ptr = args_slice[0].l; // Get the `this` pointer
        Logger::log(format!("[DH]  - this_ptr: {:?}", this_ptr));
    }
    return jvalue { f: 0.75f32 }; // Default speed is lower than this
}

extern "C" fn hk_get_attack_speed_stat_boost(
    jni: *mut JNIEnv,
    callable_method: jmethodID,
    args: *mut jvalue,
    nargs: usize,
    arg: *mut ::std::os::raw::c_void,
) -> jvalue {
    Logger::log("[DH] hk_get_attack_speed_stat_boost called!");
    Logger::log(format!("[DH] Number of args: {}", nargs));
    Logger::log("[DH] Args: ");

    // Safety: Convert the raw pointer to a slice
    let args_slice = unsafe {
        std::slice::from_raw_parts(args, nargs)
    };

    // Log the arguments safely
    /*    if nargs < 1 {
            Logger::log("[DH] Not enough arguments provided");
            return jvalue { i: 0 }; // Return early if arguments are not provided
        }*/

    unsafe {
        let this_ptr = args_slice[0].l; // Get the `this` pointer
        Logger::log(format!("[DH] - this_ptr: {:?}", this_ptr));
    }

    return jvalue { f: 10.0f32 };
}



pub unsafe fn entry() {

    Logger::log("Attached to javaw.exe");

    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open("CONOUT$")
        .unwrap();
    let _ = winapi::um::processenv::SetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE, file.as_raw_handle() as *mut winapi::ctypes::c_void);

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
            JAVA_VM = Some(java_vm);
        }
        Err(err) => {
            Logger::log_fmt(format_args!("Failed to retrieve JavaVM pointer: {:?}", err));
        }
    }
    let env_res = JAVA_VM.as_ref().unwrap().attach_current_thread();
    match env_res
    {
        Ok(env) => {
            ENV = Some(env);
        }
        Err(err) => {
            Logger::log_fmt(format_args!("Failed to retrieve JNI environment: {:?}", err));
        }
    }

    //Logger::log(format!("ENV = {:?}", ENV.unwrap()));
    Logger::log("Set Global vars");
    if let Some(ref vm) = JAVA_VM {
        let b = vm;
        let a = b.get_java_vm_pointer();
        if let Some(ref mut jnienv) = ENV {
            let player_class = jnienv.find_class("com/interrupt/dungeoneer/entities/Player").unwrap();
            let pc_ref:&JClass = player_class.as_ref();
            //TODO fix attackmethod hook
            /*let AttackMethod = jnienv.get_method_id(pc_ref, "Attack", "(Lcom/interrupt/dungeoneer/game/Level;)V").unwrap();*/
            let take_damage_method = jnienv.get_method_id(pc_ref, "takeDamage", "(ILcom/interrupt/dungeoneer/entities/items/Weapon$DamageType;Lcom/interrupt/dungeoneer/entities/Entity;)I").unwrap();
            let get_walk_speed_method = jnienv.get_method_id(pc_ref, "getWalkSpeed", "()F").unwrap();
            let get_attack_speed_stat_boost_method = jnienv.get_method_id(pc_ref, "getAttackSpeedStatBoost", "()F").unwrap();

            let jnihook_init_res = JNIHook_Init(a);
            if jnihook_init_res == JNI_OK
            {
                Logger::log("Successfully initialized JNIHook!");
            }
            else
            {
                Logger::log("Failed to initialize JNIHook!");
            }

            let mut hkresult: jint;
            //TODO fix attackmethod hook
/*            hkresult = JNIHook_Attach(AttackMethod.into_raw(),
                                      Some(hk_player_attack),
                                      std::ptr::null_mut());
            Logger::log(format!("[DH] Player::Attack Hook Result: {}", hkresult));
*/
            hkresult = JNIHook_Attach(take_damage_method.into_raw(),
                                      Some(hk_take_damage),
                                      std::ptr::null_mut());
            Logger::log(format!("[DH] Player::takeDamage Hook Result: {}", hkresult));
            hkresult = JNIHook_Attach(get_walk_speed_method.into_raw(),
                                      Some(hk_get_walk_speed),
                                      std::ptr::null_mut());
            Logger::log(format!("[DH] Player::getWalkSpeed Hook Result: {}", hkresult));
            hkresult = JNIHook_Attach(get_attack_speed_stat_boost_method.into_raw(),
                                      Some(hk_get_attack_speed_stat_boost),
                                      std::ptr::null_mut());
            Logger::log(format!("[DH] Player::getAttackSpeedStatBoost Hook Result: {}", hkresult));
        }
        else {
            // Handle the case where JAVA_VM is None
            Logger::log("[PANIC] JNIENV is None!!!");
        }
        //let mut a: JavaVM = jni::sys::JavaVM::from(b.get_java_vm_pointer()).expect("REASON");

        //JNIHook_Init(&mut a);
        //JNIHook_Init(&mut a);
    } else {
        // Handle the case where JAVA_VM is None
        Logger::log("[PANIC] JAVA_VM is None!!!");
    }
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

    let loop_thread = std::thread::spawn(|| {
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