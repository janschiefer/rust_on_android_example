
#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]

#[no_mangle]
pub fn main() {
    println!("Native rust function called!");
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::JNIEnv;
    use self::jni::objects::JClass;
    use self::jni::sys::jstring;
    use std::ffi::CString;

    #[no_mangle]
    pub unsafe extern fn Java_com_example_myktapp_MainActivity_greeting(env: JNIEnv, _: JClass) -> jstring {
        let world_ptr = CString::new("Hello world from Rust world").unwrap();
        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
        output.into_inner()
    }
}