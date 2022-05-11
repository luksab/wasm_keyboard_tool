/*
* ┌───┬───┬───┬───┐       ┌───┬───┬───┬───┐
* │000│001│002│003│       │023│022│021│020│
* └───┴───┴───┴───┘       └───┴───┴───┴───┘
*
*          ┌───┐             ┌───┐   
*          │010├───┐     ┌───┤030│   
*          ├───┤012│     │032├───┤   
*          │011├───┘     └───┤031│   
*          └───┘             └───┘   
format: `([L/R][Pinkie, Ring, Middle, Index, thumbL, thumbU, thumbD]+ [key]\n)+`
*/

import init, {
    Config,
    Finger,
} from "./keyboard-converter/pkg/keyboard_converter.js";
init().then(async () => {
    // greet("WebAssembly");
    let config = await (await fetch("asentiop.cfg")).text();
    // console.log(await config.text());
    window.config = Config.from_str(config);
    console.log(window.config.check());
    console.log(window.config.to_keychordz());
});

const fingersPressed = {
    LP: false,
    LR: false,
    LM: false,
    LI: false,
    LU: false,
    LD: false,
    LL: false,
    RP: false,
    RR: false,
    RM: false,
    RI: false,
    RU: false,
    RD: false,
    RL: false,
};

const num_to_finger = [
    "LP",
    "LR",
    "LM",
    "LI",
    "RI",
    "RM",
    "RR",
    "RP",
    "LU",
    "LD",
    "LL",
    "RU",
    "RD",
    "RL",
];

document.addEventListener("DOMContentLoaded", function () {
    let $ = document.getElementById.bind(document);
    const keyMap = {
        LP: $("key-000"),
        LR: $("key-001"),
        LM: $("key-002"),
        LI: $("key-003"),
        LU: $("key-010"),
        LD: $("key-011"),
        LL: $("key-012"),
        RP: $("key-020"),
        RR: $("key-021"),
        RM: $("key-022"),
        RI: $("key-023"),
        RU: $("key-030"),
        RD: $("key-031"),
        RL: $("key-032"),
    };

    for (const key in keyMap) {
        keyMap[key].addEventListener("click", () => {
            fingersPressed[key] = !fingersPressed[key];
            keyMap[key].classList.toggle("active");
            console.log(key);
            let fingers = [];
            for (const k in fingersPressed) {
                if (fingersPressed[k]) {
                    fingers.push(k);
                }
            }
            const keys = window.config.get_key_with_fingers(fingers);
            console.log(keys);
            for (const [index, element] of keys.entries()) {
                console.log(index, element);
                keyMap[num_to_finger[index]].innerText = keys[index];
            }
        });
    }
});
