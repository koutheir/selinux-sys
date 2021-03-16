use std::path::{Path, PathBuf};
use std::{env, io};

fn main() {
    let target =
        env::var("TARGET").expect("selinux-sys: Environment variable 'TARGET' was not defined.");

    let out_dir = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .expect("selinux-sys: Environment variable 'OUT_DIR' was not defined.");

    println!("cargo:root={}", out_dir.to_str().unwrap());

    let lib = pkg_config::Config::new().probe("libselinux").unwrap();

    output_lib_dir(&lib.link_paths, &target);
    let include_path = output_include_dir(&lib.include_paths);

    generate_bindings(&out_dir, &include_path)
}

fn output_include_dir(include_paths: &[PathBuf]) -> PathBuf {
    let include_path = find_file_in_dirs("selinux/selinux.h", include_paths).unwrap();
    println!(
        "cargo:include={}/selinux/selinux.h",
        include_path.to_str().unwrap()
    );
    include_path
}

fn output_lib_dir(link_paths: &[PathBuf], target: &str) {
    if let Ok(lib_path) = find_file_in_dirs("libselinux.so", link_paths) {
        println!("cargo:lib={}/libselinux.so", lib_path.to_str().unwrap());
    } else if let Ok(lib_path) = find_file_in_dirs("libselinux.a", link_paths) {
        println!("cargo:lib={}/libselinux.a", lib_path.to_str().unwrap());
    } else if let Some(link_path) = link_paths.get(0) {
        let triplet = target.replace("-unknown-", "-").replace("-none-", "-");

        for path in &[
            link_path.join("libselinux.so"),
            link_path.join(&target).join("libselinux.so"),
            link_path.join(&triplet).join("libselinux.so"),
            link_path.join("libselinux.a"),
            link_path.join(&target).join("libselinux.a"),
            link_path.join(&triplet).join("libselinux.a"),
        ] {
            if path.exists() {
                println!("cargo:lib={}", path.to_str().unwrap());
                break;
            }
        }
    }
}

fn generate_bindings(out_dir: &Path, include_path: &Path) {
    let mut builder = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .size_t_is_usize(true)
        .derive_debug(true)
        .derive_copy(true)
        .impl_debug(true);

    // Do not expose deprecated functions.
    for &blacklisted_function in &[
        "(avc_init|security_load_booleans|checkPasswdAccess|rpm_execcon)",
        "selinux_(booleans_path|users_path|check_passwd_access)",
        "security_compute_user(_raw)?",
        "sid(get|put)",
        "matchpathcon(_init|_init_prefix|_fini|_index)?",
    ] {
        builder = builder.blacklist_function(blacklisted_function);
    }

    // Do not expose deprecated types.
    builder = builder.blacklist_type("security_context_t");

    // Expose documented functions.
    for &function in &[
        "(f|l)?(g|s)et(file|exec|fscreate|keycreate|sockcreate|peer|pid|prev)?con(_raw)?",
        "freecon(ary)?",
        "(set_)?match(path|media)con(_.+)?",
        "(is_)?(security|selabel|selinux|avc|context)_.+",
        "(init|fini|set)_selinuxmnt",
        "get_(default|ordered)_(context|type).*",
        "(string|mode)_to_(security_class|av_perm)",
        "getseuserbyname",
        "manual_user_enter_context",
        "print_access_vector",
        "query_user_context",
    ] {
        builder = builder.whitelist_function(function);
    }

    // Expose documented types and constants.
    builder = builder
        .whitelist_type("(security|selinux|access|av|avc|SEL)_.+")
        .whitelist_var("(SELINUX|SELABEL|MATCHPATHCON|SECSID|AVC)_.+");

    // Include all SELinux headers.
    for &header in &[
        "selinux/selinux.h",
        "selinux/context.h",
        "selinux/avc.h",
        "selinux/label.h",
        "selinux/restorecon.h",
        "selinux/get_context_list.h",
        "selinux/get_default_type.h",
    ] {
        builder = builder.header(include_path.join(header).to_str().unwrap());
    }

    let bindings = builder.generate().expect(
        "selinux-sys: Failed to generate Rust bindings for 'selinux/selinux.h' and other headers.",
    );

    bindings
        .write_to_file(out_dir.join("selinux-sys.rs"))
        .expect("selinux-sys: Failed to write 'selinux-sys.rs'.")
}

fn find_file_in_dirs(path_suffix: &str, dirs: &[PathBuf]) -> io::Result<PathBuf> {
    for dir in dirs {
        if dir.join(path_suffix).exists() {
            return Ok(dir.clone());
        }
    }

    Err(io::ErrorKind::NotFound.into())
}
