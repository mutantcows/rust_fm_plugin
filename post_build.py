import os
import subprocess

p = subprocess.Popen(["cargo", "build", "--release"], cwd="C:/Users/Andy/Desktop/cpp/PlugInSDK/fm-plugin")
p.wait()


try:
    os.system('taskkill /IM "FileMaker Pro 18 Advanced.exe" /F')
except Exception:
    pass

try:
    open('C:/Users/Andy/Desktop/plugin.log', 'w').close()
except Exception:
    pass

os.replace("C:/Users/Andy/Desktop/cpp/PlugInSDK/fm-plugin/target/release/fm_plugin.dll", "C:/Users/Andy/AppData/Local/FileMaker/FileMaker Pro Advanced/18.0/Extensions/rust_plugin.fmx64")

subprocess.Popen(["C:/Program Files/FileMaker/FileMaker Pro 18 Advanced/FileMaker Pro 18 Advanced.exe"])
