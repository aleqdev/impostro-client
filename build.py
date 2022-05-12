import os
import shutil

os.system("wasm-pack build --target web")

os.system("rollup ./src/main.js --format iife --file ./www/index.js")
shutil.copyfile("./pkg/impostro_bg.wasm", "./www/impostro_bg.wasm")
