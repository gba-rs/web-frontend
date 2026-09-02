const { chromium } = require('playwright');
const { spawn } = require('child_process');
const path = require('path');

const ROOT = path.resolve(__dirname, '..');

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

async function withPage(port, pagePath, run) {
    const server = spawn('python', ['-m', 'http.server', String(port)], { cwd: ROOT });
    server.stderr.on('data', () => {});

    try {
        await waitForServer(`http://localhost:${port}/${pagePath}`, 10000);

        const browser = await chromium.launch();
        const page = await browser.newPage();

        const consoleMessages = [];
        page.on('console', (msg) => consoleMessages.push(`[console.${msg.type()}] ${msg.text()}`));
        page.on('pageerror', (err) => consoleMessages.push(`[pageerror] ${err.message}`));

        await page.goto(`http://localhost:${port}/${pagePath}`);

        await run(page);

        console.log('--- console/page messages ---');
        consoleMessages.forEach((m) => console.log(m));

        await browser.close();
    } finally {
        server.kill();
    }
}

async function readCanvasInfo(page, canvasId) {
    return page.evaluate((id) => {
        const canvas = document.getElementById(id);
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
    }, canvasId);
}

function reportCanvasInfo(canvasInfo) {
    console.log('--- canvas info ---');
    if (!canvasInfo.found) {
        console.log('CANVAS NOT FOUND IN DOM');
        return false;
    }
    console.log(`width=${canvasInfo.width} height=${canvasInfo.height} clientWidth=${canvasInfo.clientWidth} clientHeight=${canvasInfo.clientHeight} display=${canvasInfo.display}`);
    const blank = isBlankCanvas(canvasInfo.pixels);
    console.log(`canvas is ${blank ? 'BLANK (all one color)' : 'NOT blank (contains varied pixel data)'}`);
    return !blank;
}

module.exports = { ROOT, waitForServer, isBlankCanvas, withPage, readCanvasInfo, reportCanvasInfo };
