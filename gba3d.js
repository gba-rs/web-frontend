(function () {
    var OUTLINE = [[348,45],[296,57],[244,62],[184,87],[100,112],[84,121],[56,229],[38,348],[33,407],[35,426],[44,444],[58,458],[78,469],[242,527],[368,554],[601,554],[649,542],[709,533],[754,521],[896,467],[923,453],[939,437],[950,404],[944,336],[922,209],[897,116],[875,103],[770,73],[730,57],[715,45]];

    var IMG_W = 972;
    var IMG_H = 573;
    var SCREEN = { x: 282, y: 125, w: 405, h: 270 };
    var BEZEL = { x: 245, y: 86, w: 483, h: 411 };

    var WORLD_W = 3.0;
    var SCALE = WORLD_W / IMG_W;
    var DEPTH = 24.5 / 144.5 * WORLD_W;
    var MM = WORLD_W / 144.5;

    function toWorld(px, py) {
        return [(px - IMG_W / 2) * SCALE, (IMG_H / 2 - py) * SCALE];
    }

    function roundedRect(w, h, r) {
        r = Math.min(r, w / 2, h / 2);
        var s = new THREE.Shape();
        s.moveTo(-w / 2 + r, -h / 2);
        s.lineTo(w / 2 - r, -h / 2);
        s.quadraticCurveTo(w / 2, -h / 2, w / 2, -h / 2 + r);
        s.lineTo(w / 2, h / 2 - r);
        s.quadraticCurveTo(w / 2, h / 2, w / 2 - r, h / 2);
        s.lineTo(-w / 2 + r, h / 2);
        s.quadraticCurveTo(-w / 2, h / 2, -w / 2, h / 2 - r);
        s.lineTo(-w / 2, -h / 2 + r);
        s.quadraticCurveTo(-w / 2, -h / 2, -w / 2 + r, -h / 2);
        return s;
    }

    // The bevel adds to `depth`, so inset it to keep total thickness exact.
    function slab(w, h, r, thickness, material) {
        var bt = thickness * 0.18;
        var geo = new THREE.ExtrudeGeometry(roundedRect(w - 2 * bt, h - 2 * bt, Math.max(r - bt, 0.001)), {
            depth: Math.max(thickness - 2 * bt, 0.0001), bevelEnabled: true, bevelThickness: bt,
            bevelSize: bt, bevelSegments: 2, curveSegments: 8
        });
        geo.computeBoundingBox();
        geo.translate(0, 0, -(geo.boundingBox.max.z + geo.boundingBox.min.z) / 2);
        return new THREE.Mesh(geo, material);
    }

    function plate(wpx, hpx, rpx, thickness, material) {
        return slab(wpx * SCALE, hpx * SCALE, rpx * SCALE, thickness, material);
    }

    function place(mesh, cxpx, cypx, z) {
        var c = toWorld(cxpx, cypx);
        mesh.position.set(c[0], c[1], z);
        return mesh;
    }

    function screenMesh(sourceCanvas, w, h) {
        var tex = new THREE.CanvasTexture(sourceCanvas);
        tex.minFilter = THREE.LinearFilter;
        tex.magFilter = THREE.NearestFilter;
        tex.wrapS = THREE.ClampToEdgeWrapping;
        tex.wrapT = THREE.ClampToEdgeWrapping;
        var mesh = new THREE.Mesh(
            new THREE.PlaneGeometry(w, h),
            new THREE.MeshBasicMaterial({ map: tex })
        );
        mesh.userData.tex = tex;
        return mesh;
    }

    function lettering(text, w, h, color, italic) {
        var canvas = document.createElement('canvas');
        canvas.width = 1024; canvas.height = 128;
        var ctx = canvas.getContext('2d');
        ctx.fillStyle = color;
        ctx.font = (italic ? 'italic ' : '') + 'bold 90px Arial, sans-serif';
        ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
        ctx.translate(512, 68);
        ctx.scale(1000 / Math.max(1, ctx.measureText(text).width), 1);
        ctx.fillText(text, 0, 0);
        var texture = new THREE.CanvasTexture(canvas);
        return new THREE.Mesh(new THREE.PlaneGeometry(w, h), new THREE.MeshBasicMaterial({map: texture, transparent: true, depthWrite: false}));
    }

    function buildGba(sourceCanvas) {
        var g = new THREE.Group();

        var shell = new THREE.MeshStandardMaterial({ color: 0x5b64ad, roughness: 0.45, metalness: 0.06 });
        var shellDark = new THREE.MeshStandardMaterial({ color: 0x474f8c, roughness: 0.5, metalness: 0.05 });
        var bezelMat = new THREE.MeshStandardMaterial({ color: 0x1b1d24, roughness: 0.35, metalness: 0.1 });
        var button = new THREE.MeshStandardMaterial({ color: 0xb3b6c3, roughness: 0.5, metalness: 0.05 });
        var labelMat = new THREE.MeshStandardMaterial({ color: 0x9297a8, roughness: 0.88 });
        var screwMat = new THREE.MeshStandardMaterial({ color: 0x6f6a55, roughness: 0.4, metalness: 0.6 });

        var shape = new THREE.Shape();
        for (var i = 0; i < OUTLINE.length; i++) {
            var prev = OUTLINE[(i + OUTLINE.length - 1) % OUTLINE.length];
            var curr = OUTLINE[i];
            var next = OUTLINE[(i + 1) % OUTLINE.length];
            var a = toWorld((prev[0] + curr[0]) / 2, (prev[1] + curr[1]) / 2);
            var b = toWorld(curr[0], curr[1]);
            var c = toWorld((curr[0] + next[0]) / 2, (curr[1] + next[1]) / 2);
            if (i === 0) shape.moveTo(a[0], a[1]);
            shape.quadraticCurveTo(b[0], b[1], c[0], c[1]);
        }
        shape.closePath();

        var geo = new THREE.ExtrudeGeometry(shape, {
            depth: DEPTH - 0.08, bevelEnabled: true, bevelThickness: 0.04,
            bevelSize: 0.04, bevelSegments: 4, curveSegments: 6
        });
        geo.computeBoundingBox();
        geo.translate(0, 0, -(geo.boundingBox.max.z + geo.boundingBox.min.z) / 2);
        geo.computeBoundingBox();
        var frontZ = geo.boundingBox.max.z;
        var backZ = geo.boundingBox.min.z;
        g.add(new THREE.Mesh(geo, shell));

        var bezel = plate(BEZEL.w, BEZEL.h, 40, 0.018, bezelMat);
        place(bezel, BEZEL.x + BEZEL.w / 2, BEZEL.y + BEZEL.h / 2, frontZ - 0.004);
        g.add(bezel);

        var scr = screenMesh(sourceCanvas, SCREEN.w * SCALE, SCREEN.h * SCALE);
        place(scr, SCREEN.x + SCREEN.w / 2, SCREEN.y + SCREEN.h / 2, frontZ + 0.014);
        g.add(scr);
        g.userData.screen = scr;

        var recess = new THREE.Mesh(new THREE.CylinderGeometry(88 * SCALE, 88 * SCALE, 0.012, 32), shellDark);
        recess.rotation.x = Math.PI / 2;
        place(recess, 150, 232, frontZ + 0.004);
        g.add(recess);

        g.add(place(plate(46, 138, 8, 0.03, button), 150, 232, frontZ + 0.008));
        g.add(place(plate(138, 46, 8, 0.03, button), 150, 232, frontZ + 0.008));

        [[860, 210], [786, 244]].forEach(function (p) {
            var b = new THREE.Mesh(new THREE.CylinderGeometry(34 * SCALE, 34 * SCALE, 0.04, 28), button);
            b.rotation.x = Math.PI / 2;
            g.add(place(b, p[0], p[1], frontZ + 0.012));
        });

        [[190, 368], [192, 418]].forEach(function (p) {
            var s = new THREE.Mesh(new THREE.CylinderGeometry(13 * SCALE, 13 * SCALE, 0.022, 24), button);
            s.rotation.x = Math.PI / 2;
            g.add(place(s, p[0], p[1], frontZ + 0.012));
        });
        [['START', 134, 351], ['SELECT', 134, 401]].forEach(function (p) {
            var label = lettering(p[0], 76 * SCALE, 18 * SCALE, '#a5a9cf');
            place(label, p[1], p[2], frontZ + 0.012);
            label.rotation.z = -0.25;
            g.add(label);
        });
        [['A',860,210], ['B',786,244]].forEach(function (p) {
            g.add(place(lettering(p[0], 28 * SCALE, 32 * SCALE, '#777b92'), p[1], p[2], frontZ + 0.034));
        });
        g.add(place(lettering('GAME BOY ADVANCE', 340 * SCALE, 30 * SCALE, '#d4d5dd', true), 486, 455, frontZ + 0.016));
        g.add(place(lettering('Nintendo', 100 * SCALE, 19 * SCALE, '#b0b1d5'), 486, 62, frontZ + 0.016));
        g.add(place(lettering('POWER', 62 * SCALE, 15 * SCALE, '#a5a9cf'), 823, 113, frontZ + 0.014));

        for (var j = 0; j < 6; j++) {
            var slat = place(plate(70, 6, 3, 0.01, shellDark), 798, 304 + j * 16, frontZ + 0.003);
            slat.rotation.z = 0.26;
            g.add(slat);
        }

        var led = new THREE.Mesh(
            new THREE.CylinderGeometry(8 * SCALE, 8 * SCALE, 0.02, 16),
            new THREE.MeshStandardMaterial({ color: 0x2b7d2b, emissive: 0x36d436, emissiveIntensity: 0.9, roughness: 0.3 })
        );
        led.rotation.x = Math.PI / 2;
        g.add(place(led, 774, 113, frontZ + 0.008));

        g.add(place(plate(108, 22, 11, 0.008, shellDark), 486, 62, frontZ + 0.003));

        // L/R sit on the top edge, set back in Z so the shell hides them head-on
        [[180, 104, 0.29], [792, 104, -0.29]].forEach(function (p) {
            var sb = new THREE.Mesh(
                new THREE.BoxGeometry(196 * SCALE, 48 * SCALE, DEPTH * 0.70), button
            );
            var c = toWorld(p[0], p[1]);
            sb.position.set(c[0], c[1], -DEPTH * 0.13);
            sb.rotation.z = p[2];
            g.add(sb);
        });

        function backPart(mesh, cxpx, cypx, z) {
            place(mesh, cxpx, cypx, z);
            mesh.rotation.y = Math.PI;
            g.add(mesh);
        }
        backPart(plate(430, 108, 12, 0.02, bezelMat), 486, 142, backZ + 0.012);
        backPart(plate(470, 78, 8, 0.006, labelMat), 486, 288, backZ - 0.002);
        backPart(plate(330, 208, 20, 0.016, shellDark), 486, 418, backZ - 0.004);
        backPart(plate(62, 20, 8, 0.02, button), 486, 318, backZ - 0.012);

        [[150, 178], [822, 178], [86, 432], [886, 432], [300, 528], [672, 528]].forEach(function (p) {
            var screw = new THREE.Mesh(new THREE.CylinderGeometry(10 * SCALE, 10 * SCALE, 0.012, 12), screwMat);
            screw.rotation.x = Math.PI / 2;
            g.add(place(screw, p[0], p[1], backZ - 0.004));
        });

        return g;
    }

    function buildSp(sourceCanvas) {
        var g = new THREE.Group();

        var W = 82 * MM;
        var D = 82 * MM;
        var T = 12 * MM;

        var shell = new THREE.MeshStandardMaterial({ color: 0x9fa3b2, roughness: 0.62, metalness: 0.18 });
        var shellDark = new THREE.MeshStandardMaterial({ color: 0x7c8091, roughness: 0.68, metalness: 0.14 });
        var bezelMat = new THREE.MeshStandardMaterial({ color: 0x23252c, roughness: 0.32, metalness: 0.1 });
        var button = new THREE.MeshStandardMaterial({ color: 0x5f6472, roughness: 0.6, metalness: 0.12 });
        var hingeMat = new THREE.MeshStandardMaterial({ color: 0xb0b4c0, roughness: 0.45, metalness: 0.35 });

        var base = slab(W, D, 7 * MM, T, shell);
        base.rotation.x = -Math.PI / 2;
        base.position.y = T / 2;
        g.add(base);

        var topY = T + 0.001;
        function onBase(mesh, xmm, zmm, lift) {
            mesh.position.set(xmm * MM, topY + (lift || 0), zmm * MM);
            g.add(mesh);
            return mesh;
        }
        function flat(mesh) { mesh.rotation.x = -Math.PI / 2; return mesh; }

        var dRecess = new THREE.Mesh(new THREE.CylinderGeometry(13 * MM, 13 * MM, 1.2 * MM, 28), shellDark);
        onBase(dRecess, -23, -9, 0);
        onBase(flat(slab(7.5 * MM, 20 * MM, 1.6 * MM, 2.6 * MM, button)), -23, -9, 1.4 * MM);
        onBase(flat(slab(20 * MM, 7.5 * MM, 1.6 * MM, 2.6 * MM, button)), -23, -9, 1.4 * MM);

        [[27, -14], [16.5, -5]].forEach(function (p) {
            var b = new THREE.Mesh(new THREE.CylinderGeometry(5.4 * MM, 5.4 * MM, 2.6 * MM, 24), button);
            onBase(b, p[0], p[1], 1.0 * MM);
        });

        [[-7, 30], [7, 30]].forEach(function (p) {
            var b = new THREE.Mesh(new THREE.CylinderGeometry(3.2 * MM, 3.2 * MM, 1.8 * MM, 20), button);
            onBase(b, p[0], p[1], 0.6 * MM);
        });

        for (var r = 0; r < 4; r++) {
            for (var c = 0; c < 4; c++) {
                var dot = new THREE.Mesh(new THREE.CylinderGeometry(0.55 * MM, 0.55 * MM, 0.15 * MM, 8), bezelMat);
                onBase(dot, -6 + c * 4, 6 + r * 4, 0);
            }
        }

        var lightButton = new THREE.Mesh(new THREE.CylinderGeometry(2.5 * MM, 2.5 * MM, 1.0 * MM, 24), button);
        onBase(lightButton, 0, -27, 0.8 * MM);
        onBase(flat(lettering('?', 3.5 * MM, 3.5 * MM, '#353945')), 0, -27, 1.4 * MM);
        [['SELECT',-7,23], ['START',7,23], ['A',27,-14], ['B',16.5,-5]].forEach(function (p) {
            onBase(flat(lettering(p[0], (p[0].length > 1 ? 9 : 4) * MM, 2.6 * MM, '#c4c6ce')), p[1], p[2], 2.4 * MM);
        });
        var abRecess = flat(slab(28 * MM, 15 * MM, 7 * MM, 0.8 * MM, shellDark));
        abRecess.rotation.z = 0.70;
        onBase(abRecess, 21.75, -9.5, 0);
        [0x48a836, 0x6d5231].forEach(function (color, i) {
            var indicator = new THREE.Mesh(new THREE.BoxGeometry(1.3 * MM, 1.5 * MM, 2 * MM), new THREE.MeshBasicMaterial({color: color}));
            onBase(indicator, 40, -29 + i * 6, -0.5 * MM);
        });
        var volume = new THREE.Mesh(new THREE.BoxGeometry(2 * MM, 3 * MM, 8 * MM), button);
        volume.position.set(-W / 2, T * 0.55, 13 * MM);
        g.add(volume);

        var slot = new THREE.Mesh(new THREE.BoxGeometry(58 * MM, 5 * MM, 2 * MM), bezelMat);
        slot.position.set(0, T * 0.45, D / 2 - 0.5 * MM);
        g.add(slot);

        [-1, 1].forEach(function (sx) {
            var sb = new THREE.Mesh(new THREE.BoxGeometry(15 * MM, T * 0.75, 7 * MM), button);
            sb.position.set(sx * (W / 2 - 9 * MM), T * 0.55, -D / 2 + 3.5 * MM);
            g.add(sb);
        });

        var pwr = new THREE.Mesh(new THREE.BoxGeometry(2 * MM, T * 0.45, 9 * MM), button);
        pwr.position.set(W / 2 - 0.4 * MM, T * 0.55, 12 * MM);
        g.add(pwr);

        var hingeY = T;
        var hingeZ = -D / 2 + 5 * MM;
        [[-26, 20], [0, 26], [26, 20]].forEach(function (h) {
            var bar = new THREE.Mesh(new THREE.CylinderGeometry(5 * MM, 5 * MM, h[1] * MM, 20), hingeMat);
            bar.rotation.z = Math.PI / 2;
            bar.position.set(h[0] * MM, hingeY, hingeZ);
            g.add(bar);
        });

        var pivot = new THREE.Object3D();
        pivot.position.set(0, hingeY, hingeZ);
        pivot.rotation.x = -2.02;
        g.add(pivot);

        var lidZ = D / 2 - 5 * MM;
        var lidThickness = 6 * MM;
        var lid = slab(W, D, 7 * MM, lidThickness, shell);
        lid.rotation.x = -Math.PI / 2;
        lid.position.set(0, lidThickness / 2, lidZ);
        pivot.add(lid);

        function onLid(mesh, xmm, zmm, drop) {
            mesh.rotation.x = Math.PI / 2;
            mesh.position.set(xmm * MM, -(0.6 * MM + (drop || 0)), lidZ + zmm * MM);
            pivot.add(mesh);
            return mesh;
        }

        onLid(slab(72 * MM, 62 * MM, 3 * MM, 1.2 * MM, bezelMat), 0, 0, 0);
        var spScreen = screenMesh(sourceCanvas, 61.2 * MM, 40.8 * MM);
        spScreen.rotation.x = Math.PI / 2;
        spScreen.position.set(0, -(1.4 * MM), lidZ + 5 * MM);
        pivot.add(spScreen);

        onLid(lettering('GAME BOY ADVANCE SP', 45 * MM, 3 * MM, '#b9bdc4', true), 0, -24, 1.0 * MM);

        [[-34, 34], [0, 34], [34, 34], [-34, -33], [34, -33]].forEach(function (p) {
            var foot = new THREE.Mesh(new THREE.CylinderGeometry(2.6 * MM, 2.6 * MM, 0.8 * MM, 14), shellDark);
            foot.position.set(p[0] * MM, -(0.4 * MM), lidZ + p[1] * MM);
            pivot.add(foot);
        });

        g.position.y = -0.35;
        g.userData.lidPivot = pivot;
        g.userData.screen = spScreen;
        return g;
    }

    function build(host, sourceCanvas, container) {
        var renderer = new THREE.WebGLRenderer({ canvas: host, antialias: true, alpha: true });
        renderer.setPixelRatio(Math.min(window.devicePixelRatio, 1.5));

        var scene = new THREE.Scene();
        var camera = new THREE.PerspectiveCamera(35, 1, 0.1, 100);
        camera.position.set(0, 0, 6.2);

        scene.add(new THREE.AmbientLight(0xffffff, 0.45));
        var key = new THREE.DirectionalLight(0xffffff, 0.85);
        key.position.set(2, 3, 5);
        scene.add(key);
        var fill = new THREE.DirectionalLight(0xffffff, 0.45);
        fill.position.set(-3, -1, -4);
        scene.add(fill);
        var rim = new THREE.DirectionalLight(0xffffff, 0.3);
        rim.position.set(0, 2, -5);
        scene.add(rim);

        var group = new THREE.Group();
        scene.add(group);

        var gba = buildGba(sourceCanvas);
        var sp = buildSp(sourceCanvas);
        group.add(gba);
        group.add(sp);

        var DEFAULTS = { gba: { x: -0.15, y: -0.5 }, sp: { x: 0.30, y: -0.45 } };
        var MIN_DIST = 2.6, MAX_DIST = 13.0;
        var FOCUS_FAR = 6.2, FOCUS_NEAR = 3.4;
        var focusPoint = new THREE.Vector3();
        var mode = 'gba';
        var targetX = DEFAULTS.gba.x, targetY = DEFAULTS.gba.y;
        var curX = targetX, curY = targetY;
        var targetDist = 6.2, curDist = targetDist;
        var panX = 0, panY = 0;
        var dragging = false, panning = false, lastPointer = null;
        var autoSpin = true, lastReset = null;
        var pointers = {}, pinchStart = 0, pinchDist = 0;

        gba.visible = true;
        sp.visible = false;

        function resetView() {
            var d = DEFAULTS[mode] || DEFAULTS.gba;
            targetX = d.x;
            targetY = d.y;
            targetDist = 6.2;
            panX = 0;
            panY = 0;
        }

        function applyMode(next) {
            if (next === mode) return;
            mode = next;
            gba.visible = mode === 'gba';
            sp.visible = mode === 'sp';
            resetView();
        }

        function pointerCount() {
            var n = 0;
            for (var k in pointers) if (pointers.hasOwnProperty(k)) n++;
            return n;
        }

        host.addEventListener('pointerdown', function (e) {
            pointers[e.pointerId] = { x: e.clientX, y: e.clientY };
            host.setPointerCapture(e.pointerId);
            if (pointerCount() === 2) {
                dragging = false;
                panning = false;
                var p = [];
                for (var k in pointers) if (pointers.hasOwnProperty(k)) p.push(pointers[k]);
                pinchStart = Math.hypot(p[0].x - p[1].x, p[0].y - p[1].y);
                pinchDist = targetDist;
                return;
            }
            panning = e.shiftKey || e.button === 1 || e.button === 2;
            dragging = !panning;
            lastPointer = { x: e.clientX, y: e.clientY };
            host.style.cursor = panning ? 'move' : 'grabbing';
        });

        host.addEventListener('pointermove', function (e) {
            if (pointers[e.pointerId]) {
                pointers[e.pointerId].x = e.clientX;
                pointers[e.pointerId].y = e.clientY;
            }
            if (pointerCount() === 2) {
                var p = [];
                for (var k in pointers) if (pointers.hasOwnProperty(k)) p.push(pointers[k]);
                var d = Math.hypot(p[0].x - p[1].x, p[0].y - p[1].y);
                if (pinchStart > 0) {
                    targetDist = Math.max(MIN_DIST, Math.min(MAX_DIST, pinchDist * (pinchStart / Math.max(d, 1))));
                }
                return;
            }
            if (!lastPointer) return;
            var dx = e.clientX - lastPointer.x;
            var dy = e.clientY - lastPointer.y;
            if (panning) {
                panX += dx * 0.004 * (curDist / 6.2);
                panY -= dy * 0.004 * (curDist / 6.2);
            } else if (dragging) {
                targetY += dx * 0.008;
                targetX += dy * 0.008;
                targetX = Math.max(-1.2, Math.min(1.2, targetX));
            }
            lastPointer = { x: e.clientX, y: e.clientY };
        });

        function endDrag(e) {
            if (e && pointers[e.pointerId]) delete pointers[e.pointerId];
            if (pointerCount() < 2) pinchStart = 0;
            if (pointerCount() === 0) {
                dragging = false;
                panning = false;
                lastPointer = null;
                host.style.cursor = 'grab';
            }
            if (e && e.pointerId !== undefined && host.hasPointerCapture(e.pointerId)) {
                host.releasePointerCapture(e.pointerId);
            }
        }
        host.addEventListener('pointerup', endDrag);
        host.addEventListener('pointercancel', endDrag);
        host.addEventListener('contextmenu', function (e) { e.preventDefault(); });
        host.addEventListener('dblclick', resetView);

        host.addEventListener('wheel', function (e) {
            e.preventDefault();
            var step = e.deltaMode === 1 ? e.deltaY * 16 : e.deltaY;
            targetDist = Math.max(MIN_DIST, Math.min(MAX_DIST, targetDist * Math.exp(step * 0.0012)));
        }, { passive: false });

        var lastWidth = 0, lastHeight = 0;
        function resize() {
            var w = host.clientWidth, h = host.clientHeight;
            if (!w || !h) return;
            if (lastWidth !== w || lastHeight !== h) {
                lastWidth = w; lastHeight = h;
                renderer.setSize(w, h, false);
                camera.aspect = w / h;
                camera.updateProjectionMatrix();
            }
        }

        function frame() {
            requestAnimationFrame(frame);

            var wanted = container.getAttribute('data-mode') || 'gba';
            if (wanted === 'flat') return;
            applyMode(wanted);

            autoSpin = container.getAttribute('data-spin') !== 'off';

            var nonce = container.getAttribute('data-reset');
            if (lastReset === null) lastReset = nonce;
            else if (nonce !== lastReset) { lastReset = nonce; resetView(); }

            var pivot = sp.userData.lidPivot;
            if (pivot) {
                var deg = parseFloat(container.getAttribute('data-lid'));
                if (!isNaN(deg)) pivot.rotation.x = -deg * Math.PI / 180;
            }

            resize();
            if (autoSpin && !dragging && !panning) targetY += 0.0035;
            curX += (targetX - curX) * 0.12;
            curY += (targetY - curY) * 0.12;
            curDist += (targetDist - curDist) * 0.15;
            group.rotation.x = curX;
            group.rotation.y = curY;
            camera.position.z = curDist;

            // Close up, the screen is what matters, so it becomes the zoom pivot.
            var blend = (FOCUS_FAR - curDist) / (FOCUS_FAR - FOCUS_NEAR);
            blend = Math.max(0, Math.min(1, blend));
            blend = blend * blend * (3 - 2 * blend);
            group.position.set(panX, panY, 0);
            var active = (mode === 'sp' ? sp : gba).userData.screen;
            if (active && blend > 0) {
                group.updateMatrixWorld(true);
                active.getWorldPosition(focusPoint);
                group.position.set(
                    panX - blend * (focusPoint.x - panX),
                    panY - blend * (focusPoint.y - panY),
                    -blend * focusPoint.z
                );
            }
            if (active) active.userData.tex.needsUpdate = true;
            renderer.render(scene, camera);
        }
        frame();
    }

    function tryInit() {
        if (!window.THREE) return false;
        var host = document.getElementById('gba-3d');
        var source = document.getElementById('gba-canvas');
        var container = document.querySelector('.gba-body');
        if (!host || !source || !container) return false;
        build(host, source, container);
        return true;
    }

    var poll = setInterval(function () {
        if (tryInit()) clearInterval(poll);
    }, 120);
})();
