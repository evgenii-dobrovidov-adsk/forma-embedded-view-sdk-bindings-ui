const WEAVE_BASE = 'https://app.autodeskforma.eu/design-system/v2';

const loadedScripts = new Set<string>();
let baseLoaded = false;

export function ensureBase(): void {
  if (baseLoaded) return;
  baseLoaded = true;

  const link = document.createElement('link');
  link.rel = 'stylesheet';
  link.href = `${WEAVE_BASE}/forma/styles/base.css`;
  document.head.appendChild(link);
}

export function ensureComponent(componentPath: string): void {
  ensureBase();

  if (loadedScripts.has(componentPath)) return;
  loadedScripts.add(componentPath);

  const script = document.createElement('script');
  script.type = 'module';
  script.src = `${WEAVE_BASE}/weave/components/${componentPath}`;
  document.head.appendChild(script);
}

export const WEAVE_COMPONENTS = {
  button: 'button/weave-button.js',
  input: 'input/weave-input.js',
  banner: 'banner/weave-banner.js',
  checkbox: 'checkbox/weave-checkbox.js',
  select: 'dropdown/weave-select.js',
  toggle: 'toggle/weave-toggle.js',
  slider: 'slider/weave-slider.js',
} as const;
