#!/usr/bin/env python3
"""Turn meta.json lockfile data into extra/version-skew.txt (a table for the book)."""
import json
m = json.load(open("meta.json"))["locks"]
ROWS = [("egui","egui-app"),("iced","iced-counter"),("Slint","slint-app"),("Xilem","xilem-counter"),
        ("Dioxus Native","dioxus-native-app"),("Freya","freya-app"),("Floem","floem-counter"),("gpui","gpui-hello")]
COLS = ["winit","wgpu","taffy","accesskit","parley","cosmic-text"]
def cell(pkg, lib):
    v = sorted(set(m.get(pkg, {}).get(lib, [])))
    return " + ".join(v) if v else "-"
w = [max(len(r[0]) for r in ROWS)] + [max(len(c), *(len(cell(p, c)) for _, p in ROWS)) for c in COLS]
def line(cells): return "  ".join(str(c).ljust(w[i]) for i, c in enumerate(cells)).rstrip()
out = [line([""] + COLS), line(["-" * x for x in w])]
out += [line([n] + [cell(p, c) for c in COLS]) for n, p in ROWS]
open("extra/version-skew.txt", "w").write("\n".join(out) + "\n")
print("\n".join(out))
