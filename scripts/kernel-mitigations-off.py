#!/usr/bin/env python
"""
Script to disable mitigations on kernels; run from pacman hooks.
Obtiene los nombres de los paquetes desde stdin (usado con NeedsTargets).
"""

import os
import glob
import logging
import shutil
import sys

LOADER_ENTRIES_PATH = "/boot/loader/entries/*.conf"


logging.basicConfig(
    level=logging.INFO,
    format="  %(message)s",
)


def pre_phase():
    kernels = sorted(glob.glob(LOADER_ENTRIES_PATH))
    for kernel in kernels:
        shutil.copy2(kernel, f"{kernel}.bk")


def post_phase():
    kernels = sorted(glob.glob(LOADER_ENTRIES_PATH))

    for kernel in kernels:
        logging.info("==> %s", os.path.splitext(os.path.basename(kernel))[0])

        cfg_mtime = os.path.getmtime(kernel)
        bk_mtime = (
            os.path.getmtime(f"{kernel}.bk") if os.path.exists(f"{kernel}.bk") else 0
        )

        if cfg_mtime <= bk_mtime or bk_mtime == 0:
            logging.debug("    Up to date")
            if bk_mtime != 0:
                os.unlink(f"{kernel}.bk")
        else:
            shutil.move(f"{kernel}.bk", kernel)

            with open(kernel, "r", encoding="utf-8") as f:
                lines = f.readlines()

            changed = False
            for i, line in enumerate(lines):
                if line.startswith("options "):
                    if "split_lock_detect" not in lines[i]:
                        logging.info("    Disabling split lock")
                        lines[i] = lines[i].strip() + " split_lock_detect=off\n"
                        changed = True

                    if "mitigations=off" not in lines[i]:
                        logging.info("    Disabling CPU mitigations")
                        lines[i] = lines[i].strip() + " mitigations=off\n"
                        changed = True

                    break

            if changed:
                with open(kernel, "w", encoding="utf-8") as f_out:
                    f_out.writelines(lines)
            else:
                logging.debug("    Up to date")


if __name__ == "__main__":
    if sys.argv[1] == "post":
        post_phase()
    else:
        pre_phase()
