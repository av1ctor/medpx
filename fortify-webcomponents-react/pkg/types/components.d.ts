/**
 * @license
 * Copyright (c) Peculiar Ventures, LLC.
 *
 * This source code is licensed under the BSD 3-Clause license found in the
 * LICENSE file in the root directory of this source tree.
 */
/// <reference types="react" />
import { JSX } from '@peculiar/fortify-webcomponents';
import type { SocketProvider, SocketCrypto } from '@webcrypto-local/client';

export declare const PeculiarFortifyCertificates: import("react").ForwardRefExoticComponent<JSX.PeculiarFortifyCertificates & Omit<import("react").HTMLAttributes<HTMLPeculiarFortifyCertificatesElement>, "style"> & import("./react-component-lib/interfaces").StyleReactProps & import("react").RefAttributes<HTMLPeculiarFortifyCertificatesElement>>;
export declare const PeculiarFortifyEnrollment: import("react").ForwardRefExoticComponent<JSX.PeculiarFortifyEnrollment & Omit<import("react").HTMLAttributes<HTMLPeculiarFortifyEnrollmentElement>, "style"> & import("./react-component-lib/interfaces").StyleReactProps & import("react").RefAttributes<HTMLPeculiarFortifyEnrollmentElement>>;

export {SocketProvider};

export interface ISelectionSuccessEvent {
    certificateId: string;
    providerId: string;
    privateKeyId: string;
    socketProvider: SocketProvider;
}

export interface PeculiarFortifyCertificatesCustomEvent<T> extends CustomEvent<T> {
    detail: T;
    target: HTMLPeculiarFortifyCertificatesElement;
}

export {SocketCrypto};