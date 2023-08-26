import { createSafeContext } from '@mantine/utils';

interface ArrayInputContextValue {
    onCreate: () => void;
    onDelete: (index: number) => void;
}

export const [ArrayInputProvider, useArrayInputContext] = createSafeContext<ArrayInputContextValue>(
  'ArrayInput component was not found in tree'
);