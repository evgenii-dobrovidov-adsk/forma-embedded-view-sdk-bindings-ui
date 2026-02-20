import type { NodeDesc } from './types';
import { ensureBase, ensureComponent, WEAVE_COMPONENTS } from './loader';

function renderNode(node: NodeDesc): HTMLElement {
  switch (node.type) {
    case 'column':
      return renderColumn(node);
    case 'row':
      return renderRow(node);
    case 'button':
      return renderButton(node);
    case 'input':
      return renderInput(node);
    case 'text':
      return renderText(node);
    case 'alert':
      return renderAlert(node);
    case 'image':
      return renderImage(node);
    case 'checkbox':
      return renderCheckbox(node);
    case 'select':
      return renderSelect(node);
    case 'separator':
      return renderSeparator();
  }
}

function renderColumn(node: Extract<NodeDesc, { type: 'column' }>): HTMLElement {
  const el = document.createElement('div');
  el.style.display = 'flex';
  el.style.flexDirection = 'column';
  el.style.alignItems = 'stretch';
  el.style.justifyContent = 'flex-start';

  for (const child of node.children) {
    el.appendChild(renderNode(child));
  }
  return el;
}

function renderRow(node: Extract<NodeDesc, { type: 'row' }>): HTMLElement {
  const el = document.createElement('div');
  el.style.display = 'flex';
  el.style.flexDirection = 'row';
  el.style.alignItems = 'center';
  el.style.justifyContent = 'center';
  el.style.gap = '8px';

  for (const child of node.children) {
    el.appendChild(renderNode(child));
  }
  return el;
}

function renderButton(node: Extract<NodeDesc, { type: 'button' }>): HTMLElement {
  ensureComponent(WEAVE_COMPONENTS.button);

  const el = document.createElement('weave-button') as HTMLElement;
  el.setAttribute('variant', node.variant);
  if (node.disabled) {
    el.setAttribute('disabled', '');
  }
  el.textContent = node.label;

  if (node.onClick) {
    el.addEventListener('click', node.onClick);
  }
  return el;
}

const WEAVE_INPUT_TYPES = new Set(['text', 'number', 'email']);

function renderInput(node: Extract<NodeDesc, { type: 'input' }>): HTMLElement {
  if (WEAVE_INPUT_TYPES.has(node.inputType)) {
    return renderWeaveInput(node);
  }
  return renderNativeInput(node);
}

function renderWeaveInput(node: Extract<NodeDesc, { type: 'input' }>): HTMLElement {
  ensureComponent(WEAVE_COMPONENTS.input);

  const el = document.createElement('weave-input') as HTMLElement;
  el.setAttribute('type', node.inputType);
  el.setAttribute('placeholder', node.placeholder);
  el.setAttribute('value', node.value);
  if (node.disabled) {
    el.setAttribute('disabled', '');
  }

  if (node.onChange) {
    const handler = node.onChange;
    el.addEventListener('change', ((e: CustomEvent) => {
      handler(e.detail?.value ?? (e.target as HTMLInputElement).value);
    }) as EventListener);
  }
  return el;
}

function renderNativeInput(node: Extract<NodeDesc, { type: 'input' }>): HTMLElement {
  ensureBase();

  const el = document.createElement('input');
  el.type = node.inputType;
  el.placeholder = node.placeholder;
  el.value = node.value;
  el.disabled = node.disabled;

  if (node.onChange) {
    const handler = node.onChange;
    el.addEventListener('input', () => {
      handler(el.value);
    });
  }
  return el;
}

function renderText(node: Extract<NodeDesc, { type: 'text' }>): HTMLElement {
  ensureBase();

  const tagMap: Record<string, string> = {
    h1: 'h1',
    h2: 'h2',
    h3: 'h3',
    p: 'p',
    code: 'code',
  };

  if (node.level === 'code') {
    const pre = document.createElement('pre');
    const code = document.createElement('code');
    code.textContent = node.text;
    pre.appendChild(code);
    return pre;
  }

  const tag = tagMap[node.level] ?? 'p';
  const el = document.createElement(tag);
  el.textContent = node.text;
  return el;
}

function renderAlert(node: Extract<NodeDesc, { type: 'alert' }>): HTMLElement {
  ensureComponent(WEAVE_COMPONENTS.banner);

  const el = document.createElement('weave-banner') as HTMLElement;
  el.setAttribute('variant', node.alertType);

  if (node.title) {
    const titleEl = document.createElement('span');
    titleEl.setAttribute('slot', 'title');
    titleEl.textContent = node.title;
    el.appendChild(titleEl);
  }

  const textNode = document.createTextNode(node.text);
  el.appendChild(textNode);
  return el;
}

function renderImage(node: Extract<NodeDesc, { type: 'image' }>): HTMLElement {
  const el = document.createElement('img');
  el.src = node.src;
  if (node.alt) {
    el.alt = node.alt;
  }
  el.style.maxWidth = '100%';
  return el;
}

function renderCheckbox(node: Extract<NodeDesc, { type: 'checkbox' }>): HTMLElement {
  ensureComponent(WEAVE_COMPONENTS.checkbox);

  const el = document.createElement('weave-checkbox') as HTMLElement;
  if (node.checked) {
    el.setAttribute('checked', '');
  }
  if (node.disabled) {
    el.setAttribute('disabled', '');
  }
  el.setAttribute('showlabel', '');
  el.setAttribute('label', node.label);

  if (node.onChange) {
    const handler = node.onChange;
    el.addEventListener('change', ((e: CustomEvent) => {
      handler(e.detail?.checked ?? false);
    }) as EventListener);
  }
  return el;
}

function renderSelect(node: Extract<NodeDesc, { type: 'select' }>): HTMLElement {
  ensureComponent(WEAVE_COMPONENTS.select);

  const el = document.createElement('weave-select') as HTMLElement;
  el.setAttribute('value', node.value);
  if (node.placeholder) {
    el.setAttribute('placeholder', node.placeholder);
  }
  if (node.disabled) {
    el.setAttribute('disabled', '');
  }

  for (const opt of node.options) {
    const optEl = document.createElement('weave-select-option') as HTMLElement;
    optEl.setAttribute('value', opt.value);
    optEl.textContent = opt.label;
    el.appendChild(optEl);
  }

  if (node.onChange) {
    const handler = node.onChange;
    el.addEventListener('change', ((e: CustomEvent) => {
      handler(e.detail?.value ?? '');
    }) as EventListener);
  }
  return el;
}

function renderSeparator(): HTMLElement {
  const el = document.createElement('hr');
  el.style.width = '100%';
  el.style.border = 'none';
  el.style.borderTop = '1px solid #e0e0e0';
  el.style.margin = '8px 0';
  return el;
}

export function renderTree(selector: string, nodes: NodeDesc[]): void {
  const target = document.querySelector(selector);
  if (!target) {
    throw new Error(`renderInto(): element not found for selector '${selector}'`);
  }

  target.innerHTML = '';
  for (const node of nodes) {
    target.appendChild(renderNode(node));
  }
}
