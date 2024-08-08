use std::os::raw::{c_int, c_void};

use jni::objects::{JObject, JObjectArray, JValue};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use windows::core::s;

use crate::util::logger::Logger;
use crate::void::ENV;

type GetCreatedJavaVMs = extern "system" fn(*mut *mut c_void, c_int, *mut c_int) -> c_int;

pub fn get_jni_get_created_jvms() -> Option<GetCreatedJavaVMs> {
    let jvm_module = unsafe { GetModuleHandleA(s!("jvm.dll").as_ptr() as *const i8) };
    if jvm_module.is_null() {
        return None;
    }
    let jvm_proc_address = unsafe { GetProcAddress(jvm_module, s!("JNI_GetCreatedJavaVMs").as_ptr() as *const i8) };
    if jvm_proc_address.is_null() {
        return None;
    }
    let get_created_jvm = unsafe { std::mem::transmute(jvm_proc_address) };
    Some(get_created_jvm)
}

pub fn get_created_jvms() -> Option<Vec<*mut c_void>> {
    let get_created_jvms = get_jni_get_created_jvms()?;
    let mut jvm = Vec::with_capacity(1);
    let mut num_vms = 0;
    let status = get_created_jvms(jvm.as_mut_ptr(), jvm.capacity() as c_int, &mut num_vms as *mut c_int);
    if status != 0 {
        return None;
    }
    unsafe { jvm.set_len(num_vms as usize) };
    Some(jvm)
}

pub unsafe fn get_class_loader<'a>() -> Option<JObject<'a>> {
    let stack_traces_map: JObject = match ENV.as_deref_mut().unwrap().call_static_method(
        "java/lang/Thread",
        "getAllStackTraces",
        "()Ljava/util/Map;",
        &[]
    ) {
        Ok(obj) => {
            Logger::log("Successfully retrieved all stack traces.");
            obj.l().unwrap_or_else(|e| {
                Logger::log(format!("Failed to convert stack traces to JObject: {:?}", e));
                JObject::null()
            })
        }
        Err(e) => {
            Logger::log(format!("Failed to call getAllStackTraces: {:?}", e));
            return None;
        }
    };

    let threads_set: JObject = match ENV.as_deref_mut().unwrap().call_method(
        &stack_traces_map,
        "keySet",
        "()Ljava/util/Set;",
        &[]
    ) {
        Ok(obj) => {
            Logger::log("Successfully retrieved the key set from stack traces map.");
            obj.l().unwrap_or_else(|e| {
                Logger::log(format!("Failed to convert key set to JObject: {:?}", e));
                JObject::null()
            })
        }
        Err(e) => {
            Logger::log(format!("Failed to call keySet on stack traces map: {:?}", e));
            return None;
        }
    };

    let threads: JObject = match ENV.as_deref_mut().unwrap().call_method(
        &threads_set,
        "toArray",
        "()[Ljava/lang/Object;",
        &[]
    ) {
        Ok(obj) => {
            Logger::log("Successfully converted key set to array.");
            obj.l().unwrap_or_else(|e| {
                Logger::log(format!("Failed to convert to array: {:?}", e));
                JObject::null()
            })
        }
        Err(e) => {
            Logger::log(format!("Failed to call toArray on threads set: {:?}", e));
            return None;
        }
    };

    let threads_array: JObjectArray = JObjectArray::from(threads);
    let threads_amount: i32 = match ENV.as_deref_mut().unwrap().get_array_length(&threads_array) {
        Ok(len) => {
            Logger::log(format!("Successfully retrieved threads array length: {}", len));
            len
        }
        Err(e) => {
            Logger::log(format!("Failed to get array length: {:?}", e));
            return None;
        }
    };

    let mut class_loader: JObject = JObject::null();

    Logger::log_fmt(format_args!("{}{}", "Threads: ", threads_amount));

    for i in 0..threads_amount {
        println!("Processing thread index: {}", i);
        let thread: JObject<'_> = match ENV.as_deref_mut().unwrap().get_object_array_element(&threads_array, i) {
            Ok(th) => {
                Logger::log(format!("Successfully retrieved thread at index {}", i));
                th
            }
            Err(e) => {
                Logger::log(format!("Failed to get thread at index {}: {:?}", i, e));
                continue; // Skip to the next iteration
            }
        };

        // Check the context class loader
        class_loader = match ENV.as_deref_mut().unwrap().call_method(
            &thread,
            "getContextClassLoader",
            "()Ljava/lang/ClassLoader;",
            &[]
        ) {
            Ok(cl) => {
                Logger::log("Successfully retrieved context class loader.");
                cl.l().unwrap_or_else(|e| {
                    Logger::log(format!("Failed to convert context class loader to JObject: {:?}", e));
                    JObject::null()
                })
            }
            Err(e) => {
                Logger::log(format!("Failed to call getContextClassLoader: {:?}", e));
                continue; // Skip to the next iteration
            }
        };

        if !class_loader.is_null() {
            let class_name = match ENV.as_deref_mut().unwrap().new_string("net.minecraft.client.Minecraft") {
                Ok(name) => {
                    Logger::log("Successfully created string for Minecraft class.");
                    name
                }
                Err(e) => {
                    Logger::log(format!("Failed to create new string: {:?}", e));
                    continue; // Skip to the next iteration
                }
            };

            let minecraft_class = match ENV.as_deref_mut().unwrap().call_method(
                &class_loader,
                "findClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                &[JValue::Object(&*class_name)],
            ) {
                Ok(cls) => {
                    Logger::log("Successfully called findClass on class loader.");
                    cls.l().unwrap_or_else(|e| {
                        Logger::log(format!("Failed to convert Minecraft class to JObject: {:?}", e));
                        JObject::null()
                    })
                }
                Err(e) => {
                    Logger::log(format!("Failed to call findClass: {:?}", e));
                    continue; // Skip to the next iteration
                }
            };

            if !minecraft_class.is_null() {
                Logger::log("Found Minecraft class");
                ENV.as_deref_mut().unwrap().delete_local_ref(minecraft_class).unwrap();
                break; // Exit the loop if the class is found
            }
        }

        ENV.as_deref_mut().unwrap().delete_local_ref(thread).unwrap();
    }

    Some(class_loader)
}