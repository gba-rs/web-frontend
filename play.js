import init, { run_player } from './pkg/gba_web_frontend.js';
async function main() {
   await init({ module_or_path: 'pkg/gba_web_frontend_bg.wasm' });
   run_player();
}
main()
