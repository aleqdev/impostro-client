import init, { start } from '../pkg/impostro.js';
async function main() {
   await init('impostro_bg.wasm');
   start();
}
main()