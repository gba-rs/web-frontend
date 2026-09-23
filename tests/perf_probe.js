const { withPage } = require('./harness');

const PORT = 8767;
const BIOS_PATH = process.env.GBA_BIOS || 'C:/Users/gnoe2/Projects/GBA/gba-resources/bios/gba_bios.bin';
const ROM_PATH = process.env.GBA_ROM || 'C:/Users/gnoe2/Projects/GBA/gba-resources/roms/Advance Wars (USA).gba';

async function main() {
    await withPage(PORT, 'index.html#/', async (page) => {
        await page.evaluate(() => {
            window.__presentCount = 0;
            const put = CanvasRenderingContext2D.prototype.putImageData;
            CanvasRenderingContext2D.prototype.putImageData = function (...args) {
                if (this.canvas.id === 'gba-canvas2') window.__presentCount++;
                return put.apply(this, args);
            };
        });

        await page.waitForSelector('input[type=file]', { state: 'attached', timeout: 5000 });
        const fileInputs = page.locator('input[type=file]');
        await fileInputs.nth(0).setInputFiles(BIOS_PATH);
        await fileInputs.nth(1).setInputFiles(ROM_PATH);
        await page.waitForTimeout(300);

        await page.click('button:has-text("Play")');
        await page.waitForTimeout(500);
        await page.evaluate(() => { window.__presentCount = 0; });

        const durationMs = 3000;
        await page.waitForTimeout(durationMs);

        const count = await page.evaluate(() => window.__presentCount);
        const fps = count / (durationMs / 1000);
        console.log(`Canvas presentations in ${durationMs}ms: ${count} (~${fps.toFixed(1)} fps)`);
    });
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
