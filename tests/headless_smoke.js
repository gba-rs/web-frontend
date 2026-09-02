const { chromium } = require('playwright');
const { spawn } = require('child_process');
const path = require('path');

const PORT = 8765;
const ROOT = path.resolve(__dirname, '..');
const BIOS_PATH = process.env.GBA_BIOS || 'C:/Users/gnoe2/Projects/GBA/gba-resources/bios/gba_bios.bin';
const ROM_PATH = process.env.GBA_ROM || 'C:/Users/gnoe2/Projects/GBA/gba-resources/roms/Advance Wars (USA).gba';

function waitForServer(url, timeoutMs) {
    const start = Date.now();
    return new Promise((resolve, reject) => {
        const attempt = () => {
            fetch(url).then(() => resolve()).catch((err) => {
                if (Date.now() - start > timeoutMs) reject(err);
                else setTimeout(attempt, 200);
            });
        };
        attempt();
    });
}

function isBlankCanvas(pixelData) {
    for (let i = 0; i < pixelData.length; i += 4) {
        if (pixelData[i] !== pixelData[0] || pixelData[i + 1] !== pixelData[1] || pixelData[i + 2] !== pixelData[2]) {
            return false;
        }
    }
    return true;
}

async function main() {
    const server = spawn('python', ['-m', 'http.server', String(PORT)], { cwd: ROOT });
    server.stderr.on('data', () => {});

    try {
        await waitForServer(`http://localhost:${PORT}/index.html`, 10000);

        const browser = await chromium.launch();
        const page = await browser.newPage();

        const consoleMessages = [];
        page.on('console', (msg) => consoleMessages.push(`[console.${msg.type()}] ${msg.text()}`));
        page.on('pageerror', (err) => consoleMessages.push(`[pageerror] ${err.message}`));

        await page.goto(`http://localhost:${PORT}/index.html`);
        await page.waitForSelector('#inputGroupFile01', { timeout: 5000 });

        await page.setInputFiles('#inputGroupFile01', BIOS_PATH);
        await page.setInputFiles('#inputGroupFile02', ROM_PATH);
        await page.waitForTimeout(500);

        await page.click('button:has-text("Init Emulator")');
        await page.waitForTimeout(500);

        await page.click('a:has-text("Graphics")');
        await page.waitForTimeout(200);

        await page.click('button:has-text("Go")');
        await page.waitForTimeout(2000);

        const canvasInfo = await page.evaluate(() => {
            const canvas = document.getElementById('gba-canvas');
            if (!canvas) return { found: false };
            const ctx = canvas.getContext('2d');
            const data = ctx.getImageData(0, 0, canvas.width, canvas.height);
            return {
                found: true,
                width: canvas.width,
                height: canvas.height,
                clientWidth: canvas.clientWidth,
                clientHeight: canvas.clientHeight,
                display: getComputedStyle(canvas).display,
                pixels: Array.from(data.data),
            };
        });

        await page.screenshot({ path: path.join(__dirname, 'screenshot.png'), fullPage: true });

        console.log('--- canvas info ---');
        if (!canvasInfo.found) {
            console.log('CANVAS NOT FOUND IN DOM');
        } else {
            console.log(`width=${canvasInfo.width} height=${canvasInfo.height} clientWidth=${canvasInfo.clientWidth} clientHeight=${canvasInfo.clientHeight} display=${canvasInfo.display}`);
            const blank = isBlankCanvas(canvasInfo.pixels);
            console.log(`canvas is ${blank ? 'BLANK (all one color)' : 'NOT blank (contains varied pixel data)'}`);
        }

        console.log('--- console/page messages ---');
        consoleMessages.forEach((m) => console.log(m));

        await browser.close();
    } finally {
        server.kill();
    }
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
