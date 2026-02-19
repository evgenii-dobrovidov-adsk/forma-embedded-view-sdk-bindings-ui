import { col } from '../src/index';

const logEl = document.getElementById('log')!;
function log(msg: string) {
  const line = document.createElement('div');
  line.textContent = `[${new Date().toLocaleTimeString()}] ${msg}`;
  logEl.appendChild(line);
  logEl.scrollTop = logEl.scrollHeight;
}

let nameValue = '';
let colorValue = '#4a90d9';
let agreeChecked = false;
let selectedFont = 'sans';

function render() {
  col()
    .p('UI Library Demo', 'h1')
    .p('A fluent chaining interface for building UIs with Weave components.', 'p')

    .separator()

    .p('Text Input', 'h3')
    .row()
      .input('text', 'Enter your name...', nameValue, false, (v) => {
        nameValue = v;
        log(`Name changed: "${v}"`);
      })
      .button('Greet', false, 'solid', () => {
        log(`Hello, ${nameValue || 'World'}!`);
      })
    .endRow()

    .separator()

    .p('Color Picker', 'h3')
    .row()
      .p('Pick a color:', 'p')
      .input('color', '', colorValue, false, (v) => {
        colorValue = v;
        log(`Color changed: ${v}`);
        render();
      })
      .button('Reset Color', false, 'outlined', () => {
        colorValue = '#4a90d9';
        log('Color reset to default');
        render();
      })
    .endRow()

    .separator()

    .p('Select Dropdown', 'h3')
    .row()
      .p('Font family:', 'p')
      .select(
        [
          { value: 'sans', label: 'Sans-serif' },
          { value: 'serif', label: 'Serif' },
          { value: 'mono', label: 'Monospace' },
        ],
        selectedFont,
        'Choose a font...',
        false,
        (v) => {
          selectedFont = v;
          log(`Font changed: ${v}`);
        },
      )
    .endRow()

    .separator()

    .p('Checkbox & Disabled States', 'h3')
    .row()
      .checkbox('I agree to the terms', agreeChecked, false, (checked) => {
        agreeChecked = checked;
        log(`Agreement: ${checked ? 'accepted' : 'declined'}`);
        render();
      })
    .endRow()
    .row()
      .button('Submit', !agreeChecked, 'solid', () => {
        log('Form submitted!');
      })
      .button('Cancel', false, 'flat', () => {
        log('Cancelled');
      })
    .endRow()

    .separator()

    .p('Alerts', 'h3')
    .alert('Operation completed successfully.', 'info', '')
    .alert('Your session will expire in 5 minutes.', 'warning', 'Heads up')
    .alert('Failed to save changes. Please try again.', 'error', 'Error')

    .separator()

    .p('Code Block', 'h3')
    .p('col().p("Hello", "h1").button("Click", false, "solid", fn).endCol().renderInto("#app")', 'code')

    .separator()

    .p('Image', 'h3')
    .img('https://placehold.co/300x100/4a90d9/ffffff?text=@forma/ui-lib', 'Placeholder image')

    .separator()

    .p('Disabled Input', 'h3')
    .input('text', '', 'This input is disabled', true)

  .endCol()
  .renderInto('#app');
}

render();
