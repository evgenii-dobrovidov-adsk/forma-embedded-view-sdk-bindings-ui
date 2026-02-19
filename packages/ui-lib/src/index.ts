export { UIBuilder } from './builder';
export type {
  NodeDesc,
  ContainerNode,
  TextLevel,
  InputType,
  AlertType,
  ButtonVariant,
  SelectOption,
} from './types';

import { UIBuilder } from './builder';

export function col(): UIBuilder {
  return new UIBuilder().col();
}

export function row(): UIBuilder {
  return new UIBuilder().row();
}
