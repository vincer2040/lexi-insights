import { getEntries } from "./api";
import { createElement, text } from "./util";

const url = window.location.origin;

const entriesEl = document.getElementById("entries");

async function main() {
    let entries = await getEntries(url);
    let i: number, len = entries.length;
    const ulEl = createElement("ul");
    for (i = 0; i < len; ++i) {
        const liEl = createElement("li");
        const entry = entries[i].Array;
        const key = entry[0].BulkString;
        const value = entry[1].BulkString;
        const keyEl = createElement("p");
        const valEl = createElement("p");

        text(keyEl, key);
        text(valEl, value);
        liEl.append(keyEl, valEl);
        ulEl.append(liEl);
    }
    entriesEl?.replaceChildren(ulEl);
}

main();
