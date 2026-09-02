const { withPage } = require('./harness');

const PORT = 8767;
const BIOS_PATH = process.env.GBA_BIOS || 'C:/Users/gnoe2/Projects/GBA/gba-resources/bios/gba_bios.bin';
const ROM_PATH = process.env.GBA_ROM || 'C:/Users/gnoe2/Projects/GBA/gba-resources/roms/Advance Wars (USA).gba';

async function main() {
    await withPage(PORT, 'index.html#/', async (page) => {
        await page.evaluate(() => {
            window.__rafCount = 0;
            const realRaf = window.requestAnimationFrame.bind(window);
            window.requestAnimationFrame = (cb) => realRaf((t) => { window.__rafCount++; cb(t); });
        });

        await page.waitForSelector('input[type=file]', { state: 'attached', timeout: 5000 });
        const fileInputs = page.locator('input[type=file]');
        await fileInputs.nth(0).setInputFiles(BIOS_PATH);
        await fileInputs.nth(1).setInputFiles(ROM_PATH);
        await page.waitForTimeout(300);

        await page.click('button:has-text("Play")');
        await page.waitForTimeout(500);
        await page.evaluate(() => { window.__rafCount = 0; });

        const durationMs = 3000;
        await page.waitForTimeout(durationMs);

        const count = await page.evaluate(() => window.__rafCount);
        const fps = count / (durationMs / 1000);
        console.log(`rAF callbacks in ${durationMs}ms: ${count} (~${fps.toFixed(1)} fps)`);
    });
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
