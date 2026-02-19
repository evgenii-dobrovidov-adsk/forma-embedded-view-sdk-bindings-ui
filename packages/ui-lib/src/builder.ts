import type {
  NodeDesc,
  ContainerNode,
  TextLevel,
  InputType,
  AlertType,
  ButtonVariant,
  SelectOption,
} from './types';
import { renderTree } from './render';

export class UIBuilder {
  private rootNodes: NodeDesc[] = [];
  private stack: ContainerNode[] = [];

  private current(): NodeDesc[] {
    if (this.stack.length > 0) {
      return this.stack[this.stack.length - 1].children;
    }
    return this.rootNodes;
  }

  col(): this {
    const node: ContainerNode = { type: 'column', children: [] };
    this.current().push(node);
    this.stack.push(node);
    return this;
  }

  endCol(): this {
    if (this.stack.length === 0) {
      throw new Error('endCol() called without matching col()');
    }
    const top = this.stack[this.stack.length - 1];
    if (top.type !== 'column') {
      throw new Error(`endCol() called but current container is '${top.type}'`);
    }
    this.stack.pop();
    return this;
  }

  row(): this {
    const node: ContainerNode = { type: 'row', children: [] };
    this.current().push(node);
    this.stack.push(node);
    return this;
  }

  endRow(): this {
    if (this.stack.length === 0) {
      throw new Error('endRow() called without matching row()');
    }
    const top = this.stack[this.stack.length - 1];
    if (top.type !== 'row') {
      throw new Error(`endRow() called but current container is '${top.type}'`);
    }
    this.stack.pop();
    return this;
  }

  button(
    label: string,
    disabled: boolean,
    variant: ButtonVariant,
    onClick?: () => void,
  ): this {
    this.current().push({
      type: 'button',
      label,
      disabled,
      onClick,
      variant,
    });
    return this;
  }

  input(
    inputType: InputType,
    placeholder: string,
    value: string,
    disabled: boolean,
    onChange?: (value: string) => void,
  ): this {
    this.current().push({
      type: 'input',
      inputType,
      placeholder,
      value,
      disabled,
      onChange,
    });
    return this;
  }

  p(text: string, level: TextLevel): this {
    this.current().push({ type: 'text', text, level });
    return this;
  }

  alert(
    text: string,
    alertType: AlertType,
    title: string,
  ): this {
    this.current().push({
      type: 'alert',
      text,
      alertType,
      title: title || undefined,
    });
    return this;
  }

  img(src: string, alt: string): this {
    this.current().push({
      type: 'image',
      src,
      alt: alt || undefined,
    });
    return this;
  }

  checkbox(
    label: string,
    checked: boolean,
    disabled: boolean,
    onChange?: (checked: boolean) => void,
  ): this {
    this.current().push({
      type: 'checkbox',
      label,
      checked,
      disabled,
      onChange,
    });
    return this;
  }

  select(
    options: SelectOption[],
    value: string,
    placeholder: string,
    disabled: boolean,
    onChange?: (value: string) => void,
  ): this {
    this.current().push({
      type: 'select',
      options,
      value,
      placeholder,
      disabled,
      onChange,
    });
    return this;
  }

  separator(): this {
    this.current().push({ type: 'separator' });
    return this;
  }

  renderInto(selector: string): void {
    while (this.stack.length > 0) {
      this.stack.pop();
    }
    renderTree(selector, this.rootNodes);
  }
}
