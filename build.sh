
wasm-pack build --target web
[ ! -d "./www/" ] && npm init wasm-app www
rollup ./main.js --format iife --file ./www/index.js
cp ./index.html ./www/index.html
cp ./index.css ./www/index.css
cp ./package.json ./www/package.json
cp ./pkg/impostro_bg.wasm ./www/impostro_bg.wasm