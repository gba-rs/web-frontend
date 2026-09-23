const { withPage } = require('./harness');

const PORT = 8768;
const BIOS_PATH = process.env.GBA_BIOS || 'C:/Users/gnoe2/Projects/GBA/gba-resources/bios/gba_bios.bin';
const ROM_PATH = process.env.GBA_ROM || 'C:/Users/gnoe2/Projects/GBA/gba-resources/roms/Advance Wars (USA).gba';

async function samplePixels(page) {
    return page.evaluate(() => {
        const canvas = document.getElementById('gba-canvas');
        const ctx = canvas.getContext('2d');
        return Array.from(ctx.getImageData(0, 0, canvas.width, canvas.height).data);
    });
}

function differs(a, b) {
    if (a.length !== b.length) return true;
    for (let i = 0; i < a.length; i++) {
        if (a[i] !== b[i]) return true;
    }
    return false;
}

async function main() {
    let ok = false;

    await withPage(PORT, 'index.html#/', async (page) => {
        await page.waitForSelector('input[type=file]', { state: 'attached', timeout: 5000 });

        const fileInputs = page.locator('input[type=file]');
        await fileInputs.nth(0).setInputFiles(BIOS_PATH);
        await fileInputs.nth(1).setInputFiles(ROM_PATH);
        await page.waitForTimeout(500);

        await page.click('button:has-text("Play")');
        await page.waitForTimeout(1000);

        const before = await samplePixels(page);

        await page.click('a:has-text("Debugger")');
        await page.waitForTimeout(1000);

        const stillThere = await page.evaluate(() => !!document.getElementById('gba-canvas'));
        if (!stillThere) {
            console.error('canvas is gone after navigating to the debugger route');
            return;
        }

        const after = await samplePixels(page);
        const stillAnimating = differs(before, after);
        console.log(`canvas still animating after navigating to debugger: ${stillAnimating}`);
        ok = stillAnimating;
    });

    if (!ok) {
        console.error('game did not keep running across a route navigation');
        process.exit(1);
    }
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
