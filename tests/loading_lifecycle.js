const assert = require('node:assert/strict');
const path = require('node:path');
const { withPage } = require('./harness');

async function main() {
    await withPage(8780, 'index.html#/debug', async page => {
        const errors = [];
        page.on('pageerror', error => errors.push(error.message));
        await page.waitForSelector('#inputGroupFile01', { state: 'attached' });
        await page.evaluate(() => {
            window.draws = 0;
            window.pendingFrames = new Set();
            const request = window.requestAnimationFrame.bind(window);
            const cancel = window.cancelAnimationFrame.bind(window);
            window.requestAnimationFrame = callback => {
                const id = request(time => { window.pendingFrames.delete(id); callback(time); });
                window.pendingFrames.add(id);
                return id;
            };
            window.cancelAnimationFrame = id => { window.pendingFrames.delete(id); cancel(id); };
            const put = CanvasRenderingContext2D.prototype.putImageData;
            CanvasRenderingContext2D.prototype.putImageData = function (...args) {
                if (this.canvas.id === 'gba-canvas2') window.draws++;
                return put.apply(this, args);
            };
        });
        await page.setInputFiles('#inputGroupFile01', process.env.GBA_BIOS || path.resolve(__dirname, '../../gba_bios.bin'));
        await page.setInputFiles('#inputGroupFile02', process.env.GBA_ROM || 'E:/Retro Games/GBA/English/Advance Wars (USA).gba');
        await page.waitForFunction(() => !document.querySelector('#inputGroupFile02').previousElementSibling.textContent.includes('Choose File'));
        await page.getByRole('button', { name: 'Init Emulator', exact: true }).click();
        await page.getByRole('button', { name: 'Go', exact: true }).click();
        await page.waitForFunction(() => window.draws > 5);
        const pending = await page.evaluate(() => window.pendingFrames.size);
        for (let i = 0; i < 5; i++) await page.getByRole('button', { name: 'Go', exact: true }).click();
        assert.equal(await page.evaluate(() => window.pendingFrames.size), pending, 'Go must not create extra loops');
        await page.getByRole('button', { name: 'Init Emulator', exact: true }).click();
        const stopped = await page.evaluate(() => window.draws);
        await page.waitForTimeout(150);
        assert.equal(await page.evaluate(() => window.draws), stopped, 'Init must stop the old emulator');
        await page.getByRole('button', { name: 'Go', exact: true }).click();
        await page.waitForFunction(n => window.draws > n + 5, stopped);
        await page.getByRole('button', { name: 'Stop', exact: true }).click();
        await page.getByRole('button', { name: 'Save State', exact: true }).click();
        await page.waitForFunction(async () => {
            const db = await new Promise((resolve, reject) => {
                const request = indexedDB.open('gba-web-frontend');
                request.onsuccess = () => resolve(request.result);
                request.onerror = () => reject(request.error);
            });
            try {
                return await new Promise((resolve, reject) => {
                    const request = db.transaction('blobs').objectStore('blobs').openCursor();
                    request.onsuccess = () => {
                        const cursor = request.result;
                        if (!cursor) return resolve(false);
                        if (String(cursor.key).startsWith('save-state:')) {
                            window.stateBytes = cursor.value.byteLength;
                            return resolve(true);
                        }
                        cursor.continue();
                    };
                    request.onerror = () => reject(request.error);
                });
            } finally { db.close(); }
        });
        assert.ok(await page.evaluate(() => window.stateBytes < 2 * 1024 * 1024), 'state should exclude ROM and unused memory');
        await page.getByRole('button', { name: 'Load State', exact: true }).click();
        await page.waitForTimeout(300);
        const beforeResume = await page.evaluate(() => window.draws);
        await page.getByRole('button', { name: 'Go', exact: true }).click();
        await page.waitForFunction(n => window.draws > n + 5, beforeResume);
        assert.deepEqual(errors, []);
        console.log('Loop lifecycle and compact save/load passed; state bytes:', await page.evaluate(() => window.stateBytes));
    });
}
main().catch(error => { console.error(error); process.exitCode = 1; });
