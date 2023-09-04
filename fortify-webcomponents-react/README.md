# `@peculiar/fortify-webcomponents-react`

These are React specific building blocks on top of [@peculiar/fortify-webcomponents](../webcomponents) components.

## Installation

#### npm

```
npm install @peculiar/fortify-webcomponents-react
```

## Usage

Then you can use a components anywhere in your JSX.

```tsx
import React from 'react';
import ReactDOM from 'react-dom';
import { PeculiarFortifyCertificates } from '@peculiar/fortify-webcomponents-react';

ReactDOM.render(
  <PeculiarFortifyCertificates />,
  document.querySelector('#root'),
);
```

And add styles file to your HTML.

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@peculiar/fortify-webcomponents/dist/peculiar/peculiar.css">
```

## Examples

Are you looking for an example project to get started? [We host some](https://fortifyapp.com/examples/certificate-management).

## Documentation

Check out our [documentation website](https://fortifyapp.com/docs/overview).