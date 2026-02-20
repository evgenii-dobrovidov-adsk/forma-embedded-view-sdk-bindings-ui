export type TextLevel = 'h1' | 'h2' | 'h3' | 'p' | 'code';

export type AlertType = 'error' | 'warning' | 'info';

export type InputType =
  | 'text'
  | 'number'
  | 'email'
  | 'password'
  | 'tel'
  | 'url'
  | 'search'
  | 'date'
  | 'time'
  | 'datetime-local'
  | 'month'
  | 'week'
  | 'color'
  | 'range'
  | 'hidden';

export type ButtonVariant = 'outlined' | 'flat' | 'solid';

export interface SelectOption {
  value: string;
  label: string;
}

export type ContainerNode =
  | { type: 'column'; gapPx: number; children: NodeDesc[] }
  | { type: 'row'; gapPx: number; children: NodeDesc[] };

export type NodeDesc =
  | { type: 'column'; gapPx: number; children: NodeDesc[] }
  | { type: 'row'; gapPx: number; children: NodeDesc[] }
  | {
      type: 'button';
      label: string;
      disabled: boolean;
      onClick?: () => void;
      variant: ButtonVariant;
    }
  | {
      type: 'input';
      inputType: InputType;
      placeholder: string;
      value: string;
      disabled: boolean;
      onChange?: (value: string) => void;
    }
  | { type: 'text'; text: string; level: TextLevel }
  | { type: 'alert'; text: string; alertType: AlertType; title?: string }
  | { type: 'image'; src: string; alt?: string }
  | {
      type: 'checkbox';
      label: string;
      checked: boolean;
      disabled: boolean;
      onChange?: (checked: boolean) => void;
    }
  | {
      type: 'select';
      options: SelectOption[];
      value: string;
      placeholder: string;
      disabled: boolean;
      onChange?: (value: string) => void;
    }
  | { type: 'separator' };
