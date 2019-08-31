#!/usr/bin/env python3
# Copyright (c) 2019 Xu Shaohua <xushaohua2016@outlook.com>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import os
import re
import subprocess
import sys


DEFINES = {
    "linux": {
        "aarch64": {
            "compiler": "aarch64-linux-gnu-gcc-8",
            "deb": ["linux-libc-dev-arm64-cross", "gcc-8-aarch64-linux-gnu"],
            "include": "/usr/aarch64-linux-gnu/include",
            "errno": "/usr/aarch64-linux-gnu/include/asm/errno.h",
            "sysno": "/usr/aarch64-linux-gnu/include/asm/unistd.h",
        },
        "arm": {
            "compiler": "arm-linux-gnueabihf-gcc-8",
            "deb": ["linux-libc-dev-armhf-cross", "gcc-8-arm-linux-gnueabihf"],
            "include": "/usr/aarch-linux-gnu/include",
            "errno": "/usr/arm-linux-gnueabihf/include/asm/errno.h",
            "sysno": "/usr/arm-linux-gnueabihf/include/asm/unistd.h",
        },
        "mips": {
            "compiler": "gcc",
            "deb": ["linux-libc-dev-mips-cross", "gcc"],
            "include": "/usr/mips-linux-gnu/include",
            "errno": "/usr/mips-linux-gnu/include/asm/errno.h",
            "sysno": "/usr/mips-linux-gnu/include/asm/unistd.h",
        },
        "mipsle": {
            "compiler": "mipsel-linux-gnu-gcc-8",
            "deb": ["linux-libc-dev-mipsel-cross", "gcc-8-mipsel-linux-gnu"],
            "include": "/usr/mipsel-linux-gnu/include",
            "errno": "/usr/mipsel-linux-gnu/include/asm/errno.h",
            "sysno": "/usr/mipsel-linux-gnu/include/asm/unistd.h",
        },
        "mips64": {
            "compiler": "mips64-linux-gnuabi64-gcc-8",
            "deb": ["linux-libc-dev-mips64-cross", "gcc-8-mips64-linux-gnuabi64"],
            "include": "/usr/mips64-linux-gnuabi64/include",
            "errno": "/usr/mips64-linux-gnuabi64/include/asm/errno.h",
            "sysno": "/usr/mips64-linux-gnuabi64/include/asm/unistd.h",
        },
        "mips64le": {
            "compiler": "mips64el-linux-gnuabi64-gcc-8",
            "deb": ["linux-libc-dev-mips64el-cross", "gcc-8-mips64el-linux-gnuabi64"],
            "include": "/usr/mips64el-linux-gnuabi64/include",
            "errno": "/usr/mips64el-linux-gnuabi64/include/asm/errno.h",
            "sysno": "/usr/mips64el-linux-gnuabi64/include/asm/unistd.h",
        },
        "ppc64": {
            "compiler": "powerpc64-linux-gnu-gcc-8",
            "deb": ["linux-libc-dev-ppc64-cross", "gcc-8-powerpc64-linux-gnu"],
            "include": "/usr/powerpc64-linux-gnu/include",
            "errno": "/usr/powerpc64-linux-gnu/include/asm/errno.h",
            "sysno": "/usr/powerpc64-linux-gnu/include/asm/unistd.h",
        },
        "ppc64le": {
            "compiler": "powerpc64-linux-gnu-gcc-8",
            "deb": ["linux-libc-dev-ppc64el-cross", "gcc-8-powerpc64le-linux-gnu"],
            "include": "/usr/powerpc64le-linux-gnu/include",
            "errno": "/usr/powerpc64le-linux-gnu/include/asm/errno.h",
            "sysno": "/usr/powerpc64le-linux-gnu/include/asm/unistd.h",
        },
        "s390x": {
            "compiler": "s390x-linux-gnu-gcc-8",
            "deb": ["linux-libc-dev-s390x-cross", "gcc-8-s390x-linux-gnu"],
            "include": "/usr/s390x-linux-gnu/include",
            "errno": "/usr/s390x-linux-gnu/include/asm/errno.h",
            "sysno": "/usr/s390x-linux-gnu/include/asm/unistd.h",
        },
        "sparc64": {
            "compiler": "sparc64-linux-gnu-gcc-8",
            "deb": ["linux-libc-dev-sparc64-cross", "gcc-8-sparc64-linux-gnu"],
            "include": "/usr/sparc64-linux-gnu/include",
            "errno": "/usr/sparc64-linux-gnu/include/asm/errno.h",
            "sysno": "/usr/sparc64-linux-gnu/include/asm/unistd.h",
        },
        "x86": {
            "compiler": "i686-linux-gnu-gcc-8",
            "deb": ["linux-libc-dev-i386-cross", "gcc-8-i686-linux-gnu"],
            "include": "/usr/i686-linux-gnu/include",
            "errno": "/usr/i686-linux-gnu/include/asm/errno.h",
            "sysno": "/usr/i686-linux-gnu/include/asm/unistd.h",
        },
        "x86_64": {
            "compiler": "gcc",
            "deb": ["linux-libc-dev-amd64-cross", "gcc"],
            "include": "/usr/x86_64-linux-gnu/include",
            "errno": "/usr/x86_64-linux-gnu/include/asm/errno.h",
            "sysno": "/usr/x86_64-linux-gnu/include/asm/unistd.h",
        },
    },
}

