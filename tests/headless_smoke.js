const path = require('path');
const { withPage, readCanvasInfo, reportCanvasInfo } = require('./harness');

const PORT = 8765;
const BIOS_PATH = process.env.GBA_BIOS || 'C:/Users/gnoe2/Projects/GBA/gba-resources/bios/gba_bios.bin';
const ROM_PATH = process.env.GBA_ROM || 'C:/Users/gnoe2/Projects/GBA/gba-resources/roms/Advance Wars (USA).gba';

async function main() {
    let ok = false;

    await withPage(PORT, 'index.html#/debug', async (page) => {
        await page.waitForSelector('#inputGroupFile01', { state: 'attached', timeout: 5000 });

        await page.setInputFiles('#inputGroupFile01', BIOS_PATH);
        await page.setInputFiles('#inputGroupFile02', ROM_PATH);
        await page.waitForTimeout(500);

        await page.click('button:has-text("Init Emulator")');
        await page.waitForTimeout(500);

        await page.click('a:has-text("Graphics")');
        await page.waitForTimeout(200);

        await page.click('button:has-text("Go")');
        await page.waitForTimeout(2000);

        const canvasInfo = await readCanvasInfo(page, 'gba-canvas');
        await page.screenshot({ path: path.join(__dirname, 'screenshot.png'), fullPage: true });
        ok = reportCanvasInfo(canvasInfo);
    });

    if (!ok) {
        console.error('debugger canvas did not render');
        process.exit(1);
    }
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
