#!/bin/bash
/root/.cargo/bin/rustc /user_code.rs -o /user_program && timeout 2s /user_program