use phf::phf_map;

pub const USER_CODE_LANGUAGES_EXEC_ARGS: phf::Map<&'static str, (&'static str, &'static str)> = phf_map! {
    // "LANG_CODE" => ("EXEC FUNCS", "NAME_FILE")
    "python" => ("timeout 2s /usr/bin/python3 /user_code.py", "/user_code.py"),
    "py" => ("timeout 2s /usr/bin/python3 /user_code.py", "/user_code.py"),
    "rust" => ("/scripts/run_rust.sh", "/user_code.rs"),
    "rs" => ("/scripts/run_rust.sh", "/user_code.rs"),
    "javascript" => ("timeout 2s /usr/bin/node /user_code.js", "/user_code.js"),
    "js" => ("timeout 2s /usr/bin/node /user_code.js", "/user_code.js"),
    "c" => ("/scripts/run_c.sh", "/user_code.c"),
    "cpp" => ("/scripts/run_cpp.sh", "/user_code.cpp"),
    "lua" => ("timeout 2s lua /user_code.lua", "/user_code.lua")
};
