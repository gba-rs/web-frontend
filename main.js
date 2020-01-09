import init, { run_app } from './pkg/gba_web_frontend.js';
async function main() {
   await init('/pkg/gba_web_frontend_bg.wasm');
   run_app();
}
main()