def read_errno(os_name, arch_name):
    compiler = get_compiler(os_name, arch_name)
    header_file = get_errno_header(os_name, arch_name)
    include_dir = get_include_dir(os_name, arch_name)
    cmd = [compiler, "-I", include_dir, "-E", "-dD", header_file]
    p = subprocess.Popen(cmd, stdout=subprocess.PIPE)
    out, err = p.communicate()
    if p.returncode != 0 or err:
        print(err)
        sys.exit(1)
    return parse_errno(out.decode())

def parse_errno(content):
    lines = [
        "",
        "// Code generated by mkerrno_linux.py; DO NOT EDIT.",
        "",
        "pub type Errno = i32;"
        "",
    ]

    errno_pattern = re.compile("^#define E(\w+)\s+(\d+)")
    for line in content.split('\n'):
        m = errno_pattern.match(line)
        if m:
            line = "pub const E{0}: Errno = {1};".format(m.group(1).upper(), m.group(2))
            lines.append(line)
    return lines

def read_sysno(os_name, arch_name):
    compiler = get_compiler(os_name, arch_name)
    header_file = get_sysno_header(os_name, arch_name)
    include_dir = get_include_dir(os_name, arch_name)
    cmd = [compiler, "-I", include_dir, "-E", "-dD", header_file]
    p = subprocess.Popen(cmd, stdout=subprocess.PIPE)
    out, err = p.communicate()
    if p.returncode != 0 or err:
        print(err)
        sys.exit(1)
    return parse_sysno(out.decode())

def parse_sysno(content):
    lines = [
        "",
        "// Code generated by mksysno_linux.py; DO NOT EDIT.",
        "",
        "pub type Sysno = usize;",
        "",
    ]
    pattern = re.compile("^#define __NR_(\w+)\s+(\d+)")
    for line in content.split("\n"):
        # Ignore syscall
        if line.startswith("#define __NR_syscalls"):
            continue

        m = pattern.match(line)
        if m:
            line = "pub const SYS_{0}: Sysno = {1};".format(m.group(1).upper(), m.group(2))
            lines.append(line)
    return lines

def get_compiler(os_name, arch_name):
    return DEFINES[os_name][arch_name]["compiler"]

def get_include_dir(os_name, arch_name):
    return DEFINES[os_name][arch_name]["include"]

def get_errno_header(os_name, arch_name):
    return DEFINES[os_name][arch_name]["errno"]

def get_sysno_header(os_name, arch_name):
    return DEFINES[os_name][arch_name]["sysno"]

def main():
    if len(sys.argv) != 3:
        print("Usage: %s os arch" % sys.argv[0])
        sys.exit(1)
    if sys.argv[1] == "-e":
        with open(sys.argv[2]) as fh:
            content = fh.read()
            print("\n".join(parse_errno(content)))
        return
    elif sys.argv[1] == "-s":
        with open(sys.argv[2]) as fh:
            content = fh.read()
            print("\n".join(parse_sysno(content)))
        return

    os_name = sys.argv[1]
    arch_name = sys.argv[2]
    folder_name = "{0}-{1}".format(os_name, arch_name)
    platform_folder = os.path.join("platform", folder_name)
    #os.makedirs(platform_folder, exist_ok=True)

    errno_lines = read_errno(os_name, arch_name)
    errno_content = "\n".join(errno_lines)
    errno_file = os.path.join(platform_folder, "errno.rs")
    with open(errno_file, "w") as fh:
        fh.write(errno_content)

    sysno_lines = read_sysno(os_name, arch_name)
    sysno_content = "\n".join(sysno_lines)
    sysno_file = os.path.join(platform_folder, "sysno.rs")
    with open(sysno_file, "w") as fh:
        fh.write(sysno_content)

if __name__ == "__main__":
    main()
