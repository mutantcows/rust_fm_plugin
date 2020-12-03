import os
import subprocess

try:
    os.system('taskkill /IM "FileMaker Pro.exe" /F')
except Exception:
    pass


p = subprocess.Popen(["cargo", "build", "--release"], cwd="C:/Users/andy/Desktop/fm-plugin")
p.wait()


try:
    open('C:/Users/Andy/Desktop/plugin.log', 'w').close()
except Exception:
    pass

os.replace("C:/Users/andy/Desktop/fm-plugin/target/release/fm_plugin.dll", "C:/Users/andy/AppData/Local/FileMaker/FileMaker Pro/19.0/Extensions/rust_plugin.fmx64")

subprocess.Popen(["C:/Program Files/FileMaker/FileMaker Pro 19/FileMaker Pro.exe"])
