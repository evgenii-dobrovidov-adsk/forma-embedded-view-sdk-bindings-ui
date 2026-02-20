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

export function col(gapPx = 8): UIBuilder {
  return new UIBuilder().col(gapPx);
}

export function row(gapPx = 8): UIBuilder {
  return new UIBuilder().row(gapPx);
}
