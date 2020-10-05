import { memory } from "asciidf/asciidf_bg";
import { do_thing, Pixels, Fetcher } from 'asciidf';

const W = 200;
const H = 40;
const pixels = Pixels.new(W, H);
const fetcher = Fetcher.new();
const canvas = document.getElementById('world');
const ctx = canvas.getContext('2d');

// const pre = document.createElement('pre');
// pre.style.lineHeight = 0.9;
// document.body.appendChild(pre);

function mainLoop() {
    pixels.hacky_update();
    pixels.write_to_buffer(fetcher);

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.font = "8px monospace";
    ctx.fillStyle = "black";

    const ptr = fetcher.fetch();
    const data = new Uint32Array(memory.buffer, ptr, W * H * 2);
    for(var y=0; y<H; y++) {
        var s = "";
        for(var x=0; x<W; x++) {
            const idx = y*W + x;
            const ch = String.fromCharCode(data[idx*2]);
            s += ch;
        }

        ctx.fillText(s, 0, (y/H) * 600);
    }

    // pre.innerHTML = s;


    requestAnimationFrame(mainLoop);
}
requestAnimationFrame(mainLoop);