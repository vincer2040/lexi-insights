
export function createElement(elType: string): HTMLElement {
    return document.createElement(elType);
}

export function text(element: HTMLElement, text: string) {
    element.innerText = text;
}

