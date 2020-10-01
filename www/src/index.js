import { do_thing } from 'asciidf';


const s = do_thing();

const pre = document.createElement('pre');
pre.style.lineHeight = 0.9;
pre.innerHTML = s;
document.body.appendChild(pre);