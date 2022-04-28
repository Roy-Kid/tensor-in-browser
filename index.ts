import init, {zeros} from "./pkg/tensor_in_browser.js";

await init();

let a = new Uint32Array([2, 2]);
let b = zeros(a);

