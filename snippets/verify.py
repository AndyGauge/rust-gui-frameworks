#!/usr/bin/env python3
"""Re-verify every snippet and regenerate meta.json.

For each package: cargo check (host, plus wasm32 for the web ones), record
the versions of the shared layers in its Cargo.lock, and capture output of
the headless examples. Run from this directory:  python3 verify.py
"""
import json, os, re, subprocess, sys, datetime

WATCH = ["winit","wgpu","taffy","accesskit","parley","cosmic-text","vello","skrifa","swash",
         "tiny-skia","raw-window-handle","kurbo","peniko","harfrust","fontdb","glow","glutin","wry","tao"]
WASM = {"yew-counter","leptos-counter","wasm-bindgen-dom","wgpu-web"}
RUN = {
    "semver-rust1": ["one","two"], "wgpu-device": ["one","two"], "taffy-flex": ["one","two"],
    "accesskit-tree": ["one","two"], "cosmic-shape": ["one","two"], "parley-layout": ["one","two"],
    "vello-scene": ["one"], "glutin-gl": ["two"], "egui-app": ["two"],
}
def sh(cmd, cwd):
    return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)

meta, matrix = {}, {}
for d in sorted(os.listdir(".")):
    if not os.path.isfile(f"{d}/Cargo.toml"): continue
    args = ["cargo","check","--quiet"] + ([] if d in ("tauri-app","slint-file","wgpu-web") else ["--bins"])
    ok = sh(args, d).returncode == 0
    if d in WASM:
        ok = ok and sh(args + ["--target","wasm32-unknown-unknown"], d).returncode == 0
    lock = open(f"{d}/Cargo.lock").read() if os.path.exists(f"{d}/Cargo.lock") else ""
    vers = {}
    for name, ver in re.findall(r'name = "([^"]+)"\nversion = "([^"]+)"', lock):
        if name in WATCH: vers.setdefault(name, []).append(ver)
    direct = re.findall(r'^([a-z0-9_-]+)\s*=\s*(?:"([^"]+)"|\{[^}]*version\s*=\s*"([^"]+)")', open(f"{d}/Cargo.toml").read().split("[dependencies]")[1].split("[workspace]")[0], re.M)
    outs = {}
    for b in RUN.get(d, []):
        r = sh(["cargo","run","--quiet","--bin",b], d)
        outs[b] = (r.stdout + r.stderr).strip() if r.returncode == 0 else None
    meta[d] = {"ok": ok, "deps": {n:(v1 or v2) for n,v1,v2 in direct}, "output": outs}
    matrix[d] = vers
    print(d, "ok" if ok else "FAILED", flush=True)

trees = {}
for member, crate, out in [("dioxus-native-app","taffy","taffy-dioxus-native"),("gpui-hello","taffy","taffy-gpui"),("floem-counter","taffy","taffy-floem")]:
    r = sh(["cargo","tree","-i",crate,"--depth","3","-e","normal"], member)
    open(f"extra/{out}.txt","w").write(r.stdout.strip()+"\n")

json.dump({"checked": datetime.date.today().isoformat(), "rustc": sh(["rustc","--version"],".").stdout.strip(),
           "packages": meta, "locks": matrix}, open("meta.json","w"), indent=1, sort_keys=True)
bad = [k for k,v in meta.items() if not v["ok"]]
print("FAILED:" , bad) if bad else print("all packages compile")
sys.exit(1 if bad else 0)
import subprocess; subprocess.run(["python3", "skew.py"], check=True)
