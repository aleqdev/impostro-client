import os
import shutil

os.system("wasm-pack build --target web")

if not os.path.isdir('./www/'):
    os.system("npm init wasm-app www")

os.system("rollup ./main.js --format iife --file ./www/index.js")
shutil.copyfile("./index.html", "./www/index.html")
shutil.copyfile("./index.css", "./www/index.css")
shutil.copyfile("./package.json", "./www/package.json")
shutil.copyfile("./pkg/impostro_bg.wasm", "./www/impostro_bg.wasm")
