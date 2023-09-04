/**
 * @license
 * Copyright (c) Peculiar Ventures, LLC.
 *
 * This source code is licensed under the BSD 3-Clause license found in the
 * LICENSE file in the root directory of this source tree.
 */
import { defineCustomElements } from '@peculiar/fortify-webcomponents/loader/index.es2017';
export * from './components';
if (typeof window !== 'undefined') {
    defineCustomElements(window);
}

