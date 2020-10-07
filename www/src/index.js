import { memory } from "asciidf/asciidf_bg";
import { do_thing, Pixels, Fetcher } from 'asciidf';

const W = 200;
const H = 40;
const pixels = Pixels.new(W, H);

const pre = document.getElementById("out");
let isPaused = true;

function step() {
    pixels.update_with_example('box_minus_sphere');
    pre.innerHTML = pixels.html();

    if(!isPaused) {
        requestAnimationFrame(step);
    }
}
step();


const stepBtn = document.getElementById('stepButton');
stepBtn.addEventListener('click', function() {
    step();
});

const toggleButton = document.getElementById('toggleButton');
toggleButton.addEventListener('click', function() {
    isPaused = !isPaused;
    toggleButton.innerText = isPaused ? "Play" : "Pause";
    step();
});
