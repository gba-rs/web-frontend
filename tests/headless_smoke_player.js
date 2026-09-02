const path = require('path');
const { withPage, readCanvasInfo, reportCanvasInfo } = require('./harness');

const PORT = 8766;
const BIOS_PATH = process.env.GBA_BIOS || 'C:/Users/gnoe2/Projects/GBA/gba-resources/bios/gba_bios.bin';
const ROM_PATH = process.env.GBA_ROM || 'C:/Users/gnoe2/Projects/GBA/gba-resources/roms/Advance Wars (USA).gba';

async function main() {
    let ok = false;

    await withPage(PORT, 'index.html#/', async (page) => {
        await page.waitForSelector('input[type=file]', { state: 'attached', timeout: 5000 });

        const fileInputs = page.locator('input[type=file]');
        await fileInputs.nth(0).setInputFiles(BIOS_PATH);
        await fileInputs.nth(1).setInputFiles(ROM_PATH);
        await page.waitForTimeout(500);

        await page.click('button:has-text("Play")');
        await page.waitForTimeout(2000);

        const canvasInfo = await readCanvasInfo(page, 'gba-canvas');
        await page.screenshot({ path: path.join(__dirname, 'screenshot_player.png'), fullPage: true });
        ok = reportCanvasInfo(canvasInfo);
    });

    if (!ok) {
        console.error('player canvas did not render');
        process.exit(1);
    }
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